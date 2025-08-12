use crate::{instruction_set, ConstOpArg};

#[derive(Clone, Copy)]
pub struct Reg;
impl ConstOpArg for Reg {
    const NAME: &'static str = "reg";
    const VARIANTS: &'static [&'static str] = &["r0", "r1", "r2", "r3"];
}
pub struct CalOp;
impl ConstOpArg for CalOp {
    const NAME: &'static str = "co";
    const VARIANTS: &'static [&'static str] = &[
        "add", "sub", "mul", "div", "shl", "shr", "rol", "ror", "and", "or",
    ];
}

#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Architecture {
    Nna8v1,
    Nna8v2,
}
impl Architecture {
    pub fn from_str(str: &str) -> Option<Self> {
        match str {
            "nna8v1" => Some(Self::Nna8v1),
            "nna8v2" => Some(Self::Nna8v2),
            _ => None,
        }
    }
    ///How many bytes of memory the arch can address
    pub fn addressable_size(self) -> usize {
        match self {
            Self::Nna8v1 => 256,
            Self::Nna8v2 => 65536,
        }
    }
}

instruction_set! {
    pub Nna8v1 {
        // zero
            "nop"(0x00)(),
            "brk"(0x04)(),
            "flf"(0x08)(),
            "clf"(0x0C)(),
            "jmp"(0x01)("addr":Reg,()),
            "inc"(0x02)("reg":Reg,()),
            "dec"(0x03)("reg":Reg,()),
        "lil"(0x10)("value":bit4),
        "lih"(0x20)("value":bit4),
        "mwr"(0x30)("reg":Reg,"addr":Reg),
        "mrd"(0x40)("reg":Reg,"addr":Reg),
        "mov"(0x50)("dest":Reg, "source":Reg),
        "bra"(0x60)("addr":bit4),
        "rol"(0x70)("a":Reg, "b":Reg),
        "eq" (0x80)("a":Reg, "b":Reg),
        "gt" (0x90)("a":Reg, "b":Reg),
        "add"(0xA0)("source":Reg, "a":Reg),
        "mul"(0xB0)("source":Reg, "a":Reg),
        "and"(0xC0)("source":Reg, "a":Reg),
        "not"(0xD0)("a":Reg, "b":Reg),
        "or" (0xE0)("source":Reg, "a":Reg),
        "xor"(0xF0)("source":Reg, "a":Reg)
    }

}
instruction_set! {
    pub Nna8v2 [banks] {
        //sin
            "nop"(0x00)(),
            "brk"(0x04)(),
            //? 0x08
            //? 0x0C
            "jmp"(0x01)("addr":Reg,()),
            "mpb"(0x02)("bank":Reg,()),
            "mdb"(0x03)("bank":Reg,()),
        "eq"(0x10)("a":Reg,"b":Reg),
        "gt"(0x20)("a":Reg,"b":Reg),
        //flg
            "flf"(0x30)(), // 00
            "sef"(0x3C)(), // 11
            "clf"(0x34)(), // 01
        //? 0x4
        "bra"(0x50)("addr":bit4),
        "mco"(0x60)("co":CalOp),
        "mwr"(0x70)("reg":Reg,"addr":Reg),
        "mrd"(0x80)("reg":Reg,"addr":Reg),
        "lil"(0x90)("val":bit4),
        "lih"(0xA0)("val":bit4),
        "mov"(0xB0)("dest":Reg,"src":Reg),
        "cal"(0xC0)("a":Reg,"b":Reg),
        "xor"(0xD0)("a":Reg,"b":Reg),
        "inc"(0xE0)("reg":Reg,"amount":bit2nz),
        "dec"(0xF0)("reg":Reg,"amount":bit2nz)

    }
}
