# Memory
Memory is divided into 16 banks that are each 16 nibbles big.

| addr range | function |
|------------|----------|
| 00 -> EF   | ram rw   |
| F0 -> FF   | io bank  |

# IO Bank
The IO bank contains memory mapped peripherals.
Which peripherals are available depends on the [chip variant](chip_variants.md)

## Boot Dev
All nna chips have the boot device at address 0xF0
Reading/writing from the first 4 bits reads/writes them to the device at the address pointed to by the 16 bits following

| offset |  size   |  name   |
|--------|---------|---------|
| 0 bits | 4 bits  | data    |
| 4 bits | 16 bits | address |

# Boot Sequence
1. The first 16 nibs are copied from the boot device
2. All registers and flags are reset to 0
3. Execution begins at 0x00

# Flags
There is 1 flag (overflow flag) that is set by some instructions when they overflow and used by jump instructions to conditionally jump.

# Registers
> [!NOTE]
> All registers including pc are reset to 0 when the device boots up.

> **op access**
> The register can be accessed in operations. Ex. mov r0 r2
>

| name | size | description                                                | op access |
|------|------|------------------------------------------------------------|-----------|
| r0   | 4    | General purpose, Memory reads and writes use this register | yes       |
| r1   | 4    | General purpose                                            | yes       |
| r2   | 4    | General purpose                                            | yes       |
| r3   | 4    | General purpose,                                           | yes       |
| rb   | 4    | Used as bank for read and write ops                        | no (srb)  |
| pc   | 8    | Program counter                                            | no        |

# Instructions
Instructions are 1 byte where the first 4 bits are the opcode followed by 2 arguments each 2 bits.
Parameters that take a register are noted using: [param_description].
| name       | opcode | arg0      | arg1       | description                                                                   |
|------------|--------|-----------|------------|-------------------------------------------------------------------------------|
| nop        | 0x0    | 00        | 00         | Does nothing.                                                                 |
| brk        | 0x0    | 01        | 00         | Break the debugger.                                                           |
| flf        | 0x0    | 10        | 00         | Flips flag (if flag was set reset else set)                                   |
| clf        | 0x0    | 11        | 00         | Clear flag                                                                    |
| shl        | 0x0    | [reg]     | 01         | Shift reg left by 1 into the overflow flag and stores the result in [reg].    |
| shr        | 0x0    | [reg]     | 10         | Shift reg right by 1 into the overflow flag and stores the result in [reg].   |
| srb        | 0x0    | [reg]     | 11         | Store the value of [reg] in rb                                                |
| lim        | 0x1    | value_low | value_high | Loads the immediate value into r0.                                            |
| mwr        | 0x2    | [reg]     | [addr]     | Writes [reg] to memory at [addr]. (uses rb as bank select)                    |
| mrd        | 0x3    | [reg]     | [addr]     | Reads the value at memory address [addr] into [reg]. (uses rb as bank select) |
| mov        | 0x4    | [dest]    | [source]   | Copies (moves) the value from [source] into [dest].                           |
| bra        | 0x5    | addr_low  | addr_high  | Branch to addr when the overflow flag is set.                                 |
| jmp        | 0x6    | [addr]    | [bank]     | Jumps to [addr] on bank [bank] when the overflow flag is set                  |
| eq         | 0x7    | [a]       | [b]        | Sets the overflow flag when [a] == [b]                                        |
| gt         | 0x8    | [a]       | [b]        | Sets the overflow flag when [a] > [b]                                         |
| add        | 0x9    | [a]       | [b]        | Adds [a] to the [b] and stores it to [a]. (Sets the overflow flag)            |
| mul        | 0xA    | [a]       | [b]        | Multiplies [a] with [b] and store the result in [a]. (Sets the overflow flag) |
| and        | 0xB    | [a]       | [b]        | and's [a] and [b] and stores the result in [a]                                |
| nand       | 0xC    | [a]       | [b]        | nand's [a] and [b] and stores the result in [a]                               |
| or         | 0xD    | [a]       | [b]        | or's [a] and [b] and stores the result in [a].                                |
| xor        | 0xE    | [a]       | [b]        | xor's [a] and [b] and stores the result in [a].                               |
| unassigned | 0xF    | [reg]     | [reg]      |                                                                               |


