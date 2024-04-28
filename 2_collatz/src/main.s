	.section	__TEXT,__text,regular,pure_instructions
	.macosx_version_min 10, 12
	.p2align	4, 0x90
__ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17h3663897dcbd2885aE:
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	callq	__ZN4core3ops8function6FnOnce9call_once17h2608594c88af5c47E
	## InlineAsm Start
	## InlineAsm End
	popq	%rbp
	retq
	.cfi_endproc

	.private_extern	__ZN3std2rt10lang_start17hac21861e4ee02960E
	.globl	__ZN3std2rt10lang_start17hac21861e4ee02960E
	.p2align	4, 0x90
__ZN3std2rt10lang_start17hac21861e4ee02960E:
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	subq	$16, %rsp
	movl	%ecx, %eax
	movq	%rdx, %rcx
	movq	%rsi, %rdx
	movq	%rdi, -8(%rbp)
	leaq	-8(%rbp), %rdi
	leaq	l___unnamed_1(%rip), %rsi
	movzbl	%al, %r8d
	callq	__ZN3std2rt19lang_start_internal17he8e2abef2cd73af9E
	movq	%rax, -16(%rbp)
	movq	-16(%rbp), %rax
	addq	$16, %rsp
	popq	%rbp
	retq
	.cfi_endproc

	.p2align	4, 0x90
__ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h3ab922827b5e3e30E:
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	subq	$16, %rsp
	movq	(%rdi), %rdi
	callq	__ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17h3663897dcbd2885aE
	callq	__ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17hde7c82540c1cf003E
	movb	%al, -1(%rbp)
	movzbl	-1(%rbp), %eax
	addq	$16, %rsp
	popq	%rbp
	retq
	.cfi_endproc

	.p2align	4, 0x90
__ZN4core3fmt9Arguments6new_v117h4231a801aa493fe2E:
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	subq	$96, %rsp
	movq	%r8, -96(%rbp)
	movq	%rcx, -88(%rbp)
	movq	%rdx, -80(%rbp)
	movq	%rsi, -72(%rbp)
	movq	%rdi, -64(%rbp)
	movq	%rdi, -56(%rbp)
	cmpq	%r8, %rdx
	jb	LBB3_2
	movq	-80(%rbp), %rax
	movq	-96(%rbp), %rcx
	addq	$1, %rcx
	cmpq	%rcx, %rax
	ja	LBB3_4
	jmp	LBB3_3
LBB3_2:
	leaq	l___unnamed_2(%rip), %rax
	movq	%rax, -48(%rbp)
	movq	$1, -40(%rbp)
	movq	L___unnamed_3(%rip), %rcx
	movq	L___unnamed_3+8(%rip), %rax
	movq	%rcx, -16(%rbp)
	movq	%rax, -8(%rbp)
	leaq	l___unnamed_4(%rip), %rax
	movq	%rax, -32(%rbp)
	movq	$0, -24(%rbp)
	leaq	l___unnamed_5(%rip), %rsi
	leaq	-48(%rbp), %rdi
	callq	__ZN4core9panicking9panic_fmt17h3ed68d73e880d931E
LBB3_3:
	movq	-56(%rbp), %rax
	movq	-64(%rbp), %rcx
	movq	-96(%rbp), %rdx
	movq	-88(%rbp), %rsi
	movq	-80(%rbp), %rdi
	movq	-72(%rbp), %r8
	movq	%r8, (%rcx)
	movq	%rdi, 8(%rcx)
	movq	L___unnamed_3(%rip), %r8
	movq	L___unnamed_3+8(%rip), %rdi
	movq	%r8, 32(%rcx)
	movq	%rdi, 40(%rcx)
	movq	%rsi, 16(%rcx)
	movq	%rdx, 24(%rcx)
	addq	$96, %rsp
	popq	%rbp
	retq
LBB3_4:
	jmp	LBB3_2
	.cfi_endproc

	.p2align	4, 0x90
__ZN4core3fmt9Arguments9new_const17hcc12274bc59ac8d6E:
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	subq	$96, %rsp
	movq	%rdx, -80(%rbp)
	movq	%rsi, -72(%rbp)
	movq	%rdi, -64(%rbp)
	movq	%rdi, -56(%rbp)
	cmpq	$1, %rdx
	ja	LBB4_2
	movq	-56(%rbp), %rax
	movq	-64(%rbp), %rcx
	movq	-80(%rbp), %rdx
	movq	-72(%rbp), %rsi
	movq	%rsi, (%rcx)
	movq	%rdx, 8(%rcx)
	movq	L___unnamed_3(%rip), %rsi
	movq	L___unnamed_3+8(%rip), %rdx
	movq	%rsi, 32(%rcx)
	movq	%rdx, 40(%rcx)
	leaq	l___unnamed_4(%rip), %rdx
	movq	%rdx, 16(%rcx)
	movq	$0, 24(%rcx)
	addq	$96, %rsp
	popq	%rbp
	retq
LBB4_2:
	leaq	l___unnamed_2(%rip), %rsi
	leaq	-48(%rbp), %rdi
	movq	%rdi, -88(%rbp)
	movl	$1, %edx
	callq	__ZN4core3fmt9Arguments9new_const17hcc12274bc59ac8d6E
	movq	-88(%rbp), %rdi
	leaq	l___unnamed_6(%rip), %rsi
	callq	__ZN4core9panicking9panic_fmt17h3ed68d73e880d931E
	.cfi_endproc

	.p2align	4, 0x90
__ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h6f224c62185947a3E:
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	subq	$16, %rsp
	movq	(%rdi), %rdi
	callq	__ZN4core3ops8function6FnOnce9call_once17h67dbec3c64e93b5eE
	addq	$16, %rsp
	popq	%rbp
	retq
	.cfi_endproc

	.p2align	4, 0x90
__ZN4core3ops8function6FnOnce9call_once17h2608594c88af5c47E:
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	subq	$16, %rsp
	callq	*%rdi
	addq	$16, %rsp
	popq	%rbp
	retq
	.cfi_endproc

	.p2align	4, 0x90
__ZN4core3ops8function6FnOnce9call_once17h67dbec3c64e93b5eE:
Lfunc_begin0:
	.cfi_startproc
	.cfi_personality 155, _rust_eh_personality
	.cfi_lsda 16, Lexception0
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	subq	$48, %rsp
	movq	%rdi, -32(%rbp)
Ltmp0:
	leaq	-32(%rbp), %rdi
	callq	__ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h3ab922827b5e3e30E
Ltmp1:
	movl	%eax, -36(%rbp)
	jmp	LBB7_3
LBB7_1:
	movq	-16(%rbp), %rdi
	callq	__Unwind_Resume
LBB7_2:
Ltmp2:
	movq	%rax, %rcx
	movl	%edx, %eax
	movq	%rcx, -16(%rbp)
	movl	%eax, -8(%rbp)
	jmp	LBB7_1
LBB7_3:
	movl	-36(%rbp), %eax
	addq	$48, %rsp
	popq	%rbp
	retq
Lfunc_end0:
	.cfi_endproc
	.section	__TEXT,__gcc_except_tab
	.p2align	2, 0x0
GCC_except_table7:
Lexception0:
	.byte	255
	.byte	255
	.byte	1
	.uleb128 Lcst_end0-Lcst_begin0
Lcst_begin0:
	.uleb128 Ltmp0-Lfunc_begin0
	.uleb128 Ltmp1-Ltmp0
	.uleb128 Ltmp2-Lfunc_begin0
	.byte	0
	.uleb128 Ltmp1-Lfunc_begin0
	.uleb128 Lfunc_end0-Ltmp1
	.byte	0
	.byte	0
Lcst_end0:
	.p2align	2, 0x0

	.section	__TEXT,__text,regular,pure_instructions
	.p2align	4, 0x90
__ZN4core3ptr85drop_in_place$LT$std..rt..lang_start$LT$$LP$$RP$$GT$..$u7b$$u7b$closure$u7d$$u7d$$GT$17h0bfa74c1fdc554b0E:
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	popq	%rbp
	retq
	.cfi_endproc

	.p2align	4, 0x90
__ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17hde7c82540c1cf003E:
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	xorl	%eax, %eax
	popq	%rbp
	retq
	.cfi_endproc

	.p2align	4, 0x90
__ZN4main4main17h00968fa4333830e6E:
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	subq	$224, %rsp
	movl	$6, -212(%rbp)
	leaq	-212(%rbp), %rax
	movq	%rax, -16(%rbp)
	movq	__ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$i32$GT$3fmt17hc51d24628782df44E@GOTPCREL(%rip), %rax
	movq	%rax, -8(%rbp)
	movq	-16(%rbp), %rcx
	movq	-8(%rbp), %rax
	movq	%rcx, -160(%rbp)
	movq	%rax, -152(%rbp)
	leaq	-208(%rbp), %rdi
	leaq	l___unnamed_7(%rip), %rsi
	movl	$1, %r8d
	leaq	-160(%rbp), %rcx
	movq	%r8, %rdx
	callq	__ZN4core3fmt9Arguments6new_v117h4231a801aa493fe2E
	leaq	-208(%rbp), %rdi
	callq	__ZN3std2io5stdio6_print17h758ceef33c6b275cE
LBB10_1:
	cmpl	$1, -212(%rbp)
	jne	LBB10_3
	leaq	-80(%rbp), %rdi
	leaq	l___unnamed_8(%rip), %rsi
	movl	$1, %edx
	callq	__ZN4core3fmt9Arguments9new_const17hcc12274bc59ac8d6E
	leaq	-80(%rbp), %rdi
	callq	__ZN3std2io5stdio6_print17h758ceef33c6b275cE
	addq	$224, %rsp
	popq	%rbp
	retq
LBB10_3:
	movl	-212(%rbp), %eax
	movl	%eax, -216(%rbp)
	cmpl	$-2147483648, %eax
	sete	%al
	andb	$0, %al
	testb	$1, %al
	jne	LBB10_5
	movl	-216(%rbp), %eax
	movl	$2, %ecx
	cltd
	idivl	%ecx
	cmpl	$0, %edx
	je	LBB10_6
	jmp	LBB10_7
LBB10_5:
	leaq	l___unnamed_9(%rip), %rdi
	callq	__ZN4core9panicking11panic_const24panic_const_rem_overflow17haf727c9331210b41E
LBB10_6:
	cmpl	$-2147483648, -212(%rbp)
	sete	%al
	andb	$0, %al
	testb	$1, %al
	jne	LBB10_9
	jmp	LBB10_8
LBB10_7:
	movl	$3, %eax
	imull	-212(%rbp), %eax
	movl	%eax, -220(%rbp)
	seto	%al
	testb	$1, %al
	jne	LBB10_12
	jmp	LBB10_11
LBB10_8:
	movl	-212(%rbp), %eax
	movl	$2, %ecx
	cltd
	idivl	%ecx
	movl	%eax, -212(%rbp)
	jmp	LBB10_10
LBB10_9:
	leaq	l___unnamed_10(%rip), %rdi
	callq	__ZN4core9panicking11panic_const24panic_const_div_overflow17he1a2b605b3d23e48E
LBB10_10:
	leaq	-212(%rbp), %rax
	movq	%rax, -32(%rbp)
	movq	__ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$i32$GT$3fmt17hc51d24628782df44E@GOTPCREL(%rip), %rax
	movq	%rax, -24(%rbp)
	movq	-32(%rbp), %rcx
	movq	-24(%rbp), %rax
	movq	%rcx, -96(%rbp)
	movq	%rax, -88(%rbp)
	leaq	-144(%rbp), %rdi
	leaq	l___unnamed_11(%rip), %rsi
	movl	$1, %r8d
	leaq	-96(%rbp), %rcx
	movq	%r8, %rdx
	callq	__ZN4core3fmt9Arguments6new_v117h4231a801aa493fe2E
	leaq	-144(%rbp), %rdi
	callq	__ZN3std2io5stdio6_print17h758ceef33c6b275cE
	jmp	LBB10_1
LBB10_11:
	movl	-220(%rbp), %eax
	incl	%eax
	movl	%eax, -224(%rbp)
	seto	%al
	testb	$1, %al
	jne	LBB10_14
	jmp	LBB10_13
LBB10_12:
	leaq	l___unnamed_12(%rip), %rdi
	callq	__ZN4core9panicking11panic_const24panic_const_mul_overflow17hb38e742c1aafeec6E
LBB10_13:
	movl	-224(%rbp), %eax
	movl	%eax, -212(%rbp)
	jmp	LBB10_10
LBB10_14:
	leaq	l___unnamed_12(%rip), %rdi
	callq	__ZN4core9panicking11panic_const24panic_const_add_overflow17hd921dae81dbfa093E
	.cfi_endproc

	.globl	_main
	.p2align	4, 0x90
_main:
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	movq	%rsi, %rdx
	movslq	%edi, %rsi
	leaq	__ZN4main4main17h00968fa4333830e6E(%rip), %rdi
	xorl	%ecx, %ecx
	callq	__ZN3std2rt10lang_start17hac21861e4ee02960E
	popq	%rbp
	retq
	.cfi_endproc

	.section	__DATA,__const
	.p2align	3, 0x0
l___unnamed_1:
	.quad	__ZN4core3ptr85drop_in_place$LT$std..rt..lang_start$LT$$LP$$RP$$GT$..$u7b$$u7b$closure$u7d$$u7d$$GT$17h0bfa74c1fdc554b0E
	.asciz	"\b\000\000\000\000\000\000\000\b\000\000\000\000\000\000"
	.quad	__ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h6f224c62185947a3E
	.quad	__ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h3ab922827b5e3e30E
	.quad	__ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h3ab922827b5e3e30E

	.section	__TEXT,__literal16,16byte_literals
	.p2align	3, 0x0
L___unnamed_3:
	.space	8
	.space	8

	.section	__TEXT,__const
l___unnamed_13:
	.ascii	"invalid args"

	.section	__DATA,__const
	.p2align	3, 0x0
l___unnamed_2:
	.quad	l___unnamed_13
	.asciz	"\f\000\000\000\000\000\000"

	.section	__TEXT,__const
	.p2align	3, 0x0
l___unnamed_4:
	.byte	0

l___unnamed_14:
	.ascii	"/rustc/a07f3eb43acc5df851e15176c7081a900a30a4d7/library/core/src/fmt/mod.rs"

	.section	__DATA,__const
	.p2align	3, 0x0
l___unnamed_5:
	.quad	l___unnamed_14
	.asciz	"K\000\000\000\000\000\000\000U\001\000\000\r\000\000"

	.p2align	3, 0x0
l___unnamed_6:
	.quad	l___unnamed_14
	.asciz	"K\000\000\000\000\000\000\000K\001\000\000\r\000\000"

	.p2align	3, 0x0
l___unnamed_7:
	.quad	l___unnamed_4
	.space	8

	.section	__TEXT,__const
l___unnamed_15:
	.byte	10

	.section	__DATA,__const
	.p2align	3, 0x0
l___unnamed_8:
	.quad	l___unnamed_15
	.asciz	"\001\000\000\000\000\000\000"

	.section	__TEXT,__const
l___unnamed_16:
	.ascii	"main.rs"

	.section	__DATA,__const
	.p2align	3, 0x0
l___unnamed_9:
	.quad	l___unnamed_16
	.asciz	"\007\000\000\000\000\000\000\000\007\000\000\000\f\000\000"

	.p2align	3, 0x0
l___unnamed_10:
	.quad	l___unnamed_16
	.asciz	"\007\000\000\000\000\000\000\000\t\000\000\000\r\000\000"

	.p2align	3, 0x0
l___unnamed_12:
	.quad	l___unnamed_16
	.asciz	"\007\000\000\000\000\000\000\000\013\000\000\000\021\000\000"

	.section	__TEXT,__literal4,4byte_literals
L___unnamed_17:
	.ascii	" -> "

	.section	__DATA,__const
	.p2align	3, 0x0
l___unnamed_11:
	.quad	L___unnamed_17
	.asciz	"\004\000\000\000\000\000\000"

.subsections_via_symbols
