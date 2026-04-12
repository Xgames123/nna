# nna8v3

An 8 bit general purpose processor optimized for area and for tapeout at [tinytapeout](https://tinytapeout.com).

# Memory

Memory is divided into 256 banks with each 256 bytes of addressable memory for a total of 65K of addressable memory.
The data bank select register (_db_) is used to select on which bank mwr and mrd instructions operate.

All banks are executable. Switching the executing bank can be done using the mpb instruction.

## Memory map

| addr     | size     | function        |
| -------- | -------- | --------------- |
| ..0x1000 | (0x1000) | Flash (readony) |
| ..       | (0xEF)   | RAM             |
| 0xFF00   | (0xFF)   | IO bank         |

> ![NOTE]
> Ranges don't include the lower bound

> ![NOTE] unmapped IO access
> Writes to unmapped addresses on the IO bank wil write to void and reads will result in zero

# Flags

| name | description                                                                           |
| ---- | ------------------------------------------------------------------------------------- |
| `cf` | Conditional flag. Set by `eq` and `gt` instructions and checked by `jrc` instructions |
| `of` | Overflow flag. Set when the result of an operation overflows                          |

# Registers

Registers are identical to [nna8v2](./nna8v2.md).

> [!NOTE]
> All registers including pc are reset to 0 when the device boots up.

> **access**
> How the register can be accessed

| name | size | description                                                | access |
| ---- | ---- | ---------------------------------------------------------- | ------ |
| _r0_ | 8    | General purpose, destination for lil and lih instructions. | mov    |
| _r1_ | 8    | General purpose                                            | mov    |
| _r2_ | 8    | General purpose                                            | mov    |
| _r3_ | 8    | General purpose                                            | mov    |
| _pc_ | 8    | Program counter                                            | -      |
| _co_ | 4    | Calc operation, Used by cal instruction.                   | mco    |
| _db_ | 8    | Current bank for mwr and mrd.                              | mdb    |
| _pb_ | 8    | Currently executing bank.                                  | mpb    |

# Instructions

Instructions are mostly identical to [nna8v2](./nna8v2.md). Only changes related to the `flg` and branching instructions.

Instructions are 1 byte where the first 4 bits are the opcode followed by 2 arguments each 2 bits.
Parameters that take a register are noted using: [description].

| name | opcode |  arg0   |   arg1   | description                                                                      | cycles | flag              | reg-io      |
| ---- | ------ | :-----: | :------: | -------------------------------------------------------------------------------- | ------ | ----------------- | ----------- |
| sin  | 0x0    |  {ins}  |  {ins}   | Executes the sub instruction {ins}.                                              | 1-2    | -                 | w(dp) w(db) |
| eq   | 0x1    |   [a]   |   [b]    | Sets the `flag` to the result of ([a] == [b]).                                   | -      | `cf` equal        | r           |
| gt   | 0x2    |   [a]   |   [b]    | Sets the `flag` to the result of ([a] > [b]).                                    | -      | `cf` greater than | r           |
| brc  | 0x3    |  {ins}  | {count}  | Branch relative {count} bytes conditionally based on {ins}.                      | -      | -                 | -           |
| jf   | 0x4    | {count} | {count}  | Jump forward {count}+1 bytes                                                     | -      | -                 | -           |
| jb   | 0x5    | {count} | {count}  | Jump backwards {count}+1 bytes.                                                  | 2      | -                 | -           |
| mco  | 0x6    |  {co}   |   {co}   | Loads the immediate {co} into the _co_ register                                  | -      | -                 | w(co)       |
| mwr  | 0x7    |  [reg]  |  [addr]  | Writes [reg] to memory at [addr].                                                | 2      | -                 | r           |
| mrd  | 0x8    |  [reg]  |  [addr]  | Reads the value at memory address [addr] into [reg].                             | 2      | -                 | w           |
| lil  | 0x9    | {value} | {value}  | Loads the immediate {value} into the low part of r0.                             | -      | -                 | w           |
| lih  | 0xA    | {value} | {value}  | Loads the immediate {value} into the high part of r0.                            | -      | -                 | w           |
| mov  | 0xB    | [dest]  | [source] | Copies (moves) the value from [source] into [dest].                              | -      | -                 | rw          |
| cal  | 0xC    |   [a]   |   [b]    | Executes the math operation in _co_ on [a] and [b] and stores the result in [a]. | -      | `of`              | rw r(co)    |
| xor  | 0xD    |   [a]   |   [b]    | xor's [a] and [b] and stores the result in [a].                                  | -      | -                 | rw          |
| inc  | 0xE    |  [reg]  | {amount} | Increment [reg] by {amount}+1                                                    | -      | `of`              | rw          |
| dec  | 0xF    |  [reg]  | {amount} | Decrement [reg] by {amount}+1                                                    | -      | `of`              | rw          |

> [!NOTE]
> A '-' in the cycles column means 1 cycle and is done for readability

> [!NOTE]
> Flags not mentioned in the flag column will be reset by the instruction

## Pseudo instructions

These instructions will be converted to a real instruction by the assembler

| name | conversion                                     |
| ---- | ---------------------------------------------- |
| jr   | `jf` or `jb`depending on the label argument    |
| brs  | `bfs` or `bbs` depending on the label argument |
| brr  | `bfr` or `bbr` depending on the label argument |
| clf  | `nop`                                          |

## sin {ins} {ins}

Run the sub instruction {ins}

| sub inst | arg0   | arg1 | description                                               | cycles | flag      | reg-io  |
| -------- | ------ | ---- | --------------------------------------------------------- | ------ | --------- | ------- |
| nop      | 00     | 00   | Does nothing.                                             | -      | -         | -       |
| brk      | 01     | 00   | Break the debugger.                                       | -      | -         | -       |
| ovf      | 10     | 00   | sets the `cf` to the value of `of`                        | -      | `cf`=`of` | -       |
| ?        | 11     | 00   | unused                                                    | -      | -         | -       |
| jmp      | [reg]  | 01   | Do a long jump to [addr]                                  | 2      | -         | -       |
| mpb      | [bank] | 10   | Move [bank] into the _pb_ register and reset _pc_ to zero | -      | -         | r w(pb) |
| mdb      | [bank] | 11   | Move [bank] into the _db_ register                        | -      | -         | r w(db) |

## jrc {flip}{back} {count}

| sub inst | arg0 | arg1    | description                                              | cycles | flag               | reg-io |
| -------- | ---- | ------- | -------------------------------------------------------- | ------ | ------------------ | ------ |
| bfs      | 00   | {count} | Branch forward {count}+1 bytes when `cf` is **set**.     | -      | Conditional on`cf` | -      |
| bbs      | 01   | {count} | Branch backwards {count}+1 bytes when `cf` is **set**.   | -      | Conditional on`cf` | -      |
| bfr      | 10   | {count} | Branch forward {count}+1 bytes when `cf` is **reset**.   | -      | Conditional on`cf` | -      |
| bbr      | 11   | {count} | Branch backwards {count}+1 bytes when `cf` is **reset**. | -      | Conditional on`cf` | -      |

## mco {co} {co}

Move the calculate operation into the _co_ register

NOTE: the div and mod operations have been removed

| op  | arg0 | arg1 | description         |
| --- | ---- | ---- | ------------------- |
| add | 00   | 00   | Addition            |
| sub | 00   | 01   | Subtraction         |
| mul | 00   | 10   | Multiply            |
| ?   | 00   | 11   |                     |
| shl | 01   | 00   | Logical left shift  |
| shr | 01   | 01   | Logical right shift |
| rol | 01   | 10   | Rotate left         |
| ror | 01   | 11   | Rotate right        |
| and | 10   | 00   | Binary and          |
| or  | 10   | 01   | Binary or           |
| not | 10   | 10   | Binary not          |
| ?   | 10   | 11   |                     |
| ?   | 11   | 00   |                     |
| ?   | 11   | 01   |                     |
| ?   | 11   | 10   |                     |
| ?   | 11   | 11   |                     |
