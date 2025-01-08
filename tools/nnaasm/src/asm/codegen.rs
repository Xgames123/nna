use std::{collections::HashMap, fmt::Display, ops::Range, rc::Rc};

use libnna::u4;

use super::{
    parselex::{OpToken, RefType, Token, ValueToken8},
    IntoAsmError, Located, Location,
};

pub enum CodeGenError {
    NoOrg(),
    OrgOverlap(Org, Org),
    LabelNotDefined(Box<str>),
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

fn resolve_labels(
    output: &mut [u8; 256],
    labels: HashMap<Box<str>, u8>,
    label_refs: Vec<Located<(Box<str>, u8, RefType)>>,
) -> Result<(), Located<CodeGenError>> {
    for lref in label_refs {
        let (name, addr, ref_type) = lref.value;
        let pointed_addr = labels
            .get(&name)
            .ok_or_else(|| Located::new(CodeGenError::LabelNotDefined(name), lref.location))?;

        let masked = match ref_type {
            RefType::Low => *pointed_addr & 0x0F,
            RefType::High => *pointed_addr & 0xF0 >> 4,
            RefType::Full => *pointed_addr,
        };

        // Or this with the output because it could be a ref on the end of an instruction
        output[addr as usize] |= masked;
    }

    Ok(())
}

pub fn gen(tt: Vec<Located<Token>>) -> Result<[u8; 256], Located<CodeGenError>> {
    let mut bin = [0; 256];
    let mut label_refs = Vec::new();
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
        }
    }
    //write last org
    Org::write(cur_org_addr, cur_org_loc, &data, &mut bin, &orgs)?;

    resolve_labels(&mut bin, labels, label_refs)?;

    Ok(bin)
}
pub fn pack(data: [u4; 256]) -> [u8; 128] {
    let mut output = [0u8; 128];
    for (i, nibpair) in data.chunks(2).into_iter().enumerate() {
        let high = nibpair[0].into_high(); // high is <<
        let low = nibpair[1].into_low(); // low is >>

        output[i] = low | high;
    }
    output
}
// pub fn emit(format: Format, file_ext: Option<&str>, code: [u4; 256]) -> Vec<u8> {
//     match format {
//         Format::Hex => emit_hex(code),
//         Format::Bin => emit_bin_packed(code),
//         Format::Auto => emit(
//             file_ext
//                 .map(|ext| match ext {
//                     "hex" => Some(Format::Hex),
//                     "bin" => Some(Format::Bin),
//                     _ => None,
//                 })
//                 .flatten()
//                 .unwrap_or(Format::Bin),
//             file_ext,
//             code,
//         ),
//     }
// }
