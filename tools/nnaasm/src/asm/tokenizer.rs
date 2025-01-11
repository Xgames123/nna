use std::borrow::Cow;

use super::{Located, Location};

pub struct Tokenizer<'a> {
    cur_linenum: usize,
    cur_index: usize,
    code: &'a str,
    cur_col: usize,
    cur_char: char,
    end: bool,
    char_iter: std::str::CharIndices<'a>,
    last_location: Location,
}
impl<'a> Tokenizer<'a> {
    pub fn new(code: &'a str) -> Option<Self> {
        Some(Self {
            last_location: (0, 0..0).into(),
            cur_char: '\0',
            code,
            end: false,
            cur_linenum: 0,
            cur_index: 0,
            cur_col: 0,
            char_iter: code.char_indices(),
        })
    }
    pub fn skip_line(&mut self) -> Option<()> {
        loop {
            if self.cur_char == '\n' {
                return Some(());
            }
            self.next_char_skip_comments()?;
        }
    }
    pub fn next_char(&mut self) -> Option<()> {
        let Some((index, char)) = self.char_iter.next() else {
            self.end = true;
            return None;
        };
        let size = index - self.cur_index;
        self.cur_index = index;
        self.cur_col += size;
        if self.cur_char == '\n' {
            self.cur_linenum += 1;
            self.cur_col = 0;
            // println!("newline");
        }
        self.cur_char = char;
        Some(())
    }
    pub fn next_char_skip_comments(&mut self) -> Option<()> {
        self.next_char()?;
        if self.cur_char == ';' {
            return self.skip_line();
        }
        Some(())
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
        loop {
            // println!(
            //     "next_same_line: cur_char:{} cur_col:{}",
            //     self.cur_char, self.cur_col
            // );
            if self.cur_char == '\n' {
                return None;
            }
            if !self.cur_char.is_whitespace() {
                return self.read_cur_token();
            }
            self.next_char_skip_comments()?;
        }
    }

    fn read_cur_token(&mut self) -> Option<&'a str> {
        //println!("read cur token start_col: {}", self.cur_col);
        if self.end {
            return None;
        }
        let start_col = self.cur_col;
        let start_index = self.cur_index;
        loop {
            if self.cur_char.is_whitespace() {
                self.last_location = (self.cur_linenum, start_col..self.cur_col).into();
                return Some(&self.code[start_index..self.cur_index]);
            }
            if self.next_char_skip_comments().is_none() {
                self.last_location = (self.cur_linenum, start_col..self.cur_col + 1).into();
                return Some(&self.code[start_index..self.cur_index + 1]);
            }
        }
    }
}
impl<'a> Iterator for Tokenizer<'a> {
    type Item = &'a str;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            //println!("cur_char: {}", self.cur_char);
            if !self.cur_char.is_whitespace() && self.cur_char != '\0' {
                return self.next_same_line();
            }
            self.next_char_skip_comments()?;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn next_char() {
        let code = r#"1
2 ; comment
"#;

        let mut cp = Tokenizer::new(code).unwrap();

        cp.next_char_skip_comments();
        assert_eq!(cp.cur_char, '1');
        assert_eq!(cp.cur_index, 0);
        assert_eq!(cp.cur_col, 0);
        assert_eq!(cp.cur_linenum, 0);
        cp.next_char_skip_comments();

        assert_eq!(cp.cur_char, '\n');
        assert_eq!(cp.cur_index, 1);
        assert_eq!(cp.cur_col, 1);
        assert_eq!(cp.cur_linenum, 0);
        cp.next_char_skip_comments();

        assert_eq!(cp.cur_char, '2');
        assert_eq!(cp.cur_index, 2);
        assert_eq!(cp.cur_col, 0);
        assert_eq!(cp.cur_linenum, 1);
    }

    #[test]
    fn next() {
        let code = r#"linezero

token1 ; comment
token2
token3
r1 r0"#;

        let mut cp = Tokenizer::new(code).unwrap();

        assert_eq!(cp.next(), Some("linezero"));
        assert_eq!(cp.last_location, Location::from((0, 0..8)));
        assert_eq!(cp.next(), Some("token1"));
        assert_eq!(cp.last_location, Location::from((2, 0..6)));
        assert_eq!(cp.next(), Some("token2"));
        assert_eq!(cp.last_location, Location::from((3, 0..6)));
        assert_eq!(cp.next(), Some("token3"));
        assert_eq!(cp.last_location, Location::from((4, 0..6)));
        assert_eq!(cp.next(), Some("r1"));
        assert_eq!(cp.next(), Some("r0"));
        assert_eq!(cp.next(), None);
    }

    #[test]
    fn whitespace() {
        let code = r#"
token0

between_token

token_attached_to_end"#;
        let mut cp = Tokenizer::new(code).unwrap();
        assert_eq!(cp.next(), Some("token0"));
        assert_eq!(cp.next(), Some("between_token"));
        assert_eq!(cp.next(), Some("token_attached_to_end"));
        assert_eq!(cp.last_location, Location::from((5, 0..21)));
    }
}
