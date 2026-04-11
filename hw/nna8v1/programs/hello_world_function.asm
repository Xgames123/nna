.org 0x00

; push return addr on stack
lih 0x2
mov r2 r0 ; r2 stack_ptr addr
mrd r3 r0 ; r3: stack_ptr

inc r3 ; make room on the stack

lih &ret.high
lil &ret.low
mwr r0 r3 ; write value
mwr r3 r2 ; write stack_ptr back
; end push to stack


lih &message.high
lil &message.low
mov r1 r0

lih &print.high
lil &print.low
jmp r0
ret:
brk


.org 0x20 ; stack
stack_ptr: 0x20
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

.org 0x40
print: ; r1: msg_ptr

lil &video_mem.low
lih &video_mem.high
mov r2 r0 ; r2 target_ptr

loop:
  mrd r3 r1 ; r3: cur_char
  xor r0 r0
  ;brk
  eq r0 r3
  bra &end.low .reachable &end
  clf

  mwr r3 r2 ; write cur_char to target_ptr
  inc r2 ; r2: target_ptr
  inc r1 ; r1: msg_ptr
  bra &loop.low .reachable &loop
end:
  ; pop ret addr off the stack
  lih &stack_ptr.high
  lil &stack_ptr.low
  mrd r3 r0 ; r3 stack_ptr
  mrd r1 r3 ; r1 ret_addr
  dec r3
  mwr r0 r3 ; write stack ptr back

  jmp r1 ; jmp to ret addr

.org 0xF0
video_mem:
