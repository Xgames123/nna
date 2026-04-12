# Installation

Run `tools/install.sh`

# Assembly language (nnaasm)

An overview of the nnaasm assembly language. [Information about the architectures](../README.md) and instructions can be found in the README.

## Assembler directives

Assembler directives are instructions for the assembler. You can recognize them with the leading dot.

### `.org`

```asm
.org 0xF0  ; all code and data below this directive will be put at location A0
mov r1 r2 ; this will be put at A0
mov r2 r1 ; this at A1

```

### `.include_bytes`

Includes the specified file as bytes.

```asm
.include_bytes "test.bin"
```

### `.reachable`

Throws an error when the branch instruction can not jump to specified address.

(useful mostly for nna8v2 and nna8v3)

```asm

.org 0xA0
start_of_code:

; some code that I want between 0xA0 and 0xB0

bra 0x0 .reachable &start_of_code
; .reachable 0xA0 ; this is also possible

```

### `.bank`

Same as .org but for banks.

NOTE: banks are only supported on nna8v2 and later

```
.bank 0xA0 ; the following code will be written to 0xA0 bank
```

## Labels

```
label_name: ; define label

&label_name ; byte address of the label.
&label_name.low ; low 4 bits of the byte address of the label.
&label_name.high ; high 4 bits of the byte address of the label.
; For example this code loads the address of label into r0.
lil &label_name.low
lih &label_name.high
```

## Pseudo instructions

Pseudo instructions
These instructions will be converted to a real instruction by the assembler.

If using a pseudo instruction with a relative label as an argument type.
The assembler will pick the right instruction (forward or backwards) and resolve the label to a relative address.
An error will be thrown when the address is out of range.

Example

```asm
label_name:
; some instructions
jr label_name ; NOTE: no & is required here because & gives the byte address of the label.
```

# Example programs

Example programs are available at `hw/<nna_arch>/programs`. They can be compiled by running the build.sh script in the same directory.
