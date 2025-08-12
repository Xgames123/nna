.arch "nna8v2"
.org 0x00
; print A
xor r2 r2
lil &print_char.low
lih &print_char.high
jmp r0

; print B
;xor r2 r2
;inc r2 0x1
;lil &print_char.low
;lih &print_char.high
;jmp r0

end:
brk
clf
bra &end.low .reachable &end

.org 0x0E

print_char: ; char:r2

; read cursor pos into r3
lil 0x1
lih 0x0
mdb r0 ; cursor_pos bank
mrd r3 r0 ; read cursor_pos into r3
brk
; convert r2 from char to byte pos
lil 0x4
mco mul
cal r2 r0

brk

loop:
; set db to the character mem
lil 0x2
; lih 0x0 r0 high is already zero
mdb r0

mrd r1 r2
; set db back to video mem
dec r0 0x4
mdb r0

mwr r1 r3

inc r2 0x1
inc r3 0x4

bra &loop.low .reachable &loop

; store cursor_pos
lil 0x1
mdb r0
mwr r3 r0

brk

.bank 0x01
.org 0x0
0x00
cursor_pos: 0x00

.bank 0x02
.org 0x00

; A
0b11111110
0b00001001
0b00001001
0b11111110
; B
0b11111110
0b00001001
0b00001001
0b11111110

.bank 0xFE
.org 0x00
video_mem:
