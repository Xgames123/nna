use std::borrow::Cow;

use super::tokenizer::Tokenizer;
use super::{IntoAsmError, Located, Location};
use libnna::OpCode;
use libnna::{u2, u4, ArgOpTy};

type Result<T> = std::result::Result<Located<T>, Located<LexError>>;

#[derive(Debug, Clone)]
pub enum ValueToken4 {
    LabelRef(Box<str>),
    Const(u4),
}
#[derive(Debug, Clone)]
pub enum OpToken {
    Full(u8),
    LabelRef(u4, Box<str>),
}

#[derive(Debug, Clone)]
pub enum Token {
    LabelDef(Box<str>),
    Org(u8),
    Value(ValueToken4),
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

fn parse_hex4<'a>(str: &'a str) -> Option<u4> {
    let str = str.to_lowercase();
    if str.len() != 1 {
        return None;
    }
    for char in str.chars() {
        return char.to_digit(16).map(|val| u4::from_u32(val));
    }
    return None;
}
fn parse_hex8<'a>(str: &'a str) -> Option<u8> {
    let str = str.to_lowercase();
    if str.len() != 2 {
        return None;
    }

    u8::from_str_radix(&str, 16).ok()
}
fn parse_identifier(str: &str) -> Option<Box<str>> {
    for char in str.chars() {
        if !char.is_alphabetic() && char != '_' {
            return None;
        }
    }
    Some(str[1..].into())
}

fn parse_hex2(str: &str) -> Option<u2> {
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

fn parse_value4(
    token: &str,
    location: Location,
) -> std::result::Result<Option<Located<ValueToken4>>, Located<LexError>> {
    if token.starts_with("0x") {
        let value = parse_hex4(&token[2..]).ok_or(LexError::static_located(
            "Invalid 4 bit hex literal",
            location.clone(),
        ))?;
        return Ok(Some(Located::new(ValueToken4::Const(value), location)));
    }

    if token.starts_with("&") {
        let value = parse_identifier(&token[1..]).ok_or(LexError::static_located(
            "Label ref contains invalid characters.",
            location.clone(),
        ))?;
        return Ok(Some(Located::new(ValueToken4::LabelRef(value), location)));
    }

    Ok(None)
}

fn parse_next_hex8(parser: &mut Tokenizer) -> Result<u8> {
    let token = parser.next_same_line_or_err(Cow::Borrowed(
        "Expected an 8 bit constant value after this.",
    ))?;
    let value = parse_hex8(token).ok_or(LexError::static_located(
        "Expected an 8 bit constant value.",
        parser.location(),
    ))?;

    Ok(Located::new(value, parser.location()))
}

fn parse_next_value4(parser: &mut Tokenizer) -> Result<ValueToken4> {
    let token =
        parser.next_same_line_or_err(Cow::Borrowed("Expected a 4 bit value after this."))?;
    match parse_value4(token, parser.location())? {
        Some(v) => Ok(v),
        None => Err(LexError::static_located(
            "Expected a 4 bit value.",
            parser.location(),
        )),
    }
}
fn parse_next_value2(parser: &mut Tokenizer) -> Result<u2> {
    let token =
        parser.next_same_line_or_err(Cow::Borrowed("Expected a 2 bit value after this."))?;
    let value = parse_hex2(token).ok_or(LexError::static_located(
        "Expected an 2 bit value.",
        parser.location(),
    ))?;
    Ok(Located::new(value, parser.location()))
}
fn parse_next_reg(parser: &mut Tokenizer) -> Result<u2> {
    let token = parser.next_same_line_or_err(Cow::Borrowed("Expected a register after this."))?;
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

fn parse_compiler_directive<'a>(token: &'a str, parser: &mut Tokenizer) -> Result<Token> {
    match token {
        "org" => {
            let addr = parse_next_hex8(parser)?;
            return Ok(Located::new(
                Token::Org(addr.value),
                parser.location().combine(addr.location),
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

fn parse_op<'a>(token: &'a str, parser: &mut Tokenizer) -> Result<OpToken> {
    let op = OpCode::try_from_str(token).ok_or(LexError::static_located(
        "Unknown operation. See spec for available operations",
        parser.location(),
    ))?;
    let loc = parser.location();
    Ok(match op.arg_types() {
        ArgOpTy::None(bits) => Located::new(
            OpToken::Full(op.opcode().into_high() | bits.into_low()),
            loc,
        ),
        ArgOpTy::Bit4(_arg_name) => {
            let value = parse_next_value4(parser)?;
            let token = match value.value {
                ValueToken4::Const(value) => {
                    OpToken::Full(op.opcode().into_high() | value.into_low())
                }
                ValueToken4::LabelRef(name) => OpToken::LabelRef(op.opcode(), name),
            };
            Located::new(token, loc.combine(value.location))
        }
        ArgOpTy::OneReg(_arg_name, bits) => {
            let register = parse_next_reg(parser)?;

            Located::new(
                OpToken::Full(
                    op.opcode().into_high() | (register.value.into_low() << 2) | bits.into_low(),
                ),
                loc.combine(register.location),
            )
        }
        ArgOpTy::TowReg(_arg0_name, _arg1_name) => {
            let register0 = parse_next_reg(parser)?;
            let register1 = parse_next_reg(parser)?;

            Located::new(
                OpToken::Full(
                    op.opcode().into_high()
                        | register0.value.into_low() << 2
                        | register1.value.into_low(),
                ),
                loc.combine(register0.location).combine(register1.location),
            )
        }
    })
}

pub fn parse_lex(input: &str) -> std::result::Result<Vec<Located<Token>>, Located<LexError>> {
    let mut out_vec = Vec::new();
    let Some(mut parser) = Tokenizer::new(input) else {
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
                    .map(|label| Located::new(Token::LabelDef(label), parser.location()))
                    .ok_or(LexError::static_located(
                        "invalid label name",
                        parser.location(),
                    ))?,
            );
            continue;
        }

        if let Some(value) = parse_value4(token, parser.location())? {
            out_vec.push(value.map(|vt| Token::Value(vt)));
            continue;
        }

        out_vec.push(parse_op(token, &mut parser)?.map(|opt| Token::Op(opt)));
    }
}
