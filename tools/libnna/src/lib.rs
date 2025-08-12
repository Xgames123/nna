mod extra_number_traits;
mod usmol;

pub use extra_number_traits::*;
pub use usmol::*;

pub mod instruction_sets;
pub use instruction_sets::Architecture;

mod opargs;
pub use opargs::*;

pub trait Arch: Sized + Copy {
    const BANKS: bool = false;

    fn try_from_str(str: &str) -> Option<Self>;
    fn args(self) -> OpArgs;
    fn name(self) -> &'static str;
}

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
