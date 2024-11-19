# Memory
Memory is divided into 16 banks that are each 16 nibbles big.

> [!NOTE]
> When the processor boots it starts executing from address 0x00.

| addr range | function                |
|------------|-------------------------|
| 00 -> E0   |                         |
| E0 -> E3   | reserved for future use |
| E4 -> EF   |                         |
| F0 -> FF   | io ports p0 -> p3       |

# Ports
The last bank (F) is divided into 4 ports (p0 -> p3) each 16 pins in size.

- The first 4 nibbles of each port (pin 0 -> 7) are always used as INPUTS
- The next 4 nibbles of each port (pin 8 -> 16) are always used as OUTPUTS

| port | addr | pins     | direction |
|------|------|----------|-----------|
| p0   | 0xF0 | 0 -> 3   | INPUT     |
| p0   | 0xF1 | 4 -> 7   | INPUT     |
| p0   | 0xF2 | 8 -> 11  | OUTPUT    |
| p0   | 0xF3 | 12 -> 16 | OUTPUT    |
| p1   | 0xF4 | 0 -> 3   | INPUT     |
| p1   | 0xF5 | 4 -> 7   | INPUT     |
| p1   | 0xF6 | 8 -> 11  | OUTPUT    |
| p1   | 0xF7 | 12 -> 16 | OUTPUT    |
| p2   | 0xF8 | 0 -> 3   | INPUT     |
| p2   | 0xF9 | 4 -> 7   | INPUT     |
| p2   | 0xFA | 8 -> 11  | OUTPUT    |
| p2   | 0xFB | 12 -> 16 | OUTPUT    |
| p3   | 0xFC | 0 -> 3   | INPUT     |
| p3   | 0xFD | 4 -> 7   | INPUT     |
| p3   | 0xFE | 8 -> 11  | OUTPUT    |
| p3   | 0xFF | 12 -> 16 | OUTPUT    |


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
| rb   | 4    | Used as bank for read and write ops                        | no (rbs)  |
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


