.org 0x00

lil &keyboard.low
lih &keyboard.high
mov r1 r0
lil &video_mem.low
lih &video_mem.high
xor r3 r3
loop:
brk
mrd r2 r1
eq r2 r3 ;  check if r2 is 0
bra &loop.low .assert_max_dist &loop 0x10
mwr r2 r0
clf
bra &loop.low .assert_max_dist &loop 0x10

; IO
.org 0xEF
; keyboard
keyboard:

; screen memory
.org 0xF0
video_mem:
