mod usmol;
use std::{fmt::Display, marker::PhantomData, ops::Deref};

pub use usmol::*;

pub enum Reg {
    R0,
    R1,
    R2,
    R3,
}
impl Reg {
    pub fn code(&self) -> u2 {
        match self {
            Self::R0 => u2!(0b00),
            Self::R1 => u2!(0b01),
            Self::R2 => u2!(0b10),
            Self::R3 => u2!(0b11),
        }
    }
}

///Argument type of an operation
pub enum OpArgs {
    ///No arguments
    None,
    /// 1 register as arguments
    OneReg(&'static str),
    /// 2 registers as arguments
    TowReg(&'static str, &'static str),
    /// 1 argument that is a 4 bit value
    Bit4(&'static str),
}
impl Display for OpArgs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => {}
            Self::OneReg(name) => {
                f.write_str("[")?;
                f.write_str(name)?;
                f.write_str("]")?
            }
            Self::TowReg(name1, name2) => {
                f.write_str("[")?;
                f.write_str(name1)?;
                f.write_str("]")?;
                f.write_str(" ")?;
                f.write_str("[")?;
                f.write_str(name2)?;
                f.write_str("]")?
            }
            Self::Bit4(name) => {
                f.write_str(*name)?;
                f.write_str(":4bit")?;
            }
        }
        Ok(())
    }
}

macro_rules! opargs {
    (($desc:literal:reg)) => {
        OpArgs::OneReg($desc)
    };
    (($desc1:literal:reg, $desc2:literal:reg)) => {
        OpArgs::TowReg($desc1, $desc2)
    };
    (($desc:literal:4bit)) => {
        OpArgs::Bit4($desc)
    };
    (()) => {
        OpArgs::None
    };
}

macro_rules! instruction_sets {
    ($($iset:ident{$($name:literal:$opcode:literal$args:tt),*})*) => {
        #[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
        #[derive(Copy, Clone)]
        pub enum InstructionSet {
            $($iset),*
        }

        #[derive(Clone, Copy)]
        pub struct Op(u8, InstructionSet);

        impl Op {
            pub fn try_from_str(string: &str, iset: InstructionSet) -> Option<(Self,OpArgs)> {
                if string.starts_with("?"){
                    return None;
                }
                match iset {
                    $(
                        InstructionSet::$iset =>
                            match string {
                                $(
                                    $name => Some((Self($opcode, iset),opargs!($args))),
                                )*
                                _=>{None}
                            }


                    ),*
                }
            }
            pub fn args(self) -> OpArgs {
                match self.1{
                $(
                    InstructionSet::$iset =>
                    match self.0 {
                        $(
                            $opcode => opargs!($args),
                        )*
                            _=>unreachable!()
                    }

                ),*}

            }
            pub fn opname(self) -> &'static str {
                match self.1{
                $(
                    InstructionSet::$iset =>
                    match self.0 {
                        $(
                            $opcode => $name,
                        )*
                            _=>{"?"}
                    }

                ),*}
            }
        }

    };
}
impl Op {
    pub fn into_u8(self) -> u8 {
        self.0
    }
}
impl Into<u8> for Op {
    fn into(self) -> u8 {
        self.0
    }
}
impl Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.opname())?;
        f.write_str(" ")?;
        self.args().fmt(f)
    }
}

instruction_sets! {
    Nna8v1 {
        "nop":0x00(),
        "brk":0x04(),
        "flf":0x08(),
        "clf":0x0C(),
        "jmp":0x01("addr":reg),
        "inc":0x02("reg":reg),
        "dec":0x03("reg":reg),

        "lil":0x10("value":4bit),
        "lih":0x20("value":4bit),
        "mwr":0x30("reg":reg,"addr":reg),
        "mrd":0x40("reg":reg,"addr":reg),
        "mov":0x50("dest":reg, "source":reg),
        "bra":0x60("addr":4bit),
        "rol":0x70("a":reg, "b":reg),
        "eq" :0x80("a":reg, "b":reg),
        "gt" :0x90("a":reg, "b":reg),
        "add":0xA0("source":reg, "a":reg),
        "mul":0xB0("source":reg, "a":reg),
        "and":0xC0("source":reg, "a":reg),
        "nand":0xD0("source":reg, "a":reg),
        "or" :0xE0("source":reg, "a":reg),
        "xor":0xF0("source":reg, "a":reg)
    }

    Nna8v2 {
        "nop":0x00(),
        "brk":0x04(),
        "flf":0x08(),
        "clf":0x0C(),
        "jmp":0x01("addr":reg),
        "inc":0x02("reg":reg),
        "dec":0x03("reg":reg),

        "lil":0x10("value":4bit),
        "lih":0x20("value":4bit),
        "mwr":0x30("reg":reg,"addr":reg),
        "mrd":0x40("reg":reg,"addr":reg),
        "mov":0x50("dest":reg, "source":reg),
        "bra":0x60("addr":4bit),
        "add":0x70(),
        "sub":0x71(),
        "div":0x72(),
        "mul":0x73(),
        "shl":0x74(),
        "shr":0x75(),
        "rol":0x76(),
        "ror":0x77(),
        "and":0x78(),
        "or": 0x79(),
        "not":0x7A(),
        // "?": 0x7B(),
        // "?": 0x7C(),
        // "?": 0x7D(),
        // "?": 0x7E(),
        // "?": 0x7F(),
        "eq": 0x80("a":reg, "b":reg),
        "gt": 0x90("a":reg, "b":reg),
        "cal":0xA0("a":reg, "b":reg),
        // "?": 0xB0("source":reg, "a":reg),
        // "?": 0xC0("source":reg, "a":reg),
        // "?": 0xD0("source":reg, "a":reg),
        "sbs":0xE0("bank":reg),
        "xor":0xF0("source":reg, "a":reg)
    }
}
