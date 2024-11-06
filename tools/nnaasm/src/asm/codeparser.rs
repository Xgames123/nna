use std::borrow::Cow;

use super::{Located, Location};

pub struct CodeParser<'a> {
    cur_linenum: usize,
    code: &'a str,
    cur_index: usize,
    char_iter: std::str::CharIndices<'a>,
    last_location: Location,
}
impl<'a> CodeParser<'a> {
    pub fn new(code: &'a str) -> Option<Self> {
        Some(Self {
            last_location: (0, 0..0).into(),
            code,
            cur_linenum: 0,
            cur_index: 0,
            char_iter: code.char_indices(),
        })
    }
    pub fn skip_line(&mut self) {
        loop {
            match self.next_char() {
                None => {
                    return;
                }
                Some((_, char)) => {
                    if char == '\n' {
                        return;
                    }
                }
            }
        }
    }
    pub fn next_char(&mut self) -> Option<(usize, char)> {
        let (index, char) = self.char_iter.next()?;
        self.cur_index += 1;
        if char == '\n' {
            self.cur_linenum += 1;
            self.cur_index = 0;
        }
        Some((index, char))
    }
    pub fn code(&self) -> &'a str {
        self.code
    }
    pub fn location(&self) -> Location {
        self.last_location.clone()
    }
    pub fn next_or_err(
        &mut self,
        message: Cow<'static, str>,
    ) -> Result<&'a str, Located<super::parselex::LexError>> {
        self.next()
            .ok_or(super::parselex::LexError.located(message, self.last_location.clone()))
    }
    pub fn next_same_line_or_err(
        &mut self,
        message: Cow<'static, str>,
    ) -> Result<&'a str, Located<super::parselex::LexError>> {
        self.next_same_line()
            .ok_or(super::parselex::LexError.located(message, self.last_location.clone()))
    }

    pub fn next_same_line(&mut self) -> Option<&'a str> {
        let start_index = loop {
            let (index, char) = self.next_char()?;
            if char == '\n' {
                return None;
            }
            if !char.is_whitespace() {
                break index;
            }
        };
        let range_start = self.cur_index;
        loop {
            let (index, char) = self.next_char()?;
            if char.is_whitespace() {
                let loc: Location = (self.cur_linenum, range_start..self.cur_index).into();
                self.last_location = loc.clone();
                return Some(&self.code[start_index..index]);
            }
        }
    }
}
impl<'a> Iterator for CodeParser<'a> {
    type Item = &'a str;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let (_, char) = self.next_char()?;
            if !char.is_whitespace() {
                break;
            }
        }
        self.next_same_line()
    }
}
