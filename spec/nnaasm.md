# Assembly language (nnaasm)

```asm
.org F0  ; all code and data below will be put at location A0
mov r1 r2 ; this will be put at A0
mov r2 r1 ; this at A1

label_name: ; define label

&label_name ; ref to label
```
