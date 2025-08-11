.org 0x00

loop:
; set db to the character mem
lil 0x1
lih 0x0
mdb r0

mrd r1 r2
; set db back to video mem
dec r0 0x3
mdb r0

mwr r1 r3

inc r2 0x1
inc r3 0x4

bra &loop.low .reachable &loop

end:
brk
clf
bra &end.low .reachable &end



.bank 0x01
.org 0x00

; A
0b11111110
0b00001001
0b00001001
0b11111110

.bank 0xFE
.org 0x00
video_mem:
