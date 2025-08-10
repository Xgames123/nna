use std::{collections::HashMap, fmt::Display, ops::Range, rc::Rc};

use super::{
    lex::{OpToken, RefType, Token, ValueToken8},
    IntoAsmError, Located, Location,
};
pub type Bank = [u8; 256];

pub enum CodeGenError {
    NoOrg(),
    OrgOverlap(Org, Org),
    LabelNotDefined(Box<str>),
    ReachableAssertionFailed,
}
impl IntoAsmError for Located<CodeGenError> {
    fn into_asm_error<'a>(self, code: &'a str, filename: Rc<str>) -> super::AsmError<'a> {
        let message = match self.value {
            CodeGenError::NoOrg() => {
                "Everything needs to be defined inside an .org statement. Otherwise the assembler can't know where to put it in the final output binary".to_string()
            }
            CodeGenError::LabelNotDefined(name) => format!("label '{}' is not defined", name),
            CodeGenError::OrgOverlap(org0, org1) => {
                format!("This org ({}) overlaps with: {}", org0, org1)
            },
            CodeGenError::ReachableAssertionFailed => {
                format!("Address is not reachable from here.")
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

pub fn calc_mem_usage(data: &Vec<Bank>, mem_size: u16) -> Vec<Range<usize>> {
    let mut banks = Vec::new();
    let mut lefover_size = mem_size as usize;

    for bank in data {
        let bank_size = if lefover_size < size_of::<Bank>() {
            let bank_size = size_of::<Bank>() - lefover_size;
            lefover_size = 0;
            bank_size
        } else {
            lefover_size -= size_of::<Bank>();
            size_of::<Bank>()
        };

        banks.push(calc_bank_usage(bank, bank_size));
    }
    banks
}

fn calc_bank_usage(bank: &Bank, usable_size: usize) -> Range<usize> {
    let mut zero_bytes = 0;
    for byte in &bank[..usable_size] {
        if *byte != 0 {
            zero_bytes = 0;
            continue;
        }
        zero_bytes += 1;
    }
    (usable_size - zero_bytes)..usable_size
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

pub struct OrgBuilder {
    loc: Location,
    start_addr: u8,
    bank: u8,
    data: Vec<u8>,
}
impl OrgBuilder {
    pub fn new(loc: Location, bank: u8, start_addr: u8) -> Self {
        Self {
            loc,
            bank,
            start_addr,
            data: Vec::new(),
        }
    }

    #[inline]
    pub fn write(&mut self, bin: &mut Bank, orgs: &[Org]) -> Result<Org, Located<CodeGenError>> {
        let size = self.data.len() as u8;
        let org = Org {
            start_addr: self.start_addr,
            bank: self.bank,
            size,
        };
        for other_org in orgs {
            if other_org.bank != self.bank {
                continue;
            }
            if other_org.overlap(org) {
                return Err(Located::new(
                    CodeGenError::OrgOverlap(org, *other_org),
                    self.loc.clone(),
                ));
            }
        }

        for (i, nib) in self.data.iter().enumerate() {
            bin[org.start_addr as usize + i] = *nib;
        }
        Ok(org)
    }
}

#[derive(Clone, Copy)]
pub struct Org {
    start_addr: u8,
    bank: u8,
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

struct LabelRef {
    ty: RefType,
    label: Box<str>,
    addr: u8,
    bank: u8,
}

fn alloc_bank(mem: &mut Vec<Bank>, bank: u8) -> &mut Bank {
    mem.extend((mem.len()..=bank as usize).map(|_| [0; 256]));
    //SAFETY: just extended above
    unsafe { mem.get_unchecked_mut(bank as usize) }
}

fn resolve_labels(
    mem: &mut Vec<Bank>,
    labels: HashMap<Box<str>, u8>,
    mut label_refs: Vec<Located<LabelRef>>,
) -> Result<(), Located<CodeGenError>> {
    for lref in label_refs.drain(..) {
        let bank = lref.value.bank;
        let bank = alloc_bank(mem, bank);
        let addr = lref.value.addr;

        // |= with the output because it could be a ref on the end of an instruction.
        // cases:
        // output[addr] = 0x00 => or is fine because or(0,0) = 0 and or(0,1)=1
        // output[addr] = 0x30 => or is fine because bits are masked
        bank[addr as usize] |= resolve_label(&labels, lref.map(|lref| (lref.label, lref.ty)))?;
    }

    Ok(())
}

pub fn gen(tt: Vec<Located<Token>>) -> Result<Vec<Bank>, Located<CodeGenError>> {
    let mut mem = Vec::new();
    let mut label_refs = Vec::new();
    let mut reachable_checks = Vec::new();
    let mut labels = HashMap::new();

    let mut cur_bank_num = 0;
    let mut cur_bank = alloc_bank(&mut mem, 0);
    let mut orgs = Vec::new();
    let mut cur_org: Option<OrgBuilder> = None;

    fn org(
        org: &mut Option<OrgBuilder>,
        loc: Location,
    ) -> Result<&mut OrgBuilder, Located<CodeGenError>> {
        match org {
            Some(org) => return Ok(org),
            None => return Err(Located::new(CodeGenError::NoOrg(), loc)),
        }
    }

    for token in tt.into_iter() {
        match token.value {
            Token::Op(OpToken::Full(byte)) => {
                let org = org(&mut cur_org, token.location.clone())?;
                org.data.push(byte);
            }
            Token::Op(OpToken::LabelRef(instruct, label, ref_type)) => {
                let org = org(&mut cur_org, token.location.clone())?;
                label_refs.push(Located::new(
                    LabelRef {
                        label,
                        bank: cur_bank_num,
                        addr: org.start_addr + org.data.len() as u8,
                        ty: ref_type,
                    },
                    token.location,
                ));
                org.data.push(instruct);
            }
            Token::Value(ValueToken8::Const(value)) => {
                let org = org(&mut cur_org, token.location)?;
                org.data.push(value);
            }
            Token::Value(ValueToken8::LabelRef(label, ref_type)) => {
                let org = org(&mut cur_org, token.location.clone())?;
                label_refs.push(Located::new(
                    LabelRef {
                        label,
                        bank: cur_bank_num,
                        addr: org.start_addr + org.data.len() as u8,
                        ty: ref_type,
                    },
                    token.location,
                ));
                org.data.push(0)
            }
            Token::LabelDef(name) => {
                let org = org(&mut cur_org, token.location.clone())?;
                labels.insert(name, org.start_addr + org.data.len() as u8);
            }
            Token::Org(addr) => {
                if let Some(org) = &mut cur_org {
                    orgs.push(org.write(&mut cur_bank, &orgs)?);
                }
                cur_org = Some(OrgBuilder::new(token.location, cur_bank_num, addr));
            }
            Token::Bank(addr) => {
                if let Some(org) = &mut cur_org {
                    orgs.push(org.write(&mut cur_bank, &orgs)?);
                }
                cur_org = None;
                cur_bank_num = addr;
                cur_bank = alloc_bank(&mut mem, addr);
            }
            Token::Reachable(start) => {
                let org = org(&mut cur_org, token.location.clone())?;
                let end = Located::new(
                    org.start_addr.saturating_add(org.data.len() as u8),
                    token.location,
                );
                match start {
                    ValueToken8::Const(start) => check_reachable(end, start)?,
                    ValueToken8::LabelRef(label, ref_type) => {
                        reachable_checks.push((end, label, ref_type));
                    }
                }
            }
            Token::Bytes(bytes) => {
                let org = org(&mut cur_org, token.location)?;
                org.data.extend_from_slice(&bytes);
            }
            Token::IncludeBytes(_) => {
                panic!("codegen can't work with include_bytes tokens");
            }
        }
    }
    //write last org
    if let Some(org) = &mut cur_org {
        org.write(&mut cur_bank, &orgs)?;
    }

    for (end, label, ref_type) in reachable_checks.drain(..) {
        let resolved = resolve_label(
            &labels,
            Located::new((label, ref_type), end.location.clone()),
        )?;
        check_reachable(end, resolved)?;
    }

    resolve_labels(&mut mem, labels, label_refs)?;

    Ok(mem)
}

fn check_reachable(end: Located<u8>, start: u8) -> Result<(), Located<CodeGenError>> {
    //sat_sub here because we want this compiler directive to apply to the previous instruction instead
    //of the next
    if end.value.saturating_sub(1) & 0xF0 != start & 0xF0 {
        Err(Located::new(
            CodeGenError::ReachableAssertionFailed,
            end.location,
        ))
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn alloc_bank() {
        let mut mem = Vec::new();
        assert_eq!(super::alloc_bank(&mut mem, 0), &[0; 256]);
        assert_eq!(mem, vec![[0; 256]]);

        assert_eq!(super::alloc_bank(&mut mem, 3), &[0; 256]);
        assert_eq!(mem, vec![[0; 256], [0; 256], [0; 256], [0; 256]]);

        assert_eq!(super::alloc_bank(&mut mem, 3), &[0; 256]);
        assert_eq!(mem, vec![[0; 256], [0; 256], [0; 256], [0; 256]]);
    }
}
