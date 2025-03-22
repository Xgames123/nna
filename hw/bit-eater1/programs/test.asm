.org 0x00


lih &special.high
lil &special.low
loop:
;inc r1
mrd r1 r0
bra &loop.low
brk
special: 0x69

; cmp check
; lih 0x2
; lil 0x4
; mov r1 r0
;
; lih 0x6
; lil 0x9
; mov r2 r0
;
; brk
; eq r2 r2
; gt r2 r1
; gt r1 r2
; eq r1 r2
;
; lil 0x0
; lih 0x0
; eq r1 r0
; brk
;
; lil lih test
; lil 0x2
; mov r2 r0
; lih 0x6
; lil 0x9
; mov r3 r0
; rol r3 r2
; brk

; inc dec test
; inc r0
; inc r1
; inc r2
; dec r3
; flf
; mov r1 r3
; brk

; oflag test
; clf
; flf
; clf
; brk

.org 0x10
