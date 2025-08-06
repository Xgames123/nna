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
        "add", "sub", "div", "mul", "shl", "shr", "rol", "ror", "and", "or",
    ];
}

#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[derive(Copy, Clone)]
pub enum Architecture {
    Nna8v1,
    Nna8v2,
}
impl Architecture {
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
            "jmp"(0x01)("addr":Reg),
            "inc"(0x02)("reg":Reg),
            "dec"(0x03)("reg":Reg),
        "lil"(0x10)("value":4bit),
        "lih"(0x20)("value":4bit),
        "mwr"(0x30)("reg":Reg,"addr":Reg),
        "mrd"(0x40)("reg":Reg,"addr":Reg),
        "mov"(0x50)("dest":Reg, "source":Reg),
        "bra"(0x60)("addr":4bit),
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
            "jmp"(0x01)("addr":Reg),
            "mpb"(0x02)("bank":Reg),
            "mdb"(0x03)("bank":Reg),
        "eq"(0x10)("a":Reg,"b":Reg),
        "gt"(0x20)("a":Reg,"b":Reg),
        //flg
            "flf"(0x30)(),
            "sef"(0x3C)(),
            "clf"(0x34)(),
        //? 0x4
        "bra"(0x50)("addr":4bit),
        "mco"(0x60)("co":CalOp),
        "mwr"(0x70)("reg":Reg,"addr":Reg),
        "mrd"(0x80)("reg":Reg,"addr":Reg),
        "lil"(0x90)("val":4bit),
        "lih"(0xA0)("val":4bit),
        "mov"(0xB0)("dest":Reg,"src":Reg),
        "cal"(0xC0)("a":Reg,"b":Reg),
        "xor"(0xD0)("a":Reg,"b":Reg),
        "inc"(0xE0)("reg":Reg,"amount":2bit),
        "dec"(0xF0)("reg":Reg,"amount":2bit)

    }
}
