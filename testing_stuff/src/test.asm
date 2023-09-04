global start

extern ExitProcess

section .text

start:
    push    1
    call    ExitProcess