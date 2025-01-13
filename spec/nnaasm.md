# Assembly language (nnaasm)

```asm

; all compiler directives start with a .
.org 0xF0  ; all code and data below will be put at location A0
mov r1 r2 ; this will be put at A0
mov r2 r1 ; this at A1

label_name: ; define label

&label_name ; ref to label
&label_name.low ; ref to low part of label
&label_name.high ; ref to high part of label


.include_bytes "test.bin" ; includes the file test.bin as bytes


.org 0xA0
start_of_code:

; some code that I want between 0xA0 and 0xB0

.assert_dist 0xA0 0x10 ; .assert_max_dist <start> <max_distance> gives a compiler error when the distance between the directive and <start> is more than max_distance
.assert_dist &start_of_code 0x10 ; this is also possible
```
