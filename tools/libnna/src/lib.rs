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
    None(u4),
    OneReg(&'static str, u2),
    TowReg(&'static str, &'static str),
    Bit4(&'static str),
}
impl Display for ArgOpTy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None(_) => {}
            Self::OneReg(name, _) => {
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
    (($desc:literal:reg),$arg:expr) => {
        ArgOpTy::OneReg($desc, u2::from_low($arg))
    };
    (($desc1:literal:reg, $desc2:literal:reg),$arg:expr) => {
        ArgOpTy::TowReg($desc1, $desc2)
    };
    (($desc:literal:4bit),$arg:expr) => {
        ArgOpTy::Bit4($desc)
    };
    ((),$arg:expr) => {
        ArgOpTy::None(u4::from_low($arg))
    };
}

macro_rules! ops {
    ($vis:vis $name:ident{$($opname:literal:$opcode:literal$arg:tt),*}) => {
        $vis struct $name(u8);
        impl $name{
            pub fn opcode(&self) -> u4{
                u4::from_high(self.0)
            }
            pub fn arg_types(&self) -> ArgOpTy{
                match self.0{
                    $($opcode => (opargs!($arg,self.0))),*,
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
}
