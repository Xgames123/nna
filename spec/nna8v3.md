# nna8v3

An 8 bit general purpose processor optimized for area and for tapeout at [tinytapeout](https://tinytapeout.com).

# Variants

Variants differ in the peripherals available. Multiple variant tags can be combined. For example nna8v2vp has a video buffer and two memory mapped PS/2 ports

| variant     | description                            |
| ----------- | -------------------------------------- |
| nna8v2**s** | Has a memory mapped SPI port available |
| nna8v2**u** | Has a memory mapped UART available     |

# Memory

Memory is divided into 256 banks with each 256 bytes of addressable memory for a total of 65K of addressable memory.
The data bank select register (_db_) is used to select on which bank mwr and mrd instructions operate.

All banks are executable. Switching the executing bank can be done using the mpb instruction.

## Memory map

| addr     | size     | function        |
| -------- | -------- | --------------- |
| ..0x1000 | (0x1000) | Flash (readony) |
| ?        | (0x8)    | Internal ram    |
| ..       | (0xEF)   | RAM             |
| 0xFF00   | (0xFF)   | IO bank         |

> [!NOTE]
> Ranges don't include the lower bound

## IO bank

| addr | size   | register | variants    |
| ---- | ------ | -------- | ----------- |
| 0x00 | (0x01) | _pf_     |             |
| 0x01 | (0x01) | _ps_     |             |
| 0x02 | (0x01) | _gi_     |             |
| 0x03 | (0x01) | _go_     |             |
| 0x04 | (0x01) | _rx_     | nna8v3**u** |
| 0x05 | (0x01) | _tx_     | nna8v3**u** |
| 0x06 | (0x01) | _sd_     | nna8v3**s** |
| ..   | -      | unmapped |             |
| 0x10 | (0x01) | _pcr_    |             |
| 0x11 | (0x01) | _co_     |             |
| 0x12 | (0x01) | _db_     |             |
| 0x13 | (0x01) | _pb_     |             |
| ..   | -      | unmapped |             |

See [IO section](#IO) for more information about IO registers

> [!NOTE] Unmapped IO access
> Writes to unmapped addresses on the IO bank wil write to void and reads will result in zero

# Flags

| name | description                                                                           |
| ---- | ------------------------------------------------------------------------------------- |
| `cf` | Conditional flag. Set by `eq` and `gt` instructions and checked by `jrc` instructions |

# Interrupts

Interrupts are triggered by the `brk` instruction. When triggered the cpu will jump to address (TODO find address) and store the current value of _pc_ in the _pcr_ register at the end of flash memory _pb_ will also be shadowed to 0 when in an interrupt handler.

When using `rfi` instruction the cpu will exit the interrupt handler and jump to the address in _pcr_.

# Registers

Registers are identical to [nna8v2](./nna8v2.md).

> [!NOTE]
> All registers including pc are reset to 0 when the device boots up.

> [!NOTE] **access**
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

## IO

| name | size | description                     | access | variant     |
| ---- | ---- | ------------------------------- | ------ | ----------- |
| _pf_ | 8    | Peripheral flags                | 0xFF00 |             |
| _ps_ | 8    | Peripheral status               | 0xFF01 |             |
| _gi_ | 8    | General purpose input pins[^1]  | 0xFF02 |             |
| _go_ | 8    | General purpose output pins[^1] | 0xFF03 |             |
| _rx_ | 8    | UART RX register                | 0xFF04 | nna8v3**u** |
| _tx_ | 8    | UART TX register                | 0xFF05 | nna8v3**u** |
| _sd_ | 8    | SPI data register               | 0xFF06 | nna8v3**s** |

[^1]: GPI 6,7 and GPI 5,6,7 are disabled when UART and SPI are enabled.

### Peripheral flags

The Peripheral flags register (_pf_) holds which peripherals are enabled

| bit | function    |
| --- | ----------- |
| 1   | UART enable |
| 2   | SPI enable  |
| 3   | unused      |
| 4   | unused      |
| 5   | unused      |
| 6   | unused      |
| 7   | unused      |
| 8   | unused      |

### Peripheral status

The Peripheral status register (_ps_) holds the statuses of the peripherals

| bit | function                                                         |
| --- | ---------------------------------------------------------------- |
| 1   | UART ready. Set to 1 when data is available in the _rx_ register |
| 2   | SPI ready. Set to 1 when data is available in the _sd_ register. |
| 3   | unused                                                           |
| 4   | unused                                                           |
| 5   | unused                                                           |
| 6   | unused                                                           |
| 7   | unused                                                           |
| 8   | unused                                                           |

# Instructions

Instructions are mostly identical to [nna8v2](./nna8v2.md). Only changes related to the `flg` and branching instructions.

Instructions are 1 byte where the first 4 bits are the opcode followed by 2 arguments each 2 bits.
Parameters that take a register are noted using: [description].

| name | opcode |  arg0   |   arg1   | description                                                                      | cycles | flag                              | reg-io      |
| ---- | ------ | :-----: | :------: | -------------------------------------------------------------------------------- | ------ | --------------------------------- | ----------- |
| sin  | 0x0    |  {ins}  |  {ins}   | Executes the sub instruction {ins}.                                              | 1-2    | -                                 | w(dp) w(db) |
| eq   | 0x1    |   [a]   |   [b]    | Sets the `flag` to the result of ([a] == [b]).                                   | -      | `cf` equal                        | r           |
| gt   | 0x2    |   [a]   |   [b]    | Sets the `flag` to the result of ([a] > [b]).                                    | -      | `cf` greater than                 | r           |
| brc  | 0x3    |  {ins}  | {count}  | Branch relative {count} bytes conditionally based on {ins}.                      | -      | Conditional on `cf`               | -           |
| jf   | 0x4    | {count} | {count}  | Jump forward {count}+1 bytes                                                     | -      | -                                 | -           |
| jb   | 0x5    | {count} | {count}  | Jump backwards {count}+1 bytes.                                                  | 2      | -                                 | -           |
| mco  | 0x6    |  {co}   |   {co}   | Loads the immediate {co} into the _co_ register                                  | -      | -                                 | w(co)       |
| mwr  | 0x7    |  [reg]  |  [addr]  | Writes [reg] to memory at [addr].                                                | 2      | -                                 | r           |
| mrd  | 0x8    |  [reg]  |  [addr]  | Reads the value at memory address [addr] into [reg].                             | 2      | -                                 | w           |
| lil  | 0x9    | {value} | {value}  | Loads the immediate {value} into the low part of r0.                             | -      | -                                 | w           |
| lih  | 0xA    | {value} | {value}  | Loads the immediate {value} into the high part of r0.                            | -      | -                                 | w           |
| mov  | 0xB    | [dest]  | [source] | Copies (moves) the value from [source] into [dest].                              | -      | -                                 | rw          |
| cal  | 0xC    |   [a]   |   [b]    | Executes the math operation in _co_ on [a] and [b] and stores the result in [a]. | -      | Conditional on `cf`/`cf` overflow | rw r(co)    |
| xor  | 0xD    |   [a]   |   [b]    | xor's [a] and [b] and stores the result in [a].                                  | -      | -                                 | rw          |
| inc  | 0xE    |  [reg]  | {amount} | Increment [reg] by {amount}+1                                                    | -      | `cf` overflow                     | rw          |
| dec  | 0xF    |  [reg]  | {amount} | Decrement [reg] by {amount}+1                                                    | -      | `cf` overflow                     | rw          |

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

| sub inst | arg0   | arg1 | description                                                                                        | cycles | flag | reg-io  |
| -------- | ------ | ---- | -------------------------------------------------------------------------------------------------- | ------ | ---- | ------- |
| nop      | 00     | 00   | Does nothing.                                                                                      | -      | -    | -       |
| brk      | 01     | 00   | Jump to the interrupt handler. and break the debugger in a simulator. [see interrupt](#Interrupts) | -      | -    | -       |
| ric      | 10     | 00   | Reset the instruction cache. Filling it with the next instructions until full                      | -      | -    | -       |
| rfi      | 11     | 00   | Return from interrupt [see interrupts](#Interrupts)                                                | -      | -    | -       |
| jmp      | [reg]  | 01   | Do a long jump to [addr]                                                                           | 2      | -    | -       |
| mpb      | [bank] | 10   | Move [bank] into the _pb_ register and reset _pc_ to zero                                          | -      | -    | r w(pb) |
| mdb      | [bank] | 11   | Move [bank] into the _db_ register                                                                 | -      | -    | r w(db) |

## jrc {flip}{back} {count}

| sub inst | arg0 | arg1    | description                                              | cycles | flag               | reg-io |
| -------- | ---- | ------- | -------------------------------------------------------- | ------ | ------------------ | ------ |
| bfs      | 00   | {count} | Branch forward {count}+1 bytes when `cf` is **set**.     | -      | Conditional on`cf` | -      |
| bbs      | 01   | {count} | Branch backwards {count}+1 bytes when `cf` is **set**.   | -      | Conditional on`cf` | -      |
| bfr      | 10   | {count} | Branch forward {count}+1 bytes when `cf` is **reset**.   | -      | Conditional on`cf` | -      |
| bbr      | 11   | {count} | Branch backwards {count}+1 bytes when `cf` is **reset**. | -      | Conditional on`cf` | -      |

## mco {co} {co}

Move the calculate operation into the _co_ register

The instructions div and mod have been removed and Everything have been reordered and adc and auc have been added.

| op  | arg0 | arg1 | description                         |
| --- | ---- | ---- | ----------------------------------- |
| add | 00   | 00   | Addition                            |
| sub | 00   | 01   | Subtraction                         |
| mul | 00   | 10   | Multiply                            |
| shl | 00   | 11   | Logical left shift                  |
| shr | 01   | 00   | Logical right shift                 |
| not | 01   | 01   | Binary not                          |
| and | 01   | 10   | Binary and                          |
| or  | 01   | 11   | Binary or                           |
| adc | 10   | 00   | Addition use `cf` as carry input    |
| suc | 10   | 01   | Subtraction use `cf` as carry input |
| rol | 10   | 11   | Rotate left                         |
| ror | 11   | 00   | Rotate right                        |
| not | 11   | 01   | Binary not                          |
| and | 11   | 10   | Binary and                          |
| or  | 11   | 11   | Binary or                           |
