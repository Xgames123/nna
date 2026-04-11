; pong for nna8v1
.org 0x00
start:
clf

; clear
lil &video_mem.low
lih &video_mem.high
mov r1 r0 ;r1=video mem
xor r3 r3
clr_loop:
mwr r3 r0 ; write 0 to screen mem
inc r0
bra &clr_loop.low .assert_max_dist &clr_loop 0x10

; draw
lil &ballx.low
lih &ballx.high
mrd r2 r0
add r2 r1

lil &bally.low
lih &bally.high
mrd r1 r0
xor r3 r3
inc r3 ; r3=1
rol r3 r1 ; r1=value(bally)

mwr r3 r2 ; write!!

; phys update

; dir y (keep r1)
lil &diry.low
lih &diry.high
mov r3 r0 ; r3=addr(diry)
mrd r2 r0

lil 0x7
lih 0x0
eq r2 r0
flf
bra &y_skip_flip.low .assert_max_dist &y_skip_flip 0x10
not r2 r2
mwr r2 r3
y_skip_flip:

dec r2
bra &y_skip_inc.low .assert_max_dist &y_skip_inc 0x10
inc r1 ; inc bally
inc r1 ; inc again (because we reach a dec)
y_skip_inc:
dec r1 ; dec bally
clf

; write bally
lil &bally.low
lih &bally.high
mwr r1 r0
inc r0 ; addr(ball x)
mrd r1 r0 ; read ballx

; dir x
lil &dirx.low
lih &dirx.high
mrd r0 r0

dec r0
bra &x_skip_inc.low .assert_max_dist &x_skip_inc 0x10
inc r1 ; inc ballx
inc r1 ; inc again (because we reach a dec)
y_skip_inc:
dec r1 ; dec ballx
clf

; write ballx
lil &ballx.low
lih &ballx.high
mwr r1 r0

; jmp to start lil &start.low
lih &start.high
lil &start.low
clf
jmp r0


.org 0xE0
  paddle_one: 0x00
  paddle_two: 0x00
  bally: 0x00
  ballx: 0x00
  dirx: 0x00
  diry: 0x00

; IO
.org 0xEF
; keyboard
keyboard:

; screen memory
.org 0xF0
video_mem:
