	.file	"c_to_asm.c"
	.text
	.def	__main;	.scl	2;	.type	32;	.endef
	.globl	main
	.def	main;	.scl	2;	.type	32;	.endef
	.seh_proc	main
main:
	pushq	%rbp
	.seh_pushreg	%rbp
	movq	%rsp, %rbp
	.seh_setframe	%rbp, 0
	subq	$32, %rsp
	.seh_stackalloc	32
	.seh_endprologue
	movl	%ecx, 16(%rbp)
	movq	%rdx, 24(%rbp)
	call	__main
	movl	$7, %ecx
	call	exit
	nop
	.seh_endproc
	.ident	"GCC: (Rev7, Built by MSYS2 project) 13.1.0"
	.def	exit;	.scl	2;	.type	32;	.endef
