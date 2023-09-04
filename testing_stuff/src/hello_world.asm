; Copied from https://sonictk.github.io/asm_tutorial/ as a way to test nasm, I am not very familliar with assembly (at this point)
; nasm -f win64 -o hello_world.obj hello_world.asm
; -f file_system (windows 64 bit) -o output_file input_file

; link hello_world.obj /subsystem:console /entry:main /out:hello_world_basic.exe
; INSTEAD run below in x64 native tools command promt for VS 2022
; link hello_world.obj /subsystem:console /out:hello_world_basic.exe kernel32.lib legacy_stdio_definitions.lib msvcrt.lib
; Opening code in the x64 cmd works for loading the correct (I guess) vcvarsall.bat config for x64 (Opening normaly after closing will cause it to revert back to x86)

; $LASTEXITCODE

; Return values in (scalar) RAX or (non-scalar: float, double, vector-types: __m128, __m128i, __m128d) XMM0

bits 64
default rel

segment .data
    msg db "Hello world!", 0xd, 0xa, 0

segment .text
global main
extern ExitProcess

extern printf

main:
    push    rbp
    mov     rbp, rsp
    sub     rsp, 32

    lea     rcx, [msg]
    call    printf

    xor     rax, rax
    mov     ecx, 47 ; set ecx to 47 which is used as exit code (47 means everything went well)
    call    ExitProcess