# nna8v2

An 8 bit general purpose and video processor

# Memory

Memory is divided into 256 banks with each 256 bytes of addressable memory for a total of 65K of addressable memory.
The data bank select register (_db_) is used to select on which bank mwr and mrd instructions operate.

All banks are executable. Switching the executing bank can be done using the mpb instruction.

## Memory map

| addr     | size     | function  |
| -------- | -------- | --------- |
| ..0x1000 | (0x1000) | EEPROM    |
| ..       | (0xEE)   | unused    |
| 0xFE00   | (0xFF)   | video mem |
| 0xFF00   | (0xFF)   | IO bank   |

> ![NOTE]
> Ranges don't include the lower bound

## IO bank

| addr | size   | function |
| ---- | ------ | -------- |
| 0x00 | (0x01) | _pf_     |
| 0x01 | (0x01) | _p0_     |
| 0x02 | (0x01) | _p1_     |
| 0x03 | (0x01) | _vm_     |
| ..   | -      | unused   |

# Flag

The processor contains a single flag latch that get set or reset by some instructions.

# Registers

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
| _co_ | 4    | Calc operation, Used by cal instruction.                   | cal    |
| _db_ | 8    | Current bank for mwr and mrd.                              | mdb    |
| _pb_ | 8    | Currently executing bank.                                  | mpb    |

## IO

| name | size | description | access |
| ---- | ---- | ----------- | ------ |
| _pf_ | 8    | Port flags  | 0xFF00 |
| _p0_ | 8    | Port flags  | 0xFF01 |
| _p1_ | 8    | Port flags  | 0xFF02 |

## Video

| name | size | description        | access |
| ---- | ---- | ------------------ | ------ |
| _vm_ | 8    | Video current mode | 0xFF03 |

# Instructions

Instructions are 1 byte where the first 4 bits are the opcode followed by 2 arguments each 2 bits.
Parameters that take a register are noted using: [description].

| name | opcode |  arg0   |   arg1   | description                                                                      | cycles           | flag          | reg-io      |
| ---- | ------ | :-----: | :------: | -------------------------------------------------------------------------------- | ---------------- | ------------- | ----------- |
| sin  | 0x0    |  {ins}  |  {ins}   | Runs the sub instruction {ins}.                                                  | 1-2              | -             | w(dp) w(db) |
| eq   | 0x1    |   [a]   |   [b]    | Sets the overflow flag to the result of !([a] == [b]).                           | -                | write         | r           |
| gt   | 0x2    |   [a]   |   [b]    | Sets the overflow flag to the result of !([a] > [b]).                            | -                | write         | r           |
| flg  | 0x3    |  {val}  | [unused] | Flip, set or reset the flag                                                      | -                | -             | -           |
| ?    | 0x4    |  [reg]  |  [reg]   | unused                                                                           | -                | -             | -           |
| bra  | 0x5    | {addr}  |  {addr}  | Branch to {addr} when the overflow flag is **not** set.                          | 2 when branching | skip when set | -           |
| mco  | 0x6    |  {co}   |   {co}   | Loads the {co} into the _co_ register                                            | -                | -             | w(co)       |
| mwr  | 0x7    |  [reg]  |  [addr]  | Writes [reg] to memory at [addr].                                                | 2                | -             | r           |
| mrd  | 0x8    |  [reg]  |  [addr]  | Reads the value at memory address [addr] into [reg].                             | 2                | -             | w           |
| lil  | 0x9    | {value} | {value}  | Loads the immediate {value} into the low part of r0.                             | -                | -             | w           |
| lih  | 0xA    | {value} | {value}  | Loads the immediate {value} into the high part of r0.                            | -                | -             | w           |
| mov  | 0xB    | [dest]  | [source] | Copies (moves) the value from [source] into [dest].                              | -                | -             | rw          |
| cal  | 0xC    |   [a]   |   [b]    | Executes the math operation in _co_ on [a] and [b] and stores the result in [a]. | -                | overflow      | rw r(co)    |
| xor  | 0xD    |   [a]   |   [b]    | xor's [a] and [b] and stores the result in [a].                                  | -                | -             | rw          |
| inc  | 0xE    |  [reg]  | {amount} | Increment [reg] by {amount}                                                      | -                | overflow      | rw          |
| dec  | 0xF    |  [reg]  | {amount} | Decrement [reg] by {amount}                                                      | -                | overflow      | rw          |

> [!NOTE]
> A '-' in the cycles column means 1 cycle and is done for readability

## sin {ins} {ins}

Run the sub instruction {ins}

| sub inst | arg0   | arg1 | description                                                    | cycles           | flag          | reg-io  |
| -------- | ------ | ---- | -------------------------------------------------------------- | ---------------- | ------------- | ------- |
| nop      | 00     | 00   | Does nothing.                                                  | -                | -             | -       |
| brk      | 01     | 00   | Break the debugger.                                            | -                | -             | -       |
| ?        | 10     | 00   | unused                                                         | -                | -             | -       |
| ?        | 11     | 00   | unused                                                         | -                | -             | -       |
| jmp      | [reg]  | 01   | Do a long jump to [addr] when the overflow flag is **not** set | 2 when branching | skip when set | -       |
| mpb      | [bank] | 10   | Move [bank] into the _pb_ register and reset _pc_ to zero      | -                | -             | r w(pb) |
| mdb      | [bank] | 11   | Move [bank] into the _db_ register                             | -                | -             | r w(db) |

> [!NOTE]
> A '-' in the cycles column means 1 cycle and is done for readability

## flg

| op  | arg0 | description                                |
| --- | ---- | ------------------------------------------ |
| flf | 00   | Flip flag (if flag is set: reset else set) |
| flf | 10   | Flip flag (if flag is set: reset else set) |
| sef | 11   | Set the flag                               |
| clf | 01   | Reset the flag                             |

## mco {co} {co}

Move the calculate operation into the _co_ register

| op  | arg0 | arg1 | description                     |
| --- | ---- | ---- | ------------------------------- |
| add | 00   | 00   | Addition                        |
| sub | 00   | 01   | Subtraction                     |
| mul | 00   | 10   | Multiply                        |
| div | 00   | 11   | Divide                          |
| shl | 01   | 00   | Logical left shift              |
| shr | 01   | 01   | Logical right shift             |
| rol | 01   | 10   | Rotate left                     |
| ror | 01   | 11   | Rotate right                    |
| and | 10   | 00   | Binary and                      |
| or  | 10   | 01   | Binary or                       |
| not | 10   | 10   | Binary not                      |
| mod | 10   | 11   | Remainder of division (modulus) |
| ?   | 11   | 00   |                                 |
| ?   | 11   | 01   |                                 |
| ?   | 11   | 10   |                                 |
| ?   | 11   | 11   |                                 |

# IO Ports

There are two ps/2 IO ports available.

### port flags

| bit | function |
| --- | -------- |
| 1   | p0 ready |
| 2   | p1 ready |
| 3   | unused   |
| 4   | unused   |
| 5   | unused   |
| 6   | unused   |
| 7   | unused   |
| 8   | unused   |

> ![NOTE]
> Reading a 1 from the ready flag indicates the port has fully received the data.
> writing a 0 to the ready flag starts receiving the next byte from the port

# Video modes

The current video mode is stored in the _vm_ register.

| mode | res   | colors |
| ---- | ----- | ------ |
| 0x01 | 16x16 | 256    |

# Hardware Buses

Information of buses and wires

## Instruction

- `curop` The opcode of current instruction.
- `arg` The combined arguments of the current instruction.
- `arg0` First argument of the current instruction
- `arg1` Second argument of the current instruction
- `xcycle` Indicates that the current clock cycle is the second cycle of a two cycle instruction. During a second cycle the databus is released by _pc_.
- `xcycle_next` Indicates that the next clock cycle will be an xcycle

### cal

Related to the cal instruction

- `calbus` The answer of the calculation in `co` applied between `regout_arg0` and `regout_arg1`
- `co_add` high when the `co` register contains the add instruction

## Data

- `databus` Contains the data that has been read or will be written this cycle;
- `addrbus` Contains the memory location were the data on the `databus` will be read or written.
- `bankbus` Contains the currently selected bank.
- `addrbus_full` Combined `addrbus` and `bankbus` in big endian format.
- `addr_mainmem` Indicates that the address on `addrbus_full` is in main memory.
- `dwrite` Data write. Signals that current instruction wants to write the data on `databus` to the location in `addrbus`.
- `dread` Data read. Signals that current instruction wants to read the data on location `addrbus`.
- `drw` Indicates that the processor wants to read or write data.

### Registries

- `selreg` Selected register, value is based on arg0 and extra logic to handle lil and lih
- `regin` Data that will be written to the selected register
- `reg_we` Indicates that data on regin will be written to the selected register

## Video

- `addr_video` Indicates that the address on `addrbus_full` is in video memory.
- `dwrite_video` Indicates that the processor wants to write to a memory location in video memory
- `dwrite_video` Indicates that the processor wants to read a location in video memory
