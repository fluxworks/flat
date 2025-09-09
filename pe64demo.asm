format PE64 GUI
entry start
include 'include.inc'
macro pusha
{
  push rax
}

macro popa
{
  pop rax
}

macro define_a_byte arg { db arg }

macro define_bytes arg1,arg2 { define_a_byte <arg1,arg2,3>}

macro pushstr string, [grp1, grp2]
{
	common
		db string
		db grp1, 0
		
	forward
	  db string
	  db grp2
		
	reverse
	  db string
	  db grp1
	
	local ..behind
		call ..behind
		db 'name: '#string,0
		..behind:
}
/*
when | Compare :al: with ( from )  jump ( to ) when equal. */
macro when from, to
{
	cmp al, from
	je to
}

macro test
{
	
	display 'a'
} 

purge test

macro moves dest,src,src2
{
  if src2 eq
    mov dest,src
  else
    mov dest,src
    mov src,src2
  end if
}

macro mover a,b
{
  if a in <cs,ds,es,fs,gs,ss>
    push b
    pop a
  else
    mov a,b
  end if
}

macro shift dest,count
{
  if count eqtype [0]
    push cx
    mov cl,count
    shl dest,cl
    pop cx
  else
    shl dest, count
  end if
}

section '.text' code readable executable
start:
	sub	rsp, 8 * 5
	count equ 0
	mov	r9d, count	
	restore count
		
	lea	r8,[_caption]
	lea	rdx,[_message]
	moves rcx, 0
	mover rcx, 0
	
	call	[MessageBoxA]
	shift ax, 5
	
	pusha
	popa
	pushstr( 'line1', 13, ( 5*2 ), 'line2', 'line2', 'line2', 'line2' )
.a:
	mov	ecx,eax
	call	[ExitProcess]

section '.data' data readable writeable

  _caption db 'Win64 assembly program',0
  _message db 'Hello World!',0
  define_bytes( <1,1>, 2 )

section '.idata' import data readable writeable
	triplet equ 0, 0, 0
	dd triplet, RVA kernel_name, RVA kernel_table
	dd 0,0,0, RVA user_name,RVA user_table
	dd 0,0,0,0,0

  kernel_table:
    ExitProcess dq RVA _ExitProcess
    dq 0
  user_table:
    MessageBoxA dq RVA _MessageBoxA
    dq 0

  kernel_name db 'KERNEL32.DLL',0
  user_name db 'USER32.DLL',0

  _ExitProcess dw 0
    db 'ExitProcess',0
  _MessageBoxA dw 0
    db 'MessageBoxA',0
	
final
{
	code_size = $
}
/*
139 */
