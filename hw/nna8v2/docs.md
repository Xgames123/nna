# Wiring

Information of buses and wires

# nna8v2

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
