mod extra_number_traits;
mod usmol;
use std::fmt::Display;

pub use extra_number_traits::*;
pub use usmol::*;

pub mod instruction_sets;
pub use instruction_sets::InstructionSet;

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

pub trait ISet: Sized + Copy {
    fn try_from_str(str: &str) -> Option<Self>;
    fn args(self) -> OpArgs;
    fn name(self) -> &'static str;
}

macro_rules! opargs_impl {
    (($desc:literal:reg)) => {
        crate::OpArgs::OneReg($desc)
    };
    (($desc1:literal:reg, $desc2:literal:reg)) => {
        crate::OpArgs::TowReg($desc1, $desc2)
    };
    (($desc:literal:4bit)) => {
        crate::OpArgs::Bit4($desc)
    };
    (()) => {
        crate::OpArgs::None
    };
}
pub(crate) use opargs_impl as opargs;

macro_rules! instruction_set_impl {
    ($vis:vis $name:ident {$($opname:literal($opcode:literal)$opargs:tt),*}) => {
        #[derive(Copy, Clone)]
        $vis struct $name(u8);
        impl crate::ISet for $name {
            fn try_from_str(str: &str) -> Option<Self> {
                if str.starts_with("?"){
                    return None;
                }
                match str {
                    $(
                        $opname => Some(Self($opcode)),
                    )*
                    _=>{None}
                }
            }

            fn args(self) -> crate::OpArgs {
                match self.0 {
                    $(
                        $opcode => crate::opargs!($opargs),
                    )*
                    _ => unreachable!(),
                }
            }
            fn name(self) -> &'static str {
                    match self.0 {
                        $(
                            $opcode => $opname,
                        )*
                        _ => unreachable!(),
                    }
            }

        }

        impl $name {
            pub fn into_u8(self) -> u8 {
                self.0
            }
        }
        impl Into<u8> for $name {
            fn into(self) -> u8 {
                self.0
            }
        }
        impl std::ops::Deref for $name {
            type Target=u8;
            fn deref(&self) -> &u8 {
                &self.0
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                use crate::ISet;
                f.write_str(self.name())?;
                f.write_str(" ")?;
                self.args().fmt(f)
            }
        }

    };
}
pub(crate) use instruction_set_impl as instruction_set;
