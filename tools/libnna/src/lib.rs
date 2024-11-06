mod usmol;
use std::fmt::Display;

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
pub enum ArgOpTy {
    None(),
    OneReg(&'static str),
    TowReg(&'static str, &'static str),
    Bit2(&'static str),
    Bit4(&'static str),
}
impl Display for ArgOpTy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None() => {}
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
            Self::Bit2(name) => {
                f.write_str(*name)?;
                f.write_str(":2bit")?;
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
        ArgOpTy::OneReg($desc)
    };
    (($desc1:literal:reg, $desc2:literal:reg)) => {
        ArgOpTy::TowReg($desc1, $desc2)
    };
    (($desc:literal:4bit)) => {
        ArgOpTy::Bit4($desc)
    };
    (($desc:literal:2bit)) => {
        ArgOpTy::Bit2($desc)
    };
    (()) => {
        ArgOpTy::None()
    };
}

macro_rules! ops {
    ($vis:vis $name:ident{$($opname:literal:$opcode:literal$arg:tt),*}) => {
        $vis struct $name(u8);
        impl $name{
            pub fn opcode(&self) -> u8{
                self.0
            }
            pub fn arg_types(&self) -> ArgOpTy{
                match self.0{
                    $($opcode => (opargs!($arg))),*,
                    _=>unreachable!(),
                }

            }
            pub fn try_from_str(string: &str) -> Option<Self>{
                match string{
                    $($opname => Some($name($opcode))),*,
                    _=>None,
                }
            }
            pub fn opname(&self) -> &'static str{
                match self.0{
                    $($opcode => $opname),*,
                    _=>unreachable!(),
                }
            }
        }
        impl Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let arg = self.arg_types();
                f.write_str(self.opname())?;
                f.write_str(" ")?;
                arg.fmt(f)?;
                Ok(())
            }
        }
    };
}
ops! {
    pub OpCode{
        "nop":0x00(),
        "brk":0x01(),
        "flf":0x02(),
        "clf":0x03(),
        "shl":0x04("reg":reg),
        "shr":0x08("reg":reg),

        "lim":0x10("value":4bit),
        "mew":0x20("addr":4bit),
        "mer":0x30("addr":4bit),
        "mov":0x40("source":reg, "dest":reg),
        "jms":0x50("addr":4bit),
        "jmp":0x60("addr":reg, "bank":reg),
        "eq" :0x70("a":reg, "b":reg),
        "gt" :0x80("a":reg, "b":reg),
        "add":0x90("source":reg, "a":reg),
        "mul":0xA0("source":reg, "a":reg),
        "and":0xB0("source":reg, "a":reg),
        "nand":0xC0("source":reg, "a":reg),
        "or" :0xD0("source":reg, "a":reg),
        "xor":0xE0("source":reg, "a":reg)
    }
}
