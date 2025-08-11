mod extra_number_traits;
mod usmol;
use std::fmt::Display;

pub use extra_number_traits::*;
pub use usmol::*;

pub mod instruction_sets;
pub use instruction_sets::Architecture;

pub trait ConstOpArg {
    const NAME: &'static str;
    const VARIANTS: &'static [&'static str];
}

#[derive(Clone, Copy)]
pub struct ConstArg {
    pub name: &'static str,
    pub variants: &'static [&'static str],
}
impl ConstArg {
    pub fn new<T: ConstOpArg>() -> Self {
        Self {
            name: T::NAME,
            variants: T::VARIANTS,
        }
    }
}

///Argument type of an operation
pub enum OpArgs {
    ///No arguments
    None,
    /// 1 const arg as argument 0
    ConstNone((&'static str, ConstArg)),

    /// 1 const arg as arg 0 and a 2 bit value as arg 1
    ConstBit2((&'static str, ConstArg), &'static str),

    /// 1 const arg as arg 0 and a 2 bit value (non zero) as arg 1
    ConstBit2Nz((&'static str, ConstArg), &'static str),

    /// 2 const args as arguments
    ConstConst((&'static str, ConstArg), (&'static str, ConstArg)),
    /// 1 argument that is a 4 bit value
    Bit4(&'static str),
}
fn write_const_arg(
    f: &mut std::fmt::Formatter,
    arg: ConstArg,
    desc: &'static str,
) -> std::fmt::Result {
    f.write_str("[")?;
    f.write_str(desc)?;
    f.write_str(":")?;
    f.write_str(arg.name)?;
    f.write_str("]")
}
impl Display for OpArgs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => {}
            Self::ConstNone((desc, arg)) => write_const_arg(f, *arg, desc)?,
            Self::ConstBit2((desc0, arg0), desc1) => {
                write_const_arg(f, *arg0, desc0)?;
                f.write_str(" {")?;
                f.write_str(*desc1)?;
                f.write_str(":2bit}")?;
            }
            Self::ConstBit2Nz((desc0, arg0), desc1) => {
                write_const_arg(f, *arg0, desc0)?;
                f.write_str(" {")?;
                f.write_str(*desc1)?;
                f.write_str(":2bitnz}")?;
            }
            Self::ConstConst((desc0, arg0), (desc1, arg1)) => {
                write_const_arg(f, *arg0, desc0)?;
                f.write_str(" ")?;
                write_const_arg(f, *arg1, desc1)?;
            }
            Self::Bit4(name) => {
                f.write_str("{")?;
                f.write_str(*name)?;
                f.write_str(":4bit}")?;
            }
        }
        Ok(())
    }
}

pub trait Arch: Sized + Copy {
    const BANKS: bool = false;

    fn try_from_str(str: &str) -> Option<Self>;
    fn args(self) -> OpArgs;
    fn name(self) -> &'static str;
}

macro_rules! opargs_impl {
    (($desc:literal:$const:ty)) => {
        crate::OpArgs::ConstNone(($desc, crate::ConstArg::new::<$const>()))
    };
    (($desc1:literal:$const:ty, $desc2:literal:2bit)) => {
        crate::OpArgs::ConstBit2(($desc1, crate::ConstArg::new::<$const>()), $desc2)
    };
    (($desc1:literal:$const:ty, $desc2:literal:2bitnz)) => {
        crate::OpArgs::ConstBit2Nz(($desc1, crate::ConstArg::new::<$const>()), $desc2)
    };
    (($desc1:literal:$const1:ty, $desc2:literal:$const2:ty)) => {
        crate::OpArgs::ConstConst(
            ($desc1, crate::ConstArg::new::<$const1>()),
            ($desc2, crate::ConstArg::new::<$const2>()),
        )
    };
    (($desc:literal:4bit)) => {
        crate::OpArgs::Bit4($desc)
    };
    (()) => {
        crate::OpArgs::None
    };
}
pub(crate) use opargs_impl as opargs;

macro_rules! iset_args_impl {
    ($([$arg:ident])*) => {
        $(
            iset_args!($arg)
        )*
    };
    (banks) => {
        const BANKS: bool = true;
    };
}
pub(crate) use iset_args_impl as iset_args;

macro_rules! instruction_set_impl {
    ( $vis:vis $name:ident $([$arg:ident])*{ $($opname:literal($opcode:literal)$opargs:tt),*}) => {
        #[derive(Copy, Clone)]
        $vis struct $name(u8);
        impl crate::Arch for $name {
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
            crate::iset_args!($($arg),*);

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
                use crate::Arch;
                f.write_str(self.name())?;
                f.write_str(" ")?;
                self.args().fmt(f)
            }
        }

    };
}
pub(crate) use instruction_set_impl as instruction_set;
