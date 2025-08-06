use std::borrow::Cow;

use super::{Located, Location};

struct CodeIter<'a> {
    code: &'a str,
    index: usize,
    col_index: usize,
    line: usize,
    cur_char: char,
    char_iter: std::str::CharIndices<'a>,
}
impl<'a> CodeIter<'a> {
    pub fn new(code: &'a str) -> Self {
        Self {
            code,
            index: 0,
            col_index: 0,
            line: 0,
            cur_char: '\0',
            char_iter: code.char_indices(),
        }
    }
    pub fn end(&self) -> bool {
        self.code.len() == self.index
    }

    pub fn next(&mut self) -> Option<char> {
        let Some((index, char)) = self.char_iter.next() else {
            self.index = self.code.len();
            return None;
        };

        let size = index - self.index;
        self.index = index;

        self.col_index += size;
        if self.cur_char == '\n' {
            self.line += 1;
            self.col_index = 0;
        }
        self.cur_char = char;
        Some(char)
    }
    pub fn cur_char(&self) -> char {
        self.cur_char
    }
    pub fn col_index(&self) -> usize {
        self.col_index
    }
    pub fn line(&self) -> usize {
        self.line
    }
    pub fn index(&self) -> usize {
        self.index
    }
    pub fn code(&self) -> &'a str {
        self.code
    }
}

pub struct Parser<'a> {
    last_location: Location,
    codeiter: CodeIter<'a>,
}
impl<'a> Parser<'a> {
    pub fn new(code: &'a str) -> Option<Self> {
        Some(Self {
            last_location: (0, 0..0).into(),
            codeiter: CodeIter::new(code),
        })
    }
    pub fn skip_line(&mut self) -> Option<()> {
        if self.codeiter.cur_char() == '\n' {
            return Some(());
        }
        while self.codeiter.next()? != '\n' {}
        Some(())
    }
    pub fn next_char_skip_comments(&mut self) -> Option<char> {
        let char = self.codeiter.next()?;
        if char == ';' {
            return self.skip_line().map(|()| self.codeiter.cur_char());
        }
        Some(char)
    }

    pub fn code(&self) -> &'a str {
        self.codeiter.code()
    }
    pub fn location(&self) -> Location {
        self.last_location.clone()
    }
    pub fn next_same_line_or_err(
        &mut self,
        message: Cow<'static, str>,
    ) -> Result<&'a str, Located<super::lex::LexError>> {
        self.next_same_line().ok_or(super::lex::LexError::located(
            message,
            self.last_location.clone(),
        ))
    }

    pub fn next_same_line(&mut self) -> Option<&'a str> {
        loop {
            let cur_char = self.codeiter.cur_char();
            // println!(
            //     "next_same_line: cur_char:{} cur_col:{}",
            //     self.cur_char, self.cur_col
            // );
            if cur_char == '\n' {
                return None;
            }
            if !cur_char.is_whitespace() {
                return self.read_cur_token();
            }
            self.next_char_skip_comments()?;
        }
    }

    /// NOTE: The cursor needs to be on the first byte of the token to read.
    fn read_cur_token(&mut self) -> Option<&'a str> {
        //println!("read cur token start_col: {}", self.cur_col);
        if self.codeiter.end() {
            return None;
        }
        let start_col = self.codeiter.col_index();
        let start_index = self.codeiter.index();
        let cur_char = self.codeiter.cur_char();
        let string_token = cur_char == '"';
        if string_token {
            self.codeiter.next();
        }
        loop {
            let cur_char = self.codeiter.cur_char();
            let line = self.codeiter.line();
            let col = self.codeiter.col_index();
            let index = self.codeiter.index();

            if (!string_token && cur_char.is_whitespace()) || cur_char == '\n' {
                self.last_location = (line, start_col..col).into();
                return Some(&self.codeiter.code()[start_index..index]);
            }
            if (string_token && cur_char == '"') | self.next_char_skip_comments().is_none() {
                self.last_location = (line, start_col..col + 1).into();
                return Some(&self.codeiter.code()[start_index..index + 1]);
            }
        }
    }
}
impl<'a> Iterator for Parser<'a> {
    type Item = &'a str;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            //println!("cur_char: {}", self.cur_char);
            let cur_char = self.codeiter.cur_char();
            if !cur_char.is_whitespace() && cur_char != '\0' {
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

        let mut cp = Parser::new(code).unwrap();

        println!("char 1");
        cp.next_char_skip_comments();
        assert_eq!(cp.codeiter.cur_char(), '1', "cur_char");
        assert_eq!(cp.codeiter.index(), 0, "index");
        assert_eq!(cp.codeiter.col_index(), 0, "col_index");
        assert_eq!(cp.codeiter.line(), 0, "line_index");

        println!("char newline");
        cp.next_char_skip_comments();
        assert_eq!(cp.codeiter.cur_char(), '\n', "cur_char");
        assert_eq!(cp.codeiter.index(), 1, "index");
        assert_eq!(cp.codeiter.col_index(), 1, "col_index");
        assert_eq!(cp.codeiter.line(), 0, "line");

        println!("char 2");
        cp.next_char_skip_comments();
        assert_eq!(cp.codeiter.cur_char(), '2', "cur_char");
        assert_eq!(cp.codeiter.index(), 2, "index");
        assert_eq!(cp.codeiter.col_index(), 0, "col_index");
        assert_eq!(cp.codeiter.line(), 1, "line");
    }

    #[test]
    fn next() {
        let code = r#"linezero

token1 ; comment
token2
token3
r1 r0"#;

        let mut cp = Parser::new(code).unwrap();

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
        let mut cp = Parser::new(code).unwrap();
        assert_eq!(cp.next(), Some("token0"));
        assert_eq!(cp.next(), Some("between_token"));
        assert_eq!(cp.next(), Some("token_attached_to_end"));
        assert_eq!(cp.last_location, Location::from((5, 0..21)));
    }

    #[test]
    fn strings() {
        let code = r#""first string"
token
"half string
"

"end string""#;
        let mut p = Parser::new(code).unwrap();
        assert_eq!(p.next(), Some("\"first string\""));
        assert_eq!(p.next(), Some("token"));
        assert_eq!(p.next(), Some("\"half string"));
        assert_eq!(p.next(), Some("\""));
        assert_eq!(p.next(), Some("\"end string\""));
        assert_eq!(p.next(), None);
    }
}
