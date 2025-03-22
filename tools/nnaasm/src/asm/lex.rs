use std::borrow::Cow;
use std::path::PathBuf;
use std::str::FromStr;

use super::parse::Parser;
use super::{IntoAsmError, Located, Location};
use libnna::{u2, u4, InstructionSet, Op, OpArgs};

type Result<T> = std::result::Result<Located<T>, Located<LexError>>;

pub trait UnsignedNum
where
    Self: Sized,
{
    const THEORETICAL_SIZE: usize;
    fn parse_hex(str: &str) -> Option<Self>;
}
impl UnsignedNum for u8 {
    const THEORETICAL_SIZE: usize = 8;

    fn parse_hex(str: &str) -> Option<Self> {
        if str.len() > 2 {
            return None;
        }

        u8::from_str_radix(str, 16).ok()
    }
}
impl UnsignedNum for u4 {
    const THEORETICAL_SIZE: usize = 4;

    fn parse_hex(str: &str) -> Option<Self> {
        if str.len() != 1 {
            return None;
        }
        for char in str.chars() {
            return char.to_digit(16).map(|val| u4::from_u32(val));
        }
        return None;
    }
}
impl UnsignedNum for u2 {
    const THEORETICAL_SIZE: usize = 2;

    fn parse_hex(str: &str) -> Option<Self> {
        if str.len() != 1 {
            return None;
        }
        match str {
            "0" => Some(u2::ZERO),
            "1" => Some(u2::ONE),
            "2" => Some(u2::TOW),
            "3" => Some(u2::THREE),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum ValueToken<T: UnsignedNum> {
    LabelRef(Box<str>, RefType),
    Const(T),
}
pub type ValueToken8 = ValueToken<u8>;
pub type ValueToken4 = ValueToken<u4>;

#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
pub enum OpToken {
    Full(u8),
    LabelRef(u8, Box<str>, RefType),
}

#[derive(Debug, Clone)]
pub enum Token {
    LabelDef(Box<str>),
    Org(u8),
    Value(ValueToken<u8>),
    AssertMaxDist(ValueToken<u8>, u8),
    IncludeBytes(PathBuf),
    Bytes(Vec<u8>),
    Op(OpToken),
}
impl Token {
    pub fn is_org(&self) -> bool {
        match self {
            Self::Org(_) => true,
            _ => false,
        }
    }
}

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

fn parse_value<T: UnsignedNum>(
    token: &str,
    location: Location,
) -> std::result::Result<Option<Located<ValueToken<T>>>, Located<LexError>> {
    if token.starts_with("0x") {
        let value = T::parse_hex(&token[2..]).ok_or(LexError::located(
            format!("Invalid {} bit hex literal", T::THEORETICAL_SIZE).into(),
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

fn parse_next_value<T: UnsignedNum>(parser: &mut Parser) -> Result<ValueToken<T>> {
    let token = parser.next_same_line_or_err(Cow::Owned(format!(
        "Expected an {} bit value after this.",
        T::THEORETICAL_SIZE
    )))?;
    match parse_value::<T>(token, parser.location())? {
        Some(v) => Ok(v),
        None => Err(LexError::located(
            format!("Expected an {} bit value.", T::THEORETICAL_SIZE).into(),
            parser.location(),
        )),
    }
}
fn parse_next_reg(parser: &mut Parser) -> Result<u2> {
    let token = parser.next_same_line_or_err(Cow::Borrowed(
        "Expected a register after this. Found end of file",
    ))?;
    match token {
        "r0" => Ok(Located::new(u2::ZERO, parser.location())),
        "r1" => Ok(Located::new(u2::ONE, parser.location())),
        "r2" => Ok(Located::new(u2::TOW, parser.location())),
        "r3" => Ok(Located::new(u2::THREE, parser.location())),
        _ => Err(LexError::static_located(
            "Invalid register name",
            parser.location(),
        )),
    }
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
        "assert_max_dist" => {
            let start = parse_next_value::<u8>(parser)?;
            let dist = parse_next_hex8(parser)?;

            return Ok(Located::new(
                Token::AssertMaxDist(start.value, dist.value),
                token_loc.combine(dist.location),
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

fn parse_op<'a>(token: &'a str, iset: InstructionSet, parser: &mut Parser) -> Result<OpToken> {
    let (op, args) = Op::try_from_str(token, iset).ok_or(LexError::static_located(
        "Unknown operation. See spec for available operations",
        parser.location(),
    ))?;
    let loc = parser.location();
    Ok(match args {
        OpArgs::None => Located::new(OpToken::Full(op.into_u8()), loc),
        OpArgs::Bit4(_) => {
            let value = parse_next_value::<u4>(parser)?;
            let token = match value.value {
                ValueToken4::Const(value) => OpToken::Full(op.into_u8() | value.into_low()),
                ValueToken4::LabelRef(name, ref_type) => {
                    if ref_type.is_full() {
                        return Err(LexError::static_located("Cant fit a full sized reference (8 bits) into 4 bits. Use .low or .high sufix to get the low or high part of the reference.", parser.location()));
                    }
                    OpToken::LabelRef(op.into_u8(), name, ref_type)
                }
            };
            Located::new(token, loc.combine(value.location))
        }
        OpArgs::OneReg(_) => {
            let register = parse_next_reg(parser)?;

            Located::new(
                OpToken::Full(op.into_u8() | (register.value.into_low() << 2)),
                loc.combine(register.location),
            )
        }
        OpArgs::TowReg(_arg0_name, _arg1_name) => {
            let register0 = parse_next_reg(parser)?;
            let register1 = parse_next_reg(parser)?;

            Located::new(
                OpToken::Full(
                    op.into_u8() | register0.value.into_low() << 2 | register1.value.into_low(),
                ),
                loc.combine(register0.location).combine(register1.location),
            )
        }
    })
}

pub fn parse_lex(
    input: &str,
    iset: InstructionSet,
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

        out_vec.push(parse_op(token, iset, &mut parser)?.map(|opt| Token::Op(opt)));
    }
}
