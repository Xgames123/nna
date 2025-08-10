use std::borrow::Cow;
use std::path::PathBuf;
use std::str::FromStr;

use super::parse::Parser;
use super::{IntoAsmError, Located, Location};
use libnna::{u2, u4, Arch, ConstArg, MaxValue, OpArgs, ParseBin, ParseHex};

type Result<T> = std::result::Result<Located<T>, Located<LexError>>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValueToken<T> {
    LabelRef(Box<str>, RefType),
    Const(T),
}
pub type ValueToken8 = ValueToken<u8>;
pub type ValueToken4 = ValueToken<u4>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RefType {
    Full,
    Low,
    High,
}
impl RefType {
    ///Mask bits depending on the ref type.
    ///If 4 bit value put at the low end of the returning u8
    pub fn mask_low(self, value: u8) -> u8 {
        match self {
            RefType::Low => value & 0x0F,
            RefType::High => value >> 4 & 0x0F,
            RefType::Full => value,
        }
    }
    pub fn is_full(&self) -> bool {
        match self {
            Self::Full => true,
            _ => false,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OpToken {
    Full(u8),
    LabelRef(u8, Box<str>, RefType),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token {
    LabelDef(Box<str>),
    Org(u8),
    Value(ValueToken<u8>),
    Reachable(ValueToken<u8>),
    IncludeBytes(PathBuf),
    Bank(u8),
    Bytes(Vec<u8>),
    Op(OpToken),
}

#[derive(Debug, PartialEq, Eq)]
pub struct LexError {
    message: Cow<'static, str>,
}
impl LexError {
    pub fn new(message: Cow<'static, str>) -> Self {
        Self { message }
    }
    pub fn new_static(message: &'static str) -> Self {
        Self {
            message: Cow::Borrowed(message),
        }
    }
    pub fn located(message: Cow<'static, str>, location: Location) -> Located<Self> {
        Located::new(Self { message }, location)
    }
    pub fn static_located(message: &'static str, location: Location) -> Located<Self> {
        Located::new(
            Self {
                message: Cow::Borrowed(message),
            },
            location,
        )
    }
}
impl IntoAsmError for Located<LexError> {
    fn into_asm_error<'a>(self, code: &'a str, filename: std::rc::Rc<str>) -> super::AsmError<'a> {
        super::AsmError {
            filename,
            code,
            location: self.location,
            message: self.value.message.to_string(),
        }
    }
}

fn parse_identifier<'a>(str: &'a str) -> Option<&'a str> {
    for char in str.chars() {
        if !char.is_alphabetic() && char != '_' {
            println!("char: {}", char);
            return None;
        }
    }
    Some(&str[1..])
}

fn parse_value<T: ParseHex + ParseBin + MaxValue>(
    token: &str,
    location: Location,
) -> std::result::Result<Option<Located<ValueToken<T>>>, Located<LexError>> {
    if token.starts_with("0x") {
        let value = T::parse_hex(&token[2..]).ok_or(LexError::located(
            format!("Invalid {} bit hex literal", T::BIT_COUNT).into(),
            location.clone(),
        ))?;
        return Ok(Some(Located::new(ValueToken::Const(value), location)));
    }

    if token.starts_with("&") {
        let (token, ref_type) = if token.ends_with(".low") {
            (&token[1..token.len() - 4], RefType::Low)
        } else if token.ends_with(".high") {
            (&token[1..token.len() - 5], RefType::High)
        } else {
            (&token[1..], RefType::Full)
        };
        let value = parse_identifier(&token).ok_or(LexError::static_located(
            "Label ref contains invalid characters.",
            location.clone(),
        ))?;
        return Ok(Some(Located::new(
            ValueToken::LabelRef(value.into(), ref_type),
            location,
        )));
    }

    Ok(None)
}

fn parse_next_hex8(parser: &mut Parser) -> Result<u8> {
    let token = parser.next_same_line_or_err(Cow::Borrowed(
        "Expected an 8 bit constant value after this.",
    ))?;
    if !token.starts_with("0x") {
        return Err(LexError::static_located(
            "Expected an 8 bit constant value.",
            parser.location(),
        ));
    }
    let value = u8::parse_hex(&token[2..]).ok_or(LexError::static_located(
        "Invalid 8 bit hex literal",
        parser.location(),
    ))?;

    Ok(Located::new(value, parser.location()))
}

fn parse_next_value<T: ParseBin + ParseHex + MaxValue>(
    parser: &mut Parser,
) -> Result<ValueToken<T>> {
    let token = parser.next_same_line_or_err(Cow::Owned(format!(
        "Expected an {} bit value after this.",
        T::MAX_VALUE
    )))?;
    match parse_value::<T>(token, parser.location())? {
        Some(v) => Ok(v),
        None => Err(LexError::located(
            format!("Expected an {} bit value.", T::BIT_COUNT).into(),
            parser.location(),
        )),
    }
}
fn parse_next_constarg(parser: &mut Parser, constarg: ConstArg) -> Result<u8> {
    let token = parser.next_same_line_or_err(
        format!(
            "Expected a '{}' after this. Found end of file",
            constarg.name
        )
        .into(),
    )?;
    for (i, var) in constarg.variants.iter().enumerate() {
        if *var == "?" {
            continue;
        }
        if *var == token {
            return Ok(Located::new(i as u8, parser.location()));
        }
    }
    Err(LexError::located(
        format!("Invalid '{}'", constarg.name).into(),
        parser.location(),
    ))
}

fn parse_next_str<'a>(parser: &'a mut Parser) -> Result<&'a str> {
    let token = parser.next_same_line_or_err(Cow::Borrowed(
        "Expected a string literal after this. Found end of file",
    ))?;
    if !token.starts_with("\"") {
        return Err(LexError::static_located(
            "Expected a string literal.",
            parser.location(),
        ));
    }
    if !token.ends_with("\"") {
        return Err(LexError::static_located(
            "String doesn't have an ending '\"'",
            parser.location(),
        ));
    }

    Ok(Located::new(token, parser.location()))
}

fn parse_compiler_directive<'a>(token: &'a str, parser: &mut Parser) -> Result<Token> {
    let token_loc = parser.location();
    match token {
        "org" => {
            let addr = parse_next_hex8(parser)?;
            return Ok(Located::new(
                Token::Org(addr.value),
                token_loc.combine(addr.location),
            ));
        }
        "bank" => {
            let addr = parse_next_hex8(parser)?;
            return Ok(Located::new(
                Token::Bank(addr.value),
                token_loc.combine(addr.location),
            ));
        }
        "reachable" => {
            let start = parse_next_value::<u8>(parser)?;

            return Ok(Located::new(
                Token::Reachable(start.value),
                token_loc.combine(start.location),
            ));
        }
        "include_bytes" => {
            let path_str = parse_next_str(parser)?;
            let path = PathBuf::from_str(&path_str).unwrap();
            return Ok(Located::new(
                Token::IncludeBytes(path),
                token_loc.combine(parser.location()),
            ));
        }
        _ => {
            return Err(LexError::static_located(
                "Unknown compiler directive",
                parser.location(),
            ));
        }
    }
}

fn parse_op<I: Arch + Into<u8>>(token: &str, parser: &mut Parser) -> Result<OpToken> {
    let op = I::try_from_str(token).ok_or(LexError::static_located(
        "Unknown operation. See spec for available operations",
        parser.location(),
    ))?;
    let loc = parser.location();
    Ok(match op.args() {
        OpArgs::None => Located::new(OpToken::Full(op.into()), loc),
        OpArgs::Bit4(_) => {
            let value = parse_next_value::<u4>(parser)?;
            let token = match value.value {
                ValueToken4::Const(value) => OpToken::Full(op.into() | value.into_low()),
                ValueToken4::LabelRef(name, ref_type) => {
                    if ref_type.is_full() {
                        return Err(LexError::static_located("Cant fit a full sized reference (8 bits) into 4 bits. Use .low or .high sufix to get the low or high part of the reference.", parser.location()));
                    }
                    OpToken::LabelRef(op.into(), name, ref_type)
                }
            };
            Located::new(token, loc.combine(value.location))
        }
        OpArgs::ConstNone((_, arg)) => {
            let register = parse_next_constarg(parser, arg)?;

            Located::new(
                OpToken::Full(op.into() | (register.value << 2)),
                loc.combine(register.location),
            )
        }
        OpArgs::ConstConst((_, arg0), (_, arg1)) => {
            let register0 = parse_next_constarg(parser, arg0)?;
            let register1 = parse_next_constarg(parser, arg1)?;

            Located::new(
                OpToken::Full(op.into() | register0.value << 2 | register1.value),
                loc.combine(register0.location).combine(register1.location),
            )
        }
        OpArgs::ConstBit2((_, arg0), _) => {
            let register0 = parse_next_constarg(parser, arg0)?;
            let value1 = match parse_next_value::<u2>(parser)? {
                Located {
                    location,
                    value: ValueToken::Const(v),
                } => Located::new(v, location),
                Located {
                    location,
                    value: ValueToken::LabelRef(_, _),
                } => {
                    return Err(LexError::static_located(
                        "label not allowed for 2 bit values",
                        location,
                    ))
                }
            };

            Located::new(
                OpToken::Full(op.into() | register0.value << 2 | value1.value.into_low()),
                loc.combine(register0.location).combine(value1.location),
            )
        }
    })
}

pub fn parse_lex<I: Arch + Into<u8>>(
    input: &str,
) -> std::result::Result<Vec<Located<Token>>, Located<LexError>> {
    let mut out_vec = Vec::new();
    let Some(mut parser) = Parser::new(input) else {
        return Ok(out_vec);
    };

    loop {
        let Some(token) = parser.next() else {
            return Ok(out_vec);
        };
        //println!("token: '{}'", token);
        if token.starts_with('.') {
            out_vec.push(parse_compiler_directive(&token[1..], &mut parser)?);
            continue;
        }
        if token.ends_with(':') {
            out_vec.push(
                parse_identifier(&token[..token.len() - 1])
                    .map(|label| Located::new(Token::LabelDef(label.into()), parser.location()))
                    .ok_or(LexError::static_located(
                        "invalid label name",
                        parser.location(),
                    ))?,
            );
            continue;
        }

        if let Some(value) = parse_value::<u8>(token, parser.location())? {
            out_vec.push(value.map(|vt| Token::Value(vt)));
            continue;
        }

        out_vec.push(parse_op::<I>(token, &mut parser)?.map(|opt| Token::Op(opt)));
    }
}

#[cfg(test)]
mod test {
    use libnna::instruction_sets::Nna8v1;

    use crate::asm::{lex::Token, Located};

    use super::parse_lex;

    #[test]
    fn parse_org() {
        let code = r#"
.org 0xAB

;comment "#;
        assert_eq!(
            parse_lex::<Nna8v1>(code),
            Ok(vec![Located::new(Token::Org(0xAB), (1, 0..9).into())])
        )
    }
}
