use std::borrow::Cow;

use super::{Located, Location};

pub struct CodeParser<'a> {
    cur_linenum: usize,
    cur_index: usize,
    code: &'a str,
    cur_col: usize,
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
            cur_col: 0,
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
        let size = index - self.cur_index;
        self.cur_index = index;
        self.cur_col += size;
        if char == '\n' {
            self.cur_linenum += 1;
            self.cur_col = 0;
        }
        if char == ';' {
            self.skip_line();
            return self.next_char();
        }
        Some((index, char))
    }

    pub fn code(&self) -> &'a str {
        self.code
    }
    pub fn location(&self) -> Location {
        self.last_location.clone()
    }
    pub fn next_same_line_or_err(
        &mut self,
        message: Cow<'static, str>,
    ) -> Result<&'a str, Located<super::parselex::LexError>> {
        self.next_same_line()
            .ok_or(super::parselex::LexError::located(
                message,
                self.last_location.clone(),
            ))
    }

    pub fn next_same_line(&mut self) -> Option<&'a str> {
        let (start_index, start_col) = loop {
            let (index, char) = self.next_char()?;
            if char == '\n' {
                return None;
            }
            if !char.is_whitespace() {
                break (index, self.cur_col);
            }
        };
        return self.to_end_of_token(start_index, start_col);
    }

    fn to_end_of_token(&mut self, start_index: usize, start_col: usize) -> Option<&'a str> {
        loop {
            let col = self.cur_col;
            let linenum = self.cur_linenum;
            let (index, char) = self.next_char()?;
            if char.is_whitespace() {
                self.last_location = (linenum, start_col..col + 1).into();
                return Some(&self.code[start_index..index]);
            }
        }
    }
}
impl<'a> Iterator for CodeParser<'a> {
    type Item = &'a str;
    fn next(&mut self) -> Option<Self::Item> {
        let (start_index, start_col) = loop {
            let prev_col = self.cur_col;
            let (index, char) = self.next_char()?;
            if !char.is_whitespace() {
                break (index, prev_col);
            }
        };
        self.to_end_of_token(start_index, start_col)
    }
}
