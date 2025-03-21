# nna8v2

# Flag
The flag is a bit that gets set or reset by some instructions (during overflow, companions, ...).

# Registers
> [!NOTE]
> All registers including pc are reset to 0 when the device boots up.

> **op access**
> The register can be accessed in operations. Ex. mov r0 r2
>

| name  | size | description                                           | op access |
|-------|------|-------------------------------------------------------|-----------|
| *r0*  | 8    | General purpose, immediates will be loaded into here. | yes       |
| *r1*  | 8    | General purpose                                       | yes       |
| *r2*  | 8    | General purpose                                       | yes       |
| *r3*  | 8    | General purpose                                       | yes       |
| *pc*  | 8    | Program counter                                       | no        |
| *cop* | 4    | Current calc operation, Used by cal instruction.      | no        |
| *bs*  | 8    | Current bank.                                         | no        |

# Banks
The bank select register (*bs*) is used to select on which bank mwr and mrd instructions operate. Note: Only the first bank is executable and the last bank is reserved for IO.

# Instructions
Instructions are 1 byte where the first 4 bits are the opcode followed by 2 arguments each 2 bits.
Parameters that take a register are noted using: [description].

| name | opcode |  arg0  |   arg1   | description                                                                       | cycles | flag          |
|------|--------|:------:|:--------:|-----------------------------------------------------------------------------------|--------|---------------|
| nop  | 0x0    |   00   |    00    | Does nothing.                                                                     | 1      | /             |
| brk  | 0x0    |   01   |    00    | Break the debugger.                                                               | 1      | /             |
| flf  | 0x0    |   10   |    00    | Flips flag (if flag was set reset else set)                                       | 1      | write         |
| clf  | 0x0    |   11   |    00    | Clear flag                                                                        | 1      | write         |
| jmp  | 0x0    | [reg]  |    01    | Do a long jump to [addr] when the overflow flag is **not** set                    | 1      | skip when set |
| inc  | 0x0    | [reg]  |    10    | Increment [reg] by 1                                                              | 1      | /             |
| dec  | 0x0    | [reg]  |    11    | Decrement [reg] by 1                                                              | 1      | /             |
| lil  | 0x1    | value  |  value   | Loads the immediate value into the low part of r0.                                | 1      | /             |
| lih  | 0x2    | value  |  value   | Loads the immediate value into the high part of r0.                               | 1      | /             |
| mwr  | 0x3    | [reg]  |  [addr]  | Writes [reg] to memory at [addr].                                                 | 2      | /             |
| mrd  | 0x4    | [reg]  |  [addr]  | Reads the value at memory address [addr] into [reg].                              | 2      | /             |
| mov  | 0x5    | [dest] | [source] | Copies (moves) the value from [source] into [dest].                               | 1      | /             |
| bra  | 0x6    |  addr  |   addr   | Branch to addr when the overflow flag is **not** set.                             | 1      | skip when set |
| add  | 0x7    |   00   |    00    | Sets *cop* to add                                                                 | 1      | /             |
| sub  | 0x7    |   00   |    01    | Sets *cop* to subtract                                                            | 1      | /             |
| div  | 0x7    |   00   |    10    | Sets *cop* to div                                                                 | 1      | /             |
| mul  | 0x7    |   00   |    11    | Sets *cop* to multiply                                                            | 1      | /             |
| shl  | 0x7    |   01   |    00    | Sets *cop* to shift left                                                          | 1      | /             |
| shr  | 0x7    |   01   |    01    | Sets *cop* to shift right                                                         | 1      | /             |
| rol  | 0x7    |   01   |    10    | Sets *cop* to rotate left                                                         | 1      | /             |
| ror  | 0x7    |   01   |    11    | Sets *cop* to rotate right                                                        | 1      | /             |
| and  | 0x7    |   10   |    00    | Sets *cop* to bit wise and                                                        | 1      | /             |
| or   | 0x7    |   10   |    01    | Sets *cop* to bit wise or                                                         | 1      | /             |
| not  | 0x7    |   10   |    10    | Sets *cop* to bit wise not                                                        | 1      | /             |
| ?    | 0x7    |   10   |    11    | unused                                                                            | 1      | /             |
| ?    | 0x7    |   11   |    00    | unused                                                                            | 1      | /             |
| ?    | 0x7    |   11   |    01    | unused                                                                            | 1      | /             |
| ?    | 0x7    |   11   |    10    | unused                                                                            | 1      | /             |
| ?    | 0x7    |   11   |    11    | unused                                                                            | 1      | /             |
| eq   | 0x8    |  [a]   |   [b]    | Sets the overflow flag to the result of !([a] == [b]).                            | 1      | write         |
| gt   | 0x9    |  [a]   |   [b]    | Sets the overflow flag to the result of !([a] > [b]).                             | 1      | write         |
| cal  | 0xa    |  [a]   |   [b]    | Executes the math operation in *cop* on [a] and [b] and stores the result in [a]. | 1      | overflow      |
| ?    | 0xB    |  [a]   |   [b]    | unused                                                                            | 1      | /             |
| ?    | 0xC    |  [a]   |   [b]    | unused                                                                            | 1      | /             |
| ?    | 0xD    |  [a]   |   [b]    | unused                                                                            | 1      | /             |
| sbs  | 0xE    |   00   |  [bank]  | Store [bank] into the *bs* register                                               | 1      | /             |
| xor  | 0xF    |  [a]   |   [b]    | xor's [a] and [b] and stores the result in [a].                                   | 1      | /             |


