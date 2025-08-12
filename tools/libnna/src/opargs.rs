use std::fmt::Display;

pub trait ConstOpArg {
    const NAME: &'static str;
    const VARIANTS: &'static [&'static str];
}

#[derive(Clone, Copy, Debug)]
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

#[derive(Clone, Copy)]
pub struct OpArg {
    pub desc: &'static str,
    pub ty: OpArgType,
}
impl OpArg {
    pub fn big(self) -> BigOpArg {
        BigOpArg(self)
    }
}
#[derive(Clone, Copy)]
pub enum OpArgType {
    None,
    Value { nz: bool },
    Const(ConstArg),
}

fn fmt_value(
    desc: &'static str,
    big: bool,
    nz: bool,
    f: &mut std::fmt::Formatter<'_>,
) -> std::fmt::Result {
    f.write_str("{")?;
    f.write_str(desc)?;
    f.write_str(":")?;
    if big {
        f.write_str("4")?;
    } else {
        f.write_str("2")?;
    }
    f.write_str("bit")?;
    if nz {
        f.write_str("nz")?;
    }
    f.write_str("}")?;
    Ok(())
}
impl OpArg {
    fn fmt_oparg(&self, big: bool, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.ty {
            OpArgType::None => {}
            OpArgType::Value { nz } => {
                fmt_value(self.desc, big, nz, f)?;
            }
            OpArgType::Const(c) => {
                f.write_str("[")?;
                f.write_str(self.desc)?;
                f.write_str(":")?;
                f.write_str(c.name)?;
                f.write_str("]")?;
            }
        }
        Ok(())
    }
}
impl Display for OpArg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.fmt_oparg(false, f)
    }
}

#[derive(Clone, Copy)]
pub struct BigOpArg(pub OpArg);
impl BigOpArg {
    pub fn big(self) -> BigOpArg {
        self
    }
}
impl Display for BigOpArg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt_oparg(true, f)
    }
}

#[derive(Clone, Copy)]
pub enum OpArgs {
    Arg(BigOpArg),
    ArgArg(OpArg, OpArg),
}
impl Display for OpArgs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Arg(a) => a.fmt(f)?,
            Self::ArgArg(a, b) => {
                a.fmt(f)?;
                f.write_str(" ")?;
                b.fmt(f)?;
            }
        }
        Ok(())
    }
}
macro_rules! oparg_impl {
    (()) => {
        crate::OpArg {
            ty: crate::OpArgType::None,
            desc: "",
        }
    };

    ($desc:literal:bit2) => {
        crate::OpArg {
            ty: crate::OpArgType::Value { nz: false },
            desc: $desc,
        }
    };
    ($desc:literal:bit2nz) => {
        crate::OpArg {
            ty: crate::OpArgType::Value { nz: true },
            desc: $desc,
        }
    };
    ($desc:literal:bit4) => {
        crate::BigOpArg(crate::OpArg {
            ty: crate::OpArgType::Value { nz: false },
            desc: $desc,
        })
    };

    ($desc:literal:$const:ty) => {
        crate::OpArg {
            ty: crate::OpArgType::Const(crate::ConstArg::new::<$const>()),
            desc: $desc,
        }
    };
}
pub(crate) use oparg_impl as oparg;

macro_rules! opargs_impl {
    (()) => {
        crate::OpArgs::Arg(
            crate::BigOpArg(
                crate::oparg!(())
            )
        )
    };
    (($desc:literal:$ty:ident)) => {
        crate::OpArgs::Arg(crate::oparg!($desc:$ty).big())
    };
    (($desc:literal:$ty:ident,())) => {
        crate::OpArgs::ArgArg(crate::oparg!($desc:$ty), crate::oparg!(()))
    };
    (((),$desc:literal:$ty:ident)) => {
        crate::OpArgs::ArgArg(crate::oparg!($desc:$ty), crate::oparg!(()))
    };

    (($desc0:literal:$ty0:ident,$desc1:literal:$ty1:ident)) => {
        crate::OpArgs::ArgArg(crate::oparg!($desc0:$ty0), crate::oparg!($desc1:$ty1))
    };
}
pub(crate) use opargs_impl as opargs;
