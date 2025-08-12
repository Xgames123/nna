use std::borrow::Cow;
use std::path::PathBuf;
use std::str::FromStr;

use super::parse::Parser;
use super::{IntoAsmError, Located, Location};
use libnna::instruction_sets::{Nna8v1, Nna8v2};
use libnna::{
    u2, u4, Arch, Architecture, ConstArg, MaxValue, OpArg, OpArgType, OpArgs, ParseBin, ParseHex,
};

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
impl OpToken {
    fn full_or<E>(self, e: E) -> std::result::Result<u8, E> {
        match self {
            Self::Full(v) => Ok(v),
            Self::LabelRef(_, _, _) => Err(e),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token {
    LabelDef(Box<str>),
    Org(u8),
    Value(ValueToken<u8>),
    Reachable(ValueToken<u8>),
    Bank(u8),
    Bytes(Vec<u8>),
    Op(OpToken),
    Arch(Architecture),
    IncludeBytes(PathBuf),
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
    if token.starts_with("0b") {
        let value = T::parse_bin(&token[2..]).ok_or(LexError::located(
            format!("Invalid {} bit binary literal", T::BIT_COUNT).into(),
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

    Ok(Located::new(&token[1..token.len() - 1], parser.location()))
}

fn parse_compiler_directive<'a>(token: &'a str, parser: &mut Parser) -> Result<Token> {
    let token_loc = parser.location();
    match token {
        "org" => {
            let addr = parse_next_hex8(parser)?;
            return Ok(Located::new(
                Token::Org(addr.value).into(),
                token_loc.combine(addr.location),
            ));
        }
        "bank" => {
            let addr = parse_next_hex8(parser)?;
            return Ok(Located::new(
                Token::Bank(addr.value).into(),
                token_loc.combine(addr.location),
            ));
        }
        "reachable" => {
            let start = parse_next_value::<u8>(parser)?;

            return Ok(Located::new(
                Token::Reachable(start.value).into(),
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
        "arch" => {
            let arch_t = parse_next_str(parser)?;
            let arch = Architecture::from_str(arch_t.value).ok_or(LexError::static_located(
                "Unknown architecture name",
                arch_t.location.clone(),
            ))?;
            return Ok(Located::new(
                Token::Arch(arch),
                token_loc.combine(arch_t.location),
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

fn parse_oparg(arg: OpArg, big: bool, parser: &mut Parser) -> Result<OpToken> {
    match arg.ty {
        OpArgType::None => Ok(Located::new(OpToken::Full(0), parser.location())),
        OpArgType::Const(c) => {
            let arg = parse_next_constarg(parser, c)?;
            Ok(arg.map(|v| OpToken::Full(v)))
        }
        OpArgType::Value { nz: true } => {
            if big {
                panic!("Generating code for 4 bit nonzero values is currently not supported. But an instruction in the current arch exists that requires it.");
            }
            Ok(match parse_next_value::<u4>(parser)? {
                Located {
                    location,
                    value: ValueToken::Const(v),
                } => {
                    if v.into_low() > 4 || v.into_low() == 0 {
                        return Err(LexError::static_located(
                            "non zero 2 bit value is not allowed to be 0 or bigger than 4",
                            location,
                        ));
                    }
                    Located::new(OpToken::Full(v.into_low() - 1), location)
                }
                Located {
                    location,
                    value: ValueToken::LabelRef(_, _),
                } => {
                    return Err(LexError::static_located(
                        "label not allowed for 2 bit values",
                        location,
                    ))
                }
            })
        }
        OpArgType::Value { nz: false } => Ok(if big {
            let value = parse_next_value::<u4>(parser)?;
            let token = match value.value {
                ValueToken4::Const(value) => OpToken::Full(value.into_low()),
                ValueToken4::LabelRef(name, ref_type) => {
                    if ref_type.is_full() {
                        return Err(LexError::static_located("Cant fit a full sized reference (8 bits) into 4 bits. Use .low or .high suffix to get the low or high part of the reference.", parser.location()));
                    }
                    OpToken::LabelRef(0, name, ref_type)
                }
            };
            Located::new(token, parser.location().combine(value.location))
        } else {
            match parse_next_value::<u2>(parser)? {
                Located {
                    location,
                    value: ValueToken::Const(v),
                } => Located::new(v.into_low(), location),
                Located {
                    location,
                    value: ValueToken::LabelRef(_, _),
                } => {
                    return Err(LexError::static_located(
                        "label not allowed for 2 bit values",
                        location,
                    ))
                }
            }
            .map(|v| OpToken::Full(v))
        }),
    }
}

fn parse_op<I: Arch + Into<u8>>(token: &str, parser: &mut Parser) -> Result<OpToken> {
    let op = I::try_from_str(token).ok_or(LexError::static_located(
        "Unknown operation. See spec for available operations",
        parser.location(),
    ))?;
    Ok(match op.args() {
        OpArgs::Arg(a) => parse_oparg(a.0, true, parser)?.map(|t| match t {
            OpToken::Full(v) => OpToken::Full(op.into() | v),
            OpToken::LabelRef(v, name, ty) => OpToken::LabelRef(op.into() | v, name, ty),
        }),
        OpArgs::ArgArg(a, b) => {
            let a = parse_oparg(a, false, parser)?
                .map(|a| {
                    a.full_or(LexError::static_located(
                        "Cant use a reference for a 2 bit value",
                        parser.location(),
                    ))
                })
                .lift_ok()?;
            let b = parse_oparg(b, false, parser)?
                .map(|b| {
                    b.full_or(LexError::static_located(
                        "Cant use a refference for a 2 bit value",
                        parser.location(),
                    ))
                })
                .lift_ok()?;

            Located::new(
                OpToken::Full(op.into() | a.value << 2 | b.value),
                a.location.combine(b.location),
            )
        }
    })
}

pub fn parse_lex(
    input: &str,
    default_arch: Architecture,
) -> std::result::Result<Vec<Located<Token>>, Located<LexError>> {
    let mut out_vec = Vec::new();
    let Some(mut parser) = Parser::new(input) else {
        return Ok(out_vec);
    };

    let mut arch = default_arch;
    let mut parsed_ops = false;

    loop {
        let Some(token) = parser.next() else {
            return Ok(out_vec);
        };
        //println!("token: '{}'", token);
        if token.starts_with('.') {
            let t = parse_compiler_directive(&token[1..], &mut parser)?;
            match t.value {
                Token::Arch(a) => {
                    if parsed_ops {
                        return Err(LexError::static_located(
                            "Can't use .arch after some operations have been parsed.",
                            parser.location(),
                        ));
                    }
                    arch = a
                }
                _ => {}
            };
            out_vec.push(t);
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

        parsed_ops = true;
        out_vec.push(
            match arch {
                Architecture::Nna8v1 => parse_op::<Nna8v1>(token, &mut parser),
                Architecture::Nna8v2 => parse_op::<Nna8v2>(token, &mut parser),
            }?
            .map(|opt| Token::Op(opt)),
        );
    }
}

#[cfg(test)]
mod test {
    use crate::asm::{lex::Token, Located};

    use super::parse_lex;

    #[test]
    fn parse_org() {
        let code = r#"
.org 0xAB

;comment "#;
        assert_eq!(
            parse_lex(code, libnna::Architecture::Nna8v1),
            Ok(vec![Located::new(Token::Org(0xAB), (1, 0..9).into())])
        )
    }

    #[test]
    fn parse_arch() {
        let code = r#".arch "nna8v2"
.org 0xAB
        "#;
        assert_eq!(
            parse_lex(code, libnna::Architecture::Nna8v1),
            Ok(vec![
                Located::new(Token::Arch(libnna::Architecture::Nna8v2), (0, 0..14).into()),
                Located::new(Token::Org(0xAB), (1, 0..9).into())
            ])
        )
    }
}
