# nna8v2

# Flag
The flag is a bit that gets set or reset by some instructions (during overflow, companions, ...).

# Registers
> [!NOTE]
> All registers including pc are reset to 0 when the device boots up.

> **op access**
> How the register can be accessed
>

| name | size | description                                           | access |
|------|------|-------------------------------------------------------|--------|
| *r0* | 8    | General purpose, immediates will be loaded into here. | mov    |
| *r1* | 8    | General purpose                                       | mov    |
| *r2* | 8    | General purpose                                       | mov    |
| *r3* | 8    | General purpose                                       | mov    |
| *pc* | 8    | Program counter                                       | -      |
| *co* | 4    | Calc operation, Used by cal instruction.              | cal    |
| *db* | 8    | Current bank.                                         | mdb    |
| *pb* | 8    | Program bank.                                         | mpb    |

# Banks
The data bank select register (*db*) is used to select on which bank mwr and mrd instructions operate. Note: Only the first bank is executable and the last bank is reserved for IO.

# Instructions
Instructions are 1 byte where the first 4 bits are the opcode followed by 2 arguments each 2 bits.
Parameters that take a register are noted using: [description].

| name | opcode |  arg0   |   arg1   | description                                                                      | cycles | flag          |   reg-io    |
|------|--------|:-------:|:--------:|----------------------------------------------------------------------------------|--------|---------------|:-----------:|
| sin  | 0x0    |  {ins}  |  {ins}   | Runs the sub instruction {ins}.                                                  | 1      | /             | w(dp) w(db) |
| eq   | 0x1    |   [a]   |   [b]    | Sets the overflow flag to the result of !([a] == [b]).                           | 1      | write         |      r      |
| gt   | 0x2    |   [a]   |   [b]    | Sets the overflow flag to the result of !([a] > [b]).                            | 1      | write         |      r      |
| ?    | 0x3    |  [reg]  |  [reg]   | unused                                                                           | 1      | /             |      /      |
| ?    | 0x4    |  [reg]  |  [reg]   | unused                                                                           | 1      | /             |      /      |
| bra  | 0x5    | {addr}  |  {addr}  | Branch to {addr} when the overflow flag is **not** set.                          | 1      | skip when set |      /      |
| mco  | 0x6    |  {co}   |   {co}   | Loads the {co} into the *co* register                                            | 1      | /             |    w(co)    |
| mwr  | 0x7    |  [reg]  |  [addr]  | Writes [reg] to memory at [addr].                                                | 2      | /             |      r      |
| mrd  | 0x8    |  [reg]  |  [addr]  | Reads the value at memory address [addr] into [reg].                             | 2      | /             |      w      |
| lil  | 0x9    | {value} | {value}  | Loads the immediate {value} into the low part of r0.                             | 1      | /             |      w      |
| lih  | 0xA    | {value} | {value}  | Loads the immediate {value} into the high part of r0.                            | 1      | /             |      w      |
| mov  | 0xB    | [dest]  | [source] | Copies (moves) the value from [source] into [dest].                              | 1      | /             |     rw      |
| cal  | 0xC    |   [a]   |   [b]    | Executes the math operation in *co* on [a] and [b] and stores the result in [a]. | 1      | overflow      |  rw r(co)   |
| xor  | 0xD    |   [a]   |   [b]    | xor's [a] and [b] and stores the result in [a].                                  | 1      | /             |     rw      |
| inc  | 0xE    |  [reg]  | {amount} | Increment [reg] by {amount}                                                      | 1      | overflow      |     rw      |
| dec  | 0xF    |  [reg]  | {amount} | Decrement [reg] by {amount}                                                      | 1      | overflow      |     rw      |


## sin {ins} {ins}
Run the sub instruction {ins}

| sub inst | arg0   | arg1 | description                                                    | flag          | reg-io |
|----------|--------|------|----------------------------------------------------------------|---------------|--------|
| nop      | 00     | 00   | Does nothing.                                                  | /             | -      |
| brk      | 01     | 00   | Break the debugger.                                            | /             | -      |
| flf      | 10     | 00   | Flips flag (if flag was set reset else set)                    | write         | -      |
| clf      | 11     | 00   | Clear flag                                                     | write         | -      |
| jmp      | [reg]  | 01   | Do a long jump to [addr] when the overflow flag is **not** set | skip when set | -      |
| mpb      | [bank] | 10   | Move [bank] into the *pb* register                             | /             | rw(pb) |
| mdb      | [bank] | 11   | Move [bank] into the *db* register                             | /             | rw(db) |

## mco {co} {co}
Move the calculate operation into the *co* register

| op  | arg0 | arg1 |
|-----|------|------|
| add | 00   | 00   |
| sub | 00   | 01   |
| div | 00   | 10   |
| mul | 00   | 11   |
| shl | 01   | 00   |
| shr | 01   | 01   |
| rol | 01   | 10   |
| ror | 01   | 11   |
| and | 10   | 00   |
| or  | 10   | 01   |
| not | 10   | 10   |
| ?   | 10   | 11   |
| ?   | 11   | 00   |
| ?   | 11   | 01   |
| ?   | 11   | 10   |
| ?   | 11   | 11   |
