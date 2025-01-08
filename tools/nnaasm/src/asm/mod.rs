use std::{
    ops::{Deref, Range},
    rc::Rc,
};

use self::parselex::parse_lex;

pub mod codegen;
mod parselex;
mod tokenizer;

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
            "{COLOR_RED}{BOLD}error:{RESET} {}:{}{}\n{}",
            self.filename, linenum, span.start, out
        )
    }
}
pub trait IntoAsmError {
    fn into_asm_error<'a>(self, code: &'a str, filename: Rc<str>) -> AsmError<'a>;
}

pub fn assemble(filename: Rc<str>, input: &str) -> Result<[u8; 256], AsmError> {
    let parsed = parse_lex(input).map_err(|lex| lex.into_asm_error(&input, filename.clone()))?;
    Ok(codegen::gen(parsed).map_err(|cg| cg.into_asm_error(&input, filename.clone()))?)
}
