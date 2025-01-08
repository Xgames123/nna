# nna8v0

# Memory
The total address range is 256 bytes.
But the last 16 bytes are reserved for IO.

| addr range | function |
|------------|----------|
| 00 -> EF   | ram rw   |
| F0 -> FF   | io bank  |

# IO memory range
The IO memory range contains memory mapped peripherals.
Which peripherals are available depends on the [chip variant](chip_variants.md)

## Boot Dev
All nna chips have the boot device at address 0xF0
Reading/writing from the first byte reads/writes them to the device at the address pointed to by the 2 following bytes

| size (bytes) | name                                                                                          |
|--------------|-----------------------------------------------------------------------------------------------|
| 1            | data to write to the device (when chip reads/writes here the action is started on the device) |
| 2            | address to read/write from                                                                    |

# Boot Sequence
1. The first 16 bytes are copied from the boot device
2. All registers and flags are reset to 0
3. Execution begins at 0x00

# Flags
There is 1 flag (overflow flag) that is set by some instructions when they overflow and used by branching instructions to conditionally branch.

# Registers
> [!NOTE]
> All registers including pc are reset to 0 when the device boots up.

> **op access**
> The register can be accessed in operations. Ex. mov r0 r2
>

| name | size | description                                                | op access |
|------|------|------------------------------------------------------------|-----------|
| r0   | 8    | General purpose, Memory reads and writes use this register | yes       |
| r1   | 8    | General purpose                                            | yes       |
| r2   | 8    | General purpose                                            | yes       |
| r3   | 8    | General purpose,                                           | yes       |
| pc   | 8    | Program counter                                            | no        |

# Instructions
Instructions are 1 byte where the first 4 bits are the opcode followed by 2 arguments each 2 bits.
Parameters that take a register are noted using: [description].

| name       | opcode |    arg0    |    arg1     | description                                                                      |
|------------|--------|:----------:|:-----------:|----------------------------------------------------------------------------------|
| nop        | 0x0    |     00     |     00      | Does nothing.                                                                    |
| brk        | 0x0    |     01     |     00      | Break the debugger.                                                              |
| flf        | 0x0    |     10     |     00      | Flips flag (if flag was set reset else set)                                      |
| clf        | 0x0    |     11     |     00      | Clear flag                                                                       |
| shl        | 0x0    |   [reg]    |     01      | Shift reg left by 1 into the overflow flag and stores the result in [reg].       |
| shr        | 0x0    |   [reg]    |     10      | Shift reg right by 1 into the overflow flag and stores the result in [reg].      |
| unassigned | 0x0    |   [reg]    |     11      |                                                                                  |
| lil        | 0x1    |   value    |    value    | Loads the immediate value into the low part of r0.                               |
| lih        | 0x2    |   value    |    value    | Loads the immediate value into the high part of r0.                              |
| mwr        | 0x3    |   [reg]    |   [addr]    | Writes [reg] to memory at [addr]. (uses rb as bank select)                       |
| mrd        | 0x4    |   [reg]    |   [addr]    | Reads the value at memory address [addr] into [reg]. (uses rb as bank select)    |
| mov        | 0x5    |   [dest]   |  [source]   | Copies (moves) the value from [source] into [dest].                              |
| bra        | 0x6    |    addr    |    addr     | Branch to addr when the overflow flag is set.                                    |
| jmp        | 0x7    | [addr low] | [addr high] | Do a long jump to the address [addrlow] [addrhigh] when the overflow flag is set |
| eq         | 0x8    |    [a]     |     [b]     | Sets the overflow flag when [a] == [b]                                           |
| gt         | 0x9    |    [a]     |     [b]     | Sets the overflow flag when [a] > [b]                                            |
| add        | 0xa    |    [a]     |     [b]     | Adds [a] to the [b] and stores it to [a]. (Sets the overflow flag)               |
| mul        | 0xB    |    [a]     |     [b]     | Multiplies [a] with [b] and store the result in [a]. (Sets the overflow flag)    |
| and        | 0xC    |    [a]     |     [b]     | and's [a] and [b] and stores the result in [a]                                   |
| nand       | 0xD    |    [a]     |     [b]     | nand's [a] and [b] and stores the result in [a]                                  |
| or         | 0xE    |    [a]     |     [b]     | or's [a] and [b] and stores the result in [a].                                   |
| xor        | 0xF    |    [a]     |     [b]     | xor's [a] and [b] and stores the result in [a].                                  |


