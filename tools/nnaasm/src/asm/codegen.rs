use std::{collections::HashMap, fmt::Display, ops::Range, rc::Rc};

use libnna::u4;

use super::{
    lex::{OpToken, RefType, Token, ValueToken8},
    IntoAsmError, Located, Location,
};

pub enum CodeGenError {
    NoOrg(),
    OrgOverlap(Org, Org),
    LabelNotDefined(Box<str>),
    MaxDistAssertionFailed(u8),
}
impl IntoAsmError for Located<CodeGenError> {
    fn into_asm_error<'a>(self, code: &'a str, filename: Rc<str>) -> super::AsmError<'a> {
        let message = match self.value {
            CodeGenError::NoOrg() => {
                "Everything needs to be defined inside an .org statement. Else the assembler can't know where to put it in the final output binary".to_string()
            }
            CodeGenError::LabelNotDefined(name) => format!("label '{}' is not defined", name),
            CodeGenError::OrgOverlap(org0, org1) => {
                format!("This org ({}) overlaps with: {}", org0, org1)
            },
            CodeGenError::MaxDistAssertionFailed(size) => {
                format!("Assertion failed. distance was {:#04x}", size)
            }
        };
        super::AsmError {
            filename,
            code,
            message,
            location: self.location,
        }
    }
}

pub fn calc_mem_usage(data: &[u8; 256]) -> Range<usize> {
    let mut zero_bytes = 0;
    for byte in &data[..240] {
        if *byte != 0 {
            zero_bytes = 0;
            continue;
        }
        zero_bytes += 1;
    }
    (240 - zero_bytes)..240
}

// fn emit_hex(data: [u4; 256]) -> Vec<u8> {
//     let mut output = String::new();
//
//     for nib in data.iter() {
//         //println!("{} {:#x}", util::to_hex4(*nib), nib.into_u8());
//         output.push_str(&util::to_hex4(*nib));
//         output.push('\n');
//     }
//
//     output.as_bytes().to_vec()
// }

#[derive(Clone, Copy)]
pub struct Org {
    start_addr: u8,
    size: u8,
}
impl Display for Org {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            ".org {:#04x} ; size: {:#04x}",
            self.start_addr, self.size
        ))
    }
}
impl Org {
    pub fn end_addr(self) -> u8 {
        self.start_addr + self.size
    }

    pub fn overlap(self, other: Org) -> bool {
        if other.start_addr < self.start_addr && other.end_addr() < self.start_addr {
            false
        } else if other.start_addr > self.end_addr() {
            false
        } else {
            true
        }
    }

    #[inline]
    pub fn write(
        start_addr: u8,
        location: Location,
        data: &[u8],
        bin: &mut [u8; 256],
        orgs: &[Org],
    ) -> Result<Org, Located<CodeGenError>> {
        let size = data.len() as u8;
        let org = Org { start_addr, size };
        for other_org in orgs {
            if other_org.overlap(org) {
                return Err(Located::new(
                    CodeGenError::OrgOverlap(org, *other_org),
                    location,
                ));
            }
        }

        for (i, nib) in data.iter().enumerate() {
            bin[org.start_addr as usize + i] = *nib;
        }
        Ok(org)
    }
}

#[inline]
fn resolve_label(
    labels: &HashMap<Box<str>, u8>,
    label: Located<(Box<str>, RefType)>,
) -> Result<u8, Located<CodeGenError>> {
    let resolved_label_addr = labels.get(&label.value.0).ok_or_else(|| {
        Located::new(CodeGenError::LabelNotDefined(label.value.0), label.location)
    })?;

    Ok(label.value.1.mask_low(*resolved_label_addr))
}

fn resolve_labels(
    output: &mut [u8; 256],
    labels: HashMap<Box<str>, u8>,
    mut label_refs: Vec<Located<(Box<str>, u8, RefType)>>,
) -> Result<(), Located<CodeGenError>> {
    for lref in label_refs.drain(..) {
        let addr = lref.value.1;

        // Or with the output because it could be a ref on the end of an instruction.
        // cases:
        // output[addr] = 0x00 => or is fine because or(0,0) = 0 and or(0,1)=1
        // output[addr] = 0x30 => or is fine because bits are masked
        output[addr as usize] |=
            resolve_label(&labels, lref.map(|(name, _, ref_type)| (name, ref_type)))?;
    }

    Ok(())
}

pub fn gen(tt: Vec<Located<Token>>) -> Result<[u8; 256], Located<CodeGenError>> {
    let mut bin = [0; 256];
    let mut label_refs = Vec::new();
    let mut assert_max_dists = Vec::new();
    let mut orgs = Vec::new();
    let mut labels = HashMap::new();

    let mut tt_iter = tt.into_iter();
    let (mut cur_org_addr, mut cur_org_loc) = match tt_iter.next() {
        Some(token) => match token.value {
            Token::Org(addr) => (addr, token.location),
            _ => {
                return Err(Located::new(CodeGenError::NoOrg(), token.location));
            }
        },
        None => return Ok(bin),
    };

    let mut data = Vec::new();

    for token in tt_iter {
        match token.value {
            Token::Op(OpToken::Full(byte)) => {
                data.push(byte);
            }
            Token::Op(OpToken::LabelRef(instruct, label_ref, ref_type)) => {
                label_refs.push(Located::new(
                    (label_ref, cur_org_addr + data.len() as u8, ref_type),
                    token.location,
                ));
                data.push(instruct.into_high());
            }
            Token::Value(ValueToken8::Const(value)) => data.push(value),
            Token::Value(ValueToken8::LabelRef(name, ref_type)) => {
                label_refs.push(Located::new(
                    (name, cur_org_addr + data.len() as u8, ref_type),
                    token.location,
                ));
                data.push(0)
            }
            Token::LabelDef(name) => {
                labels.insert(name, cur_org_addr + data.len() as u8);
            }
            Token::Org(addr) => {
                let org = Org::write(cur_org_addr, cur_org_loc, &data, &mut bin, &orgs)?;
                orgs.push(org);
                data.clear();
                cur_org_loc = token.location;
                cur_org_addr = addr;
            }
            Token::AssertMaxDist(start, dist) => {
                let end = Located::new(
                    cur_org_addr.saturating_add(data.len() as u8),
                    token.location,
                );
                match start {
                    ValueToken8::Const(start) => assert_max_dist(end, start, dist)?,
                    ValueToken8::LabelRef(label, ref_type) => {
                        assert_max_dists.push((end, label, ref_type, dist));
                    }
                }
            }
            Token::Bytes(bytes) => {
                data.extend_from_slice(&bytes);
            }
            Token::IncludeBytes(_) => {
                panic!("codegen can't work with include_bytes tokens");
            }
        }
    }
    //write last org
    Org::write(cur_org_addr, cur_org_loc, &data, &mut bin, &orgs)?;

    for (end, label, ref_type, max_dist) in assert_max_dists.drain(..) {
        let resolved = resolve_label(
            &labels,
            Located::new((label, ref_type), end.location.clone()),
        )?;
        assert_max_dist(end, resolved, max_dist)?;
    }

    resolve_labels(&mut bin, labels, label_refs)?;

    Ok(bin)
}

fn assert_max_dist(end: Located<u8>, start: u8, max_dist: u8) -> Result<(), Located<CodeGenError>> {
    let dist = if start > end.value {
        start.saturating_sub(end.value)
    } else {
        end.saturating_sub(start)
    };
    //println!("dist: {}", dist);
    if dist > max_dist {
        Err(Located::new(
            CodeGenError::MaxDistAssertionFailed(dist),
            end.location,
        ))
    } else {
        Ok(())
    }
}
