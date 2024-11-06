use std::borrow::Cow;
use std::cell::RefCell;
use std::ops::Range;
use std::rc::Rc;

use super::codeparser::CodeParser;
use super::{AsmError, Located, Location};
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

pub fn parse_hex4<'a>(str: &'a str) -> Option<u4> {
    let str = str.to_lowercase();
    if str.len() != 1 {
        return None;
    }
    for char in str.chars() {
        return char.to_digit(16).map(|val| u4::from_u32(val));
    }
    return None;
}
pub fn parse_hex8<'a>(str: &'a str) -> Option<u8> {
    let str = str.to_lowercase();
    if str.len() != 2 {
        return None;
    }

    u8::from_str_radix(&str, 16).ok()
}
pub fn parse_identifier(str: &str) -> Option<Box<str>> {
    for char in str.chars() {
        if !char.is_alphabetic() && char != '_' {
            return None;
        }
    }
    Some(str[1..].into())
}

pub fn parse_hex2(str: &str) -> Option<u2> {
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

pub fn parse_value4(
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

pub fn parse_next_hex8(parser: &mut CodeParser) -> Result<u8> {
    let token = parser.next_same_line_or_err(Cow::Borrowed(
        "Expected an 8 bit constant value after this.",
    ))?;
    let value = parse_hex8(token).ok_or(LexError::static_located(
        "Expected an 8 bit constant value.",
        parser.location(),
    ))?;

    Ok(Located::new(value, parser.location()))
}

pub fn parse_next_value4(parser: &mut CodeParser) -> Result<ValueToken4> {
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
pub fn parse_next_value2(parser: &mut CodeParser) -> Result<u2> {
    let token =
        parser.next_same_line_or_err(Cow::Borrowed("Expected a 2 bit value after this."))?;
    let value = parse_hex2(token).ok_or(LexError::static_located(
        "Expected an 2 bit value.",
        parser.location(),
    ))?;
    Ok(Located::new(value, parser.location()))
}
pub fn parse_next_reg(parser: &mut CodeParser) -> Result<u2> {
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

fn parse_compiler_directive<'a>(token: &'a str, parser: &mut CodeParser) -> Result<Token> {
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

fn parse_op<'a>(token: &'a str, parser: &mut CodeParser) -> Result<OpToken> {
    let op = OpCode::try_from_str(token).ok_or(LexError::static_located(
        "Unknown operation",
        parser.location(),
    ))?;
    Ok(match op.arg_types() {
        ArgOpTy::None() => Located::new(OpToken::Full(op.opcode()), parser.location()),
        ArgOpTy::Bit2(arg_name) => {
            let value = parse_next_value2(parser)?;
            Located::new(
                OpToken::Full(op.opcode() | value.value.into_low()),
                parser.location().combine(value.location),
            )
        }
        ArgOpTy::Bit4(arg_name) => {
            let value = parse_next_value4(parser)?;
            let token = match value.value {
                ValueToken4::Const(value) => OpToken::Full(op.opcode() | value.into_low()),
                ValueToken4::LabelRef(name) => OpToken::LabelRef(u4::from_high(op.opcode()), name),
            };
            Located::new(token, parser.location().combine(value.location))
        }
        ArgOpTy::OneReg(arg_name) => {
            let register = parse_next_reg(parser)?;

            Located::new(
                OpToken::Full(op.opcode() | register.value.into_low()),
                parser.location().combine(register.location),
            )
        }
        ArgOpTy::TowReg(arg0_name, arg1_name) => {
            let register0 = parse_next_reg(parser)?;
            let register1 = parse_next_reg(parser)?;

            Located::new(
                OpToken::Full(
                    op.opcode() | register0.value.into_low() << 2 | register1.value.into_low(),
                ),
                parser
                    .location()
                    .combine(register0.location)
                    .combine(register1.location),
            )
        }
    })
}

pub fn parse_lex(input: &str) -> std::result::Result<Vec<Located<Token>>, Located<LexError>> {
    let mut out_vec = Vec::new();
    let Some(mut parser) = CodeParser::new(input) else {
        return Ok(out_vec);
    };

    loop {
        let Some(token) = parser.next() else {
            return Ok(out_vec);
        };
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

    // for (linenum, line) in input.lines().enumerate() {
    //     let linenum = linenum + 1;
    //
    //     // strip comments
    //     if let Some(pos) = line.find('#') {
    //         line = &line[..pos];
    //     }
    //     if let Some(pos) = line.find(';') {
    //         line = &line[..pos];
    //     }
    //
    //     let mut iter = line.split_whitespace();
    //     loop{
    //         let Some(token) = iter.next() else {break;};
    //         if token == ".org" {
    //             let token = iter.next().ok_or(AsmError{
    //                 linenum: linenum,
    //             })?;
    //             push_tok!(
    //                 Token::Org(parse_hex8(&token).ok_or_else(|| AsmError {
    //                     linenum: Some(linenum),
    //                     code_snip: token.into(),
    //                     message: "Failed to parse hex digit".into(),
    //                     stage: Stage::Parse,
    //                 })?)
    //             );
    //             continue;
    //         }
    //
    //     }
    //
    //     for token in line.split_whitespace() {
    //         if org {
    //             continue;
    //         }
    //
    //         if token.ends_with(":") {
    //             push_tok!(Token::LabelDef(token[..token.len() - 1].into()), linenum);
    //             continue;
    //         }
    //
    //         if token.starts_with("&&") {
    //             push_tok!(
    //                 Token::LabelRef {
    //                     name: token[2..].into(),
    //                     wide: true
    //                 },
    //                 linenum
    //             );
    //             continue;
    //         }
    //         if token.starts_with("&") {
    //             push_tok!(
    //                 Token::LabelRef {
    //                     name: token[1..].into(),
    //                     wide: false
    //                 },
    //                 linenum
    //             );
    //             continue;
    //         }
    //         if token.starts_with("0x") {
    //             push_tok!(
    //                 Token::Const(parse_hex4(&token[2..]).ok_or_else(|| AsmError {
    //                     linenum: Some(linenum),
    //                     code_snip: token.into(),
    //                     message: "Failed to parse hex digit".into(),
    //                     stage: Stage::Parse,
    //                 })?),
    //                 linenum
    //             );
    //             continue;
    //         }
    //
    //         match Op::try_from_str(&token) {
    //             Some(op) => {
    //                 push_tok!(Token::Const(u4::from_low(op.opcode())), linenum);
    //
    //                 let arg = op.arg_types().0;
    //                 match arg {
    //                     ArgTy::Any(),
    //                     ArgTy::Reg(),
    //                     ArgTy::None(),
    //                 }
    //             }
    //             None => {
    //                 return Err(AsmError {
    //                     linenum: Some(linenum),
    //                     code_snip: token.into(),
    //                     message: "Invalid operation".into(),
    //                     stage: Stage::Parse,
    //                 })
    //             }
    //         }
    //     }
    // }
}
