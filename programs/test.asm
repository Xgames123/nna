.org 00
lih &message.high
mov r1 r0

lil &screen_mem.low
lih &screen_mem.high
mov r3 r0 ; r3 target

xor r0 r0 ; zero r0
loop:
mrd r2 r1

eq r2 r0 ; check if r2 is zero
flf
bra &end.low
; r2 is not 0
mwr r2 r3
inc r1
inc r3
clf
bra &loop.low

.org 10 ; make sure everything fits into 16 bytes because else branches can't be reached

end:
lil &end.low
lih &end.high
jmp r0

.org 20

message:
0x48 ; H
0x65 ; e
0x6C ; l
0x6C ; l
0x6F ; o
0x20 ; <space>
0x57 ; W
0x6F ; o
0x72 ; r
0x6C ; l
0x64 ; d
0x21 ; !
0x21 ; !
0x00

; screen memory
.org F5
screen_mem:
