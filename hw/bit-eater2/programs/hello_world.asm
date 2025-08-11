
.org 0x00
lih &message.high
mov r1 r0

lil &video_mem.low
lih &video_mem.high
mov r3 r0 ; r3 target

brk
.org 0x10
loop:
; switch to data
xor r0 r0
mdb r0 ; set bank back to 0

mrd r2 r1 ; read current char into r2

; switch to video mem bank
dec r0 0x2 ; r0 = 0xFF
mdb r0 ; set db to 0xFF

; check for null
xor r0 r0 ; zero r0
eq r2 r0 ; check if r2 is zero
bra &end.low .reachable &end
clf

; write char
mwr r2 r3
inc r1 0x1
inc r3 0x1 ; should not overflow
; flag is not set because r3 should not overflow
bra &loop.low .reachable &loop

end:
brk
lil &end.low
lih &end.high
jmp r0

.org 0x30

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

.bank 0xFE
.org 0x00
video_mem:
