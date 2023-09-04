	.file	"c_to_asm.c"
	.text
	.def	___main;	.scl	2;	.type	32;	.endef
	.globl	_main
	.def	_main;	.scl	2;	.type	32;	.endef
_main:
	pushl	%ebp
	movl	%esp, %ebp
	andl	$-16, %esp
	subl	$16, %esp
	call	___main
	movl	$7, (%esp)
	call	_exit
	.ident	"GCC: (Rev7, Built by MSYS2 project) 13.1.0"
	.def	_exit;	.scl	2;	.type	32;	.endef
