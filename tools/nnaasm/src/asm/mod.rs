use std::{
    fs::File,
    io::Read,
    ops::{Deref, Range},
    rc::Rc,
};

use self::lex::parse_lex;

pub mod codegen;
mod lex;
mod parse;

const COLOR_RED: &'static str = "\x1b[31m";
const BOLD: &'static str = "\x1b[1m";
const RESET: &'static str = "\x1b[0m";

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Location(usize, Range<usize>);
impl From<(usize, Range<usize>)> for Location {
    fn from(value: (usize, Range<usize>)) -> Self {
        if value.1.end < value.1.start {
            panic!("Starting value can't be greater than ending value. In a span")
        }
        Self(value.0, value.1)
    }
}
impl Into<(usize, Range<usize>)> for Location {
    fn into(self) -> (usize, Range<usize>) {
        (self.0, self.1)
    }
}
impl Location {
    pub fn combine(self, other: Location) -> Self {
        if other.0 != self.0 {
            panic!("Can't combine locations on different lines");
        }
        if other.1.start < self.1.start {
            Self(self.0, other.1.start..self.1.end)
        } else {
            Self(self.0, self.1.start..other.1.end)
        }
    }
}
#[derive(Debug)]
pub struct Located<T> {
    pub location: Location,
    pub value: T,
}
impl<T> Located<T> {
    pub fn new(value: T, location: Location) -> Located<T> {
        Located { value, location }
    }
    pub fn map<T2, F: FnOnce(T) -> T2>(self, f: F) -> Located<T2> {
        Located::new(f(self.value), self.location)
    }
}
impl<T> Deref for Located<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

#[derive(Debug)]
pub struct AsmError<'a> {
    pub filename: Rc<str>,
    pub code: &'a str,
    pub location: Location,
    pub message: String,
}
impl<'a> AsmError<'a> {
    const VIEW_SIZE: usize = 2;

    fn write_gutter(out: &mut String, line_num: Option<usize>, max_len: usize) {
        match line_num {
            Some(lnum) => {
                let lnum = lnum.saturating_add(1);
                for _ in Self::calc_len(lnum)..max_len {
                    out.push(' ');
                }
                out.push_str(&lnum.to_string());
            }
            None => {
                for _ in 0..max_len {
                    out.push(' ');
                }
            }
        };

        out.push_str(" | ");
    }
    fn calc_len(num: usize) -> usize {
        let mut digits = 0;
        let mut i = 1;
        while i <= num {
            digits += 1;
            i *= 10;
        }
        digits
    }

    pub fn print(&self) {
        let (linenum, span) = self.location.clone().into();
        let max_len = Self::calc_len(linenum.saturating_add(Self::VIEW_SIZE + 1));

        let mut out = String::new();
        for (i, line) in self.code.lines().enumerate() {
            if i < linenum.saturating_sub(Self::VIEW_SIZE)
                || i > linenum.saturating_add(Self::VIEW_SIZE)
            {
                continue;
            }
            Self::write_gutter(&mut out, Some(i), max_len);
            out.push_str(line);
            out.push('\n');
            if i == linenum {
                Self::write_gutter(&mut out, None, max_len);
                for _ in 0..span.start {
                    out.push(' ');
                }
                out.push_str(COLOR_RED);
                let count = (span.end - span.start).max(1);
                for _ in 0..count {
                    out.push('^');
                }
                out.push(' ');
                out.push_str(&self.message);
                out.push_str(RESET);
                out.push('\n');
            }
        }
        eprintln!(
            "{COLOR_RED}{BOLD}error:{RESET} {}:{}:{}\n{}",
            self.filename, linenum, span.start, out
        )
    }
}
pub trait IntoAsmError {
    fn into_asm_error<'a>(self, code: &'a str, filename: Rc<str>) -> AsmError<'a>;
}

fn io_to_asm_err<'a>(
    io: std::io::Error,
    location: Location,
    code: &'a str,
    filename: Rc<str>,
) -> AsmError<'a> {
    let message = match io.kind() {
        std::io::ErrorKind::NotFound => "File not found".to_string(),
        _ => {
            format!("io error: {}", io)
        }
    };
    AsmError {
        filename,
        code,
        location: location,
        message,
    }
}

fn resolve_includes<'a>(
    tokens: &mut Vec<Located<lex::Token>>,
    code: &'a str,
    filename: Rc<str>,
) -> Result<(), AsmError<'a>> {
    for token in tokens.iter_mut() {
        match &token.value {
            lex::Token::IncludeBytes(path) => {
                let mut file = File::open(path).map_err(|e| {
                    io_to_asm_err(e, token.location.clone(), code, filename.clone())
                })?;
                let mut buffer = Vec::new();
                file.read_to_end(&mut buffer).map_err(|e| {
                    io_to_asm_err(e, token.location.clone(), code, filename.clone())
                })?;
                let _ = std::mem::replace(
                    token,
                    Located::new(lex::Token::Bytes(buffer), token.location.clone()),
                );
            }
            _ => {}
        }
    }
    Ok(())
}

pub fn assemble(filename: Rc<str>, input: &str) -> Result<[u8; 256], AsmError> {
    let mut parsed =
        parse_lex(input).map_err(|lex| lex.into_asm_error(&input, filename.clone()))?;
    resolve_includes(&mut parsed, &input, filename.clone())?;
    Ok(codegen::gen(parsed).map_err(|cg| cg.into_asm_error(&input, filename.clone()))?)
}

#[cfg(test)]
mod tests {
    use super::Located;

    fn assemble_assert(code: &str, bin: &[u8]) {
        let mut full_bin = [0; 256];
        full_bin[..bin.len()].copy_from_slice(bin);
        match super::assemble("test".into(), code) {
            Ok(gen_bin) => assert_eq!(gen_bin, full_bin),
            Err(e) => {
                e.print();
                assert!(false)
            }
        }
    }
    fn assemble_assert_err(code: &str, err: Located<&str>) {
        match super::assemble("test".into(), &code) {
            Ok(_) => assert!(false, "An error should be thrown. but isn't"),
            Err(e) => {
                assert_eq!(e.message, err.value);
                assert_eq!(e.location, err.location);
            }
        }
    }
    #[test]
    fn full_test() {
        let code = r#"
.org 0x00

lih 0x2
lil 0x4
mov r1 r0"#;

        let mut bin = [0; 256];
        bin[0] = 0x22;
        bin[1] = 0x14;
        bin[2] = 0x54;
        assemble_assert(code, &[0x22, 0x14, 0x54]);
    }

    #[test]
    fn org_overflow() {
        let code = r#".org 0x10
nop
nop
.org 0x11
nop
        "#;
        assemble_assert_err(
            code,
            Located::new(
                "This org (.org 0x11 ; size: 0x01) overlaps with: .org 0x10 ; size: 0x02",
                (3, 0..9).into(),
            ),
        );
    }

    #[test]
    fn max_dist_fail() {
        let code = r#"
.org 0x20
nop
nop
nop
nop
nop
.assert_max_dist 0x20 0x4
"#;
        assemble_assert_err(
            code,
            Located::new("Assertion failed. distance was 0x05", (7, 0..25).into()),
        );
    }

    #[test]
    fn max_dist_success() {
        let code = r#"
.org 0x20
nop ; 0x20
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop ; 0x2A
nop
nop
nop
nop
nop ; jmp
.assert_max_dist 0x20 0x10
"#;
        assemble_assert(code, &[0; 0]);
    }
}
