use crate::instruction_set;

#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[derive(Copy, Clone)]
pub enum InstructionSet {
    Nna8v1,
}

instruction_set! {
    pub Nna8v1 {
        "nop"(0x00)(),
        "brk"(0x04)(),
        "flf"(0x08)(),
        "clf"(0x0C)(),
        "jmp"(0x01)("addr":reg),
        "inc"(0x02)("reg":reg),
        "dec"(0x03)("reg":reg),

        "lil"(0x10)("value":4bit),
        "lih"(0x20)("value":4bit),
        "mwr"(0x30)("reg":reg,"addr":reg),
        "mrd"(0x40)("reg":reg,"addr":reg),
        "mov"(0x50)("dest":reg, "source":reg),
        "bra"(0x60)("addr":4bit),
        "rol"(0x70)("a":reg, "b":reg),
        "eq" (0x80)("a":reg, "b":reg),
        "gt" (0x90)("a":reg, "b":reg),
        "add"(0xA0)("source":reg, "a":reg),
        "mul"(0xB0)("source":reg, "a":reg),
        "and"(0xC0)("source":reg, "a":reg),
        "not"(0xD0)("a":reg, "b":reg),
        "or" (0xE0)("source":reg, "a":reg),
        "xor"(0xF0)("source":reg, "a":reg)
    }
}
