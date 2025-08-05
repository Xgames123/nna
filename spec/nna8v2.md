# nna8v2

# Flag
The flag is a bit that gets set or reset by some instructions (during overflow, companions, ...).

# Registers
> [!NOTE]
> All registers including pc are reset to 0 when the device boots up.

> **op access**
> How the register can be accessed
>

| name  | size | description                                           | access |
|-------|------|-------------------------------------------------------|--------|
| *r0*  | 8    | General purpose, immediates will be loaded into here. | mov    |
| *r1*  | 8    | General purpose                                       | mov    |
| *r2*  | 8    | General purpose                                       | mov    |
| *r3*  | 8    | General purpose                                       | mov    |
| *pc*  | 8    | Program counter                                       | -      |
| *cop* | 4    | Current calc operation, Used by cal instruction.      | cal    |
| *db*  | 8    | Current bank.                                         | mdb    |
| *pb*  | 8    | Program bank.                                         | mpb    |

# Banks
The data bank select register (*db*) is used to select on which bank mwr and mrd instructions operate. Note: Only the first bank is executable and the last bank is reserved for IO.

# Instructions
Instructions are 1 byte where the first 4 bits are the opcode followed by 2 arguments each 2 bits.
Parameters that take a register are noted using: [description].

| name | opcode |  arg0  |   arg1   | description                                                                       | cycles | flag          | reg-io |
|------|--------|:------:|:--------:|-----------------------------------------------------------------------------------|--------|---------------|:------:|
| nop  | 0x0    |   00   |    00    | Does nothing.                                                                     | 1      | /             |   -    |
| brk  | 0x0    |   01   |    00    | Break the debugger.                                                               | 1      | /             |   -    |
| flf  | 0x0    |   10   |    00    | Flips flag (if flag was set reset else set)                                       | 1      | write         |   -    |
| clf  | 0x0    |   11   |    00    | Clear flag                                                                        | 1      | write         |   -    |
| jmp  | 0x0    | [reg]  |    01    | Do a long jump to [addr] when the overflow flag is **not** set                    | 1      | skip when set |   -    |
| mpb  | 0x0    | [bank] |    10    | Move [bank] into the *pb* register                                                | 1      | /             | rw(pb) |
| mdb  | 0x0    | [bank] |    11    | Move [bank] into the *db* register                                                | 1      | /             | rw(db) |
| eq   | 0x1    |  [a]   |   [b]    | Sets the overflow flag to the result of !([a] == [b]).                            | 1      | write         |   r    |
| gt   | 0x2    |  [a]   |   [b]    | Sets the overflow flag to the result of !([a] > [b]).                             | 1      | write         |   r    |
| ?    | 0x3    | [reg]  |  [reg]   | unused                                                                            | 1      | /             |   -    |
| ?    | 0x4    | [reg]  |  [reg]   | unused                                                                            | 1      | /             |   -    |
| ?    | 0x5    | [reg]  |  [reg]   | unused                                                                            | 1      | /             |   -    |
| bra  | 0x5    |  addr  |   addr   | Branch to addr when the overflow flag is **not** set.                             | 1      | skip when set |   -    |
| add  | 0x6    |   00   |    00    | Sets *cop* to add                                                                 | 1      | /             |   -    |
| sub  | 0x6    |   00   |    01    | Sets *cop* to subtract                                                            | 1      | /             |   -    |
| div  | 0x6    |   00   |    10    | Sets *cop* to div                                                                 | 1      | /             |   -    |
| mul  | 0x6    |   00   |    11    | Sets *cop* to multiply                                                            | 1      | /             |   -    |
| shl  | 0x6    |   01   |    00    | Sets *cop* to shift left                                                          | 1      | /             |   -    |
| shr  | 0x6    |   01   |    01    | Sets *cop* to shift right                                                         | 1      | /             |   -    |
| rol  | 0x6    |   01   |    10    | Sets *cop* to rotate left                                                         | 1      | /             |   -    |
| ror  | 0x6    |   01   |    11    | Sets *cop* to rotate right                                                        | 1      | /             |   -    |
| and  | 0x6    |   10   |    00    | Sets *cop* to bit wise and                                                        | 1      | /             |   -    |
| or   | 0x6    |   10   |    01    | Sets *cop* to bit wise or                                                         | 1      | /             |   -    |
| not  | 0x6    |   10   |    10    | Sets *cop* to bit wise not                                                        | 1      | /             |   -    |
| ?    | 0x6    |   10   |    11    | unused                                                                            | 1      | /             |   -    |
| ?    | 0x6    |   11   |    00    | unused                                                                            | 1      | /             |   -    |
| ?    | 0x6    |   11   |    01    | unused                                                                            | 1      | /             |   -    |
| ?    | 0x6    |   11   |    10    | unused                                                                            | 1      | /             |   -    |
| ?    | 0x6    |   11   |    11    | unused                                                                            | 1      | /             |   -    |
| mwr  | 0x7    | [reg]  |  [addr]  | Writes [reg] to memory at [addr].                                                 | 2      | /             |   r    |
| mrd  | 0x8    | [reg]  |  [addr]  | Reads the value at memory address [addr] into [reg].                              | 2      | /             |   w    |
| lil  | 0x9    | value  |  value   | Loads the immediate value into the low part of r0.                                | 1      | /             |   w    |
| lih  | 0xA    | value  |  value   | Loads the immediate value into the high part of r0.                               | 1      | /             |   w    |
| mov  | 0xB    | [dest] | [source] | Copies (moves) the value from [source] into [dest].                               | 1      | /             |   rw   |
| ?    | 0xC    | [reg]  |  [reg]   | unused                                                                            | 1      | /             |   w    |
| cal  | 0xD    |  [a]   |   [b]    | Executes the math operation in *cop* on [a] and [b] and stores the result in [a]. | 1      | overflow      |   rw   |
| xor  | 0xE    |  [a]   |   [b]    | xor's [a] and [b] and stores the result in [a].                                   | 1      | /             |   rw   |
| inc  | 0xF    | [reg]  |    00    | Increment [reg] by 1                                                              | 1      | overflow      |   rw   |
| dec  | 0xF    | [reg]  |    01    | Decrement [reg] by 1                                                              | 1      | overflow      |   rw   |
| ?    | 0xF    | [reg]  |    10    | unused                                                                            | 1      | /             |   w    |
| ?    | 0xF    | [reg]  |    11    | unused                                                                            | 1      | /             |   w    |

