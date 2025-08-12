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


| name | opcode |  arg0   |   arg1   | description                                                                      | cycles           | flag          | reg-io      |
|------|--------|:-------:|:--------:|----------------------------------------------------------------------------------|------------------|---------------|-------------|
| sin  | 0x0    |  {ins}  |  {ins}   | Runs the sub instruction {ins}.                                                  | 1-2              | -             | w(dp) w(db) |
| eq   | 0x1    |   [a]   |   [b]    | Sets the overflow flag to the result of !([a] == [b]).                           | -                | write         | r           |
| gt   | 0x2    |   [a]   |   [b]    | Sets the overflow flag to the result of !([a] > [b]).                            | -                | write         | r           |
| flg  | 0x3    |  {val}  | [unused] | Flip, set or reset the flag                                                      | -                | -             | -           |
| ?    | 0x4    |  [reg]  |  [reg]   | unused                                                                           | -                | -             | -           |
| bra  | 0x5    | {addr}  |  {addr}  | Branch to {addr} when the overflow flag is **not** set.                          | 2 when branching | skip when set | -           |
| mco  | 0x6    |  {co}   |   {co}   | Loads the {co} into the *co* register                                            | -                | -             | w(co)       |
| mwr  | 0x7    |  [reg]  |  [addr]  | Writes [reg] to memory at [addr].                                                | 2                | -             | r           |
| mrd  | 0x8    |  [reg]  |  [addr]  | Reads the value at memory address [addr] into [reg].                             | 2                | -             | w           |
| lil  | 0x9    | {value} | {value}  | Loads the immediate {value} into the low part of r0.                             | -                | -             | w           |
| lih  | 0xA    | {value} | {value}  | Loads the immediate {value} into the high part of r0.                            | -                | -             | w           |
| mov  | 0xB    | [dest]  | [source] | Copies (moves) the value from [source] into [dest].                              | -                | -             | rw          |
| cal  | 0xC    |   [a]   |   [b]    | Executes the math operation in *co* on [a] and [b] and stores the result in [a]. | -                | overflow      | rw r(co)    |
| xor  | 0xD    |   [a]   |   [b]    | xor's [a] and [b] and stores the result in [a].                                  | -                | -             | rw          |
| inc  | 0xE    |  [reg]  | {amount} | Increment [reg] by {amount}                                                      | -                | overflow      | rw          |
| dec  | 0xF    |  [reg]  | {amount} | Decrement [reg] by {amount}                                                      | -                | overflow      | rw          |

> [!NOTE]
> A '-' in the cycles column means 1 cycle and is done for readability
>


## sin {ins} {ins}
Run the sub instruction {ins}

| sub inst | arg0   | arg1 | description                                                    | cycles            | flag          | reg-io  |
|----------|--------|------|----------------------------------------------------------------|-------------------|---------------|---------|
| nop      | 00     | 00   | Does nothing.                                                  | -                 | -             | -       |
| brk      | 01     | 00   | Break the debugger.                                            | -                 | -             | -       |
| ?        | 10     | 00   | unused                                                         | -                 | -             | -       |
| ?        | 11     | 00   | unused                                                         | -                 | -             | -       |
| jmp      | [reg]  | 01   | Do a long jump to [addr] when the overflow flag is **not** set | 2 when branching  | skip when set | -       |
| mpb      | [bank] | 10   | Move [bank] into the *pb* register                             | -                 | -             | r w(pb) |
| mdb      | [bank] | 11   | Move [bank] into the *db* register                             | -                 | -             | r w(db) |

> [!NOTE]
> A '-' in the cycles column means 1 cycle and is done for readability
>

## flg

| op  | arg0 | description                                |
|-----|------|--------------------------------------------|
| flf | 00   | Flip flag (if flag is set: reset else set) |
| flf | 10   | Flip flag (if flag is set: reset else set) |
| sef | 11   | Set the flag                               |
| clf | 01   | Reset the flag                             |


## mco {co} {co}
Move the calculate operation into the *co* register

| op  | arg0 | arg1 |
|-----|------|------|
| add | 00   | 00   |
| sub | 00   | 01   |
| mul | 00   | 10   |
| div | 00   | 11   |
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
