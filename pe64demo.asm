/*
include <quoted string - file name>

It will insert a text file into the source code. 
It allows you to break up the source code into multiple files. 
Of course, the inserted text will be preprocessed too. 
File path and name should be quoted (enclosed in ',' or ",").

Examples:
```
	include 'file.asm'
	include 'HEADERS\data.inc'
	include '..\lib\strings.asm'
	include 'C:\config.sys'
```
You can also access environment variables enclosed in %,%:
```
	include '%FASMINC%\win32a.inc'
	include '%SYSTEMROOT%\somefile.inc'
	include '%myproject%\headers\something.inc'
	include 'C:\%myprojectdir%\headers\something.inc'
``` */
include 'include.inc'
/*
<name1> equ <name2>
This tells the preprocessor to replace every occurrence of <name1> with <name2>.

Example, the source:

```
	count equ 10  ;this is a preprocessor command
	mov ecx,count
```

is preprocessed to:
```
	mov ecx,10
```

Example:
```
	mov eax,count
	count equ 10
	mov ecx,count
```

is preprocessed to:

```
	mov eax,count
	mov ecx,10
```

because the preprocessor only replaces occurrences of count that come after equ.
Even this works:

```
	10 equ 11
	mov ecx,10
```

after preprocessing, it becomes:
```
	mov ecx,11
```

Bare in mind that name1 can be any symbol. 
A symbol is just a set of chars, terminated by a blank character 
(space, tab, end of line), a comment (;), a line-break (\) or an operator 
(including assembly-time operators, not just preprocessor operators), 
but it can’t be an operator or a special symbol (like , or }, etc.)

name2 can be anything, not just one symbol:
everything up to end of line is taken. It can even be empty, then <name1> is replaced 
with a blank space.

Example:
```
	10 equ 11,12,13
	db 10
```
becomes:
```
	db 11,12,13
```
*/

format PE64 GUI
entry start
/*
You can create your own instruction/directive using “macro”:

macro <name>
{
  <body>
}

When the preprocessor encounters a macro directive it defines a macro, 
which means that each following occurence of a line starting with <name> will be replaced
by <body>. 

<name> must be a single symbol, <body> can be anything except } 
which denotes end of macro body.

Example:
```
	macro a
	{
	  push eax
	}
	xor eax,eax
	a
```
becomes:
```
	xor eax,eax
	push eax
```
Example:
```
	macro a
	{
	  push eax
	}
	macro b
	{
	  push ebx
	}
	b
	a
```
becomes:
```
	push ebx
	push eax
```
Of course, a macro doesn’t have to be indented like in my examples, 
you could also write it this way:
```
	macro push5 {push dword 5}
	push5
```
which becomes:
```
	push dword 5
```
Or even like this:
```
	macro push5 {push dword 5
	}
```
producing the same result. You have freedom of style reagarding macros indentation.

You can nest macros. If you redefine a macro, then the last definition is used. 
But if you use the original macro inside the last one, it will still work. 
Example:
```
	macro a {mov ax,5}
	macro a
	{
	  a
	  mov bx,5
	}
	macro a
	{
	  a
	  mov cx,5
	}
	a
```
becomes:
```
	mov ax,5
	mov bx,5
	mov cx,5
```
Example:
```
	macro a {1}
	a

	macro a {
	  a
	  2}

	a
	macro a {
	  a
	  3}

	a
```
becomes:
```
	1

	1
	2

	1
	2
	3
```

A macro’s name will be replaced by the macro’s body not just when the line starts 
with the macro, but in every place where an instruction mnemonic (like add, mov) 
is accepted. 

It is because the main purpose of macros is to simulate instructions. 
The only exception is instruction prefix: a macro is not accepted after an instruction 
prefix.

You can also “overload” instructions with macros. 
Since the preprocessor isn’t aware of instructions, it allows macro names to be 
instruction mnemonics:
```
macro pusha
{
  push eax ebx ecx edx ebp esi edi
}

macro popa
{
  pop edi esi ebp edx ecx ebx eax
}
```
These 2 save 4 bytes for every pusha because they don’t push ESP. 
But overloading istructions isn’t a very good idea, because someone reading your code 
may be fooled if he doesn’t know that the instruction is overloaded.
*/
macro pusha
{
  push rax ; ebx ecx edx ebp esi edi
}

macro popa
{
  pop rax ;esi ebp edx ecx ebx eax
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
/*
Preprocessor Conditionals
In fact, there is no preprocessor conditional syntax in FASM (too bad).
But the assembly directive if can be used in conjuction with the preprocessor to 
achieve the same results as with preprocessor conditionals 
(but this way it wastes more time and memory).

if is an assembly-time statement. It means that the statement is checked after 
preprocessing, and this allows some special conditional operators to work.

I won’t describe its assembly-time behavior (conditional operators like &, |, etc), 
read FASM’s docs for this. 
Here I will describe only those operators that are used with the preprocessor.

if predicate EQ condition

EQ compares two symbols and checks if they are the same. 
Value of abcd eq abcd is true, value of abcd eq 1 is false, etc. 
It is useful for comparing a symbol that will be preprocessed, like:
```
	STRINGS equ ASCII
	if STRINGS eq ASCII
	  db 'Oh yeah',0
	else if STRINGS eq UNICODE
	  du 'Oh yeah',0
	else
	  display 'unknown string type'
	end if
```
after preprocessing it will become:
```
	if ASCII eq ASCII
	  db 'Oh yeah',0
	else if ASCII eq UNICODE
	  du 'Oh yeah',0
	else
	  display 'unknown string type'
	end if
```
where the first condition is true, so only db 'Oh yeah',0 will be assembled.

Example:
```
	STRINGS equ UNICODE   ;only difference here, UNICODE instead of ASCII
	if STRINGS eq ASCII
	  db 'Oh yeah',0
	else if STRINGS eq UNICODE
	  du 'Oh yeah',0
	else
	  display 'unknown string type'
	end if
```
after preprocessing it will be:
```
	if UNICODE eq ASCII
	  db 'Oh yeah',0
	else if UNICODE eq UNICODE
	  du 'Oh yeah',0
	else
	  display 'unknown string type'
	end if
```
now the first condition (UNICODE eq ASCII) will be false, and the second one 
(UNICODE eq UNICODE) will be true, therefore du 'Oh yeah',0 will be assembled.

A better usage of this operator is for checking macro arguments, like:
```
	macro item type,value
	{
	  if type eq BYTE
		db value
	  else if type eq WORD
		dw value
	  else if type eq DWORD
		dd value
	  else if type eq STRING
		db value,0
	  end if
	}
	item BYTE,1
	item STRING,'aaaaaa'
```
which becomes:
```
	if BYTE eq BYTE
	  db 1
	else if BYTE eq WORD
	  dw 1
	else if BYTE eq DWORD
	  dd 1
	else if BYTE eq STRING
	  db 1,0
	end if
	if STRING eq BYTE
	  db 'aaaaaa'
	else if STRING eq WORD
	  dw 'aaaaaa'
	else if STRING eq DWORD
	  dd 'aaaaaa'
	else if STRING eq STRING
	  db 'aaaaaa',0
	end if
```
so only these two commands will get assembled:
```
	db 1
	db 'aaaaaa',0
```
eq (like all other preprocessor operators) can also work with empty arguments. 

This means, for example, that if eq is true, and if 5 eq is false etc. Example macro:

macro moves dest,src,src2
{
  if src2 eq
    mov dest,src
  else
    mov dest,src
    mov src,src2
  end if
}

Another operator is eqtype.
It compares whether symbols are of the same type. 
Types are:
	
	individual quoted strings 
	(those not being a part of numerical expression)
	
	floating point numbers
	any numerical expression 
	(note that any unknown word will be treated as a label, so it will also will be seen as an expression),
	
	addresses - the numerical expressions in square brackets 
	(with size operators and segment prefixes)
	
	instruction mnemonics
	registers
	size operators
	near/far operators,
	use16/use32 operators
	blank space
	
Example of a macro which allows an SHL instruction with a memory variable as count, 
like shl ax,[myvar]:
```
	macro shl dest,count
	{
	  if count eqtype [0]   ;if count is a memory variable
		push cx
		mov cl,count
		shl dest,cl
		pop cx
	  else                  ;if count is of another type
		shl dest,count      ;just use original shl
	  end if
	}

	shl ax,5
	byte_variable db 5
	shl ax,[byte_variable]
```
becomes:
```
	if 5 eqtype [0]
	  push cx
	  mov cl,5
	  shl ax,cl
	  pop cx
	else
	  shl ax,5
	end if
	byte_variable db 5
	if [byte_variable] eqtype [0]
	  push cx
	  mov cl,[byte_variable]
	  shl ax,cl
	  pop cx
	else
	  shl ax,[byte_variable]
	end if
```
and so, because of the conditionals, it will be assembled to:
```
	shl ax,5
	byte_variable db 5
	push cx
	mov cl,[byte variable]
	shl ax,cl
	pop cx
```
Note that shl ax,byte [variable] wouldn’t work with this macro, because condition byte 
[variable] eqtype [0] isn’t true (read further). 

The eqtype operator isn’t limited to just two operands. 
It just compares whether types of operands on its left-hand side and same to 
type of operands on right side of eqtype. 

For example, if eax 4 eqtype ebx name is true 
(name is a label, and thus it is a number too).

Example of extending the mov intruction so it allows moving between memory variables:
```
	macro mov dest,src
	{
	  if dest src eqtype [0] [0]
		push src
		pop dest
	  else
		mov dest,src
	  end if
	}
	mov [var1],5
	mov [var1],[var2]
```
will be preprocessed to:
```
	if [var1] 5 eqtype [0] [0]  ;false
	  push 5
	  pop [var1]
	else
	  mov [var1],5
	end if
	if [var1] [var2] eqtype [0] [0]  ;true
	  push [var2]
	  pop [var1]
	else
	  mov [var1],[var2]
	end if
```
and assembled to:
```
	mov [var1],5
	push [var2]
	pop [var1]
```
Anyway, a better (and more readable) way to write a similar macro is to use the & 
operator (not covered in this document, see FASM documentation), like:
```
	macro mov dest,src
	{
	  if (dest eqtype [0]) & (src eqtype [0])
		push src
		pop dest
	  else
		mov dest,src
	  end if
	}
```
The previous example using eqtype with four arguments was meant only to demonstrate its 
possibilities, but & should be used if possible.

Note that currently you can use incomplete expressions as argument of eqtype, 
it is sufficent that the parser recognizes its type, but this is undocumented behavior 
so I won’t describe it any further.


FASM also includes another operator which can be employed if you use multiple eqs:
```
	macro mov a,b
	{
	  if (a eq cs) | (a eq ds) | (a eq es) | (a eq fs) |\
		 (a eq gs) | (a eq ss)
		push b
		pop a
	  else
		mov a,b
	  end if
	}
```
Instead of many |ed eqs, you can use the in operator. 
It compares the symbol on its left-hand side with multiple symbols in a list on its 
right-hand side. The symbols list must be enclosed in angle brackets (< and >), 
and the symbols inside the list should be separated by commas (,).

macro mov a,b
{
  if a in <cs,ds,es,fs,gs,ss>
    push b
    pop a
  else
    mov a,b
  end if
}
in also works with multiple symbols on both sides (like eq):

if dword [eax] in <[eax], dword [eax], ptr eax, dword ptr eax>

Structures 

Structures are almost the same as macros. You declare them with the struc directive:

struc <name> <arguments> { <body> }
The difference is that when you use a structure in code, 
it must be preceded by a label (structure name).

Example:
```
	struc a {db 5}
	a
```
doesn’t work. 
A structure is only recognized when preceded by name, like:
```
	struc a {db 5}
	name a
```
which, like a macro, will get preprocessed to:
```
	db 5
```
The reason for the preceding name (ie: the one before the structure) 
is that name will be appended before every symbol inside the structure’s body that 
starts with a ..
Example:
```
	struc a {.local:}
	name1 a
	name2 a
```
will become:
```
	name1.local:
	name2.local:
```
This way you can define something similar to structures found in other languages. 
Example:
```
struc rect left,right,top,bottom  ;has arguments, like macros
{
  .left dd left
  .right dd right
  .top dd top
  .bottom dd bottom
}

r1 rect 0,20,10,30
r2 rect ?,?,?,?
```
becomes:
```
r1.left dd 0
r1.right dd 20
r1.top dd 10
r1.bottom dd 30
r2.left dd ?
r2.right dd ?
r2.top dd ?
r2.bottom dd ?
```
You can also use a cool trick with which you don’t have to specify arguments 
(and 0 will be used instead):
```
struc dummy arg
{
  .member dd arg+0
}
y1 dummy 0xACDC
y2 dummy
```
becomes:
```
y1.member dd 0xACDC+0
y2.member dd +0
```
If an argument remain unspecified its value is blank inside macro/structure body.
We also exploited the fact that + is both a binary (with two operands) and unary 
(with one operand) operator.

You'll often encounter a defined macro or structure called `struct` (not `struc`), 
which declares a structure or extends structure declaration. 
Don’t mistake that `struct` with `struc`.

Fixes
By the time FASM was evolving, it still missed one very useful feature: 
the ability to declare a macro inside a macro — ie: 
the result of unrolling the macro becomes a macro definition. 

Hypothetically, something like this:
```
	macro declare_macro_AAA
	{
	  macro AAA
	  {
		 db 'AAA',0
	  } ;end of "AAA" declaration
	} ;end of "declare_macro_AAA" declaration
```
The problem here is that when macro declare_macro_AAA is read by the preprocessor, 
the first } encountered is interpreted as its end, which isn’t what we intended. 
It is similar to what happens with other preprocessor symbols/operators 
(eg: #, `, forward, local, etc.), they get processed during expansion of the outer macro, 
so they can’t be used in inner macro declaration.

In the meantime, another preprocessor directive was added. It does the same job as equ, 
but BEFORE other preprocessing (except for things listed in Chapter 2, which are done in 
a pre-preprocessing stage, but this is internal stuff, not particularly interesting). 
This directive is fix.

It has the same syntax as equ (<symbol> fix <anything>), but replacing fixed symbols in
line is done before any other preprocessing (except things listed in Chapter 2, again). 
Preprocessing is done line by line, left to right, so if we have the following code:
```
	a equ 1
	b equ a
	a b
```
its preprocessing will happens like this:

Preprocessing line 1:
a — Preprocessor finds unknown word, skips it.
equ — “equ” is second word of line, so it memorizes that “a” equals rest of line (“1”), then deletes line.
Preprocessing line 2:
b — Preprocessor finds unknown word, skips it.
equ — “equ” is second word of line, so it memorizes that “b” equals rest of line (“a”), then deletes line.
Preprocessing line 3:
a — Preprocessor replaces “a” with “1”
b — Preprocessor replaces “b” with “a”
So it becomes:
```
	1 a
```
But if we have:
```
	a fix 1
	b fix a
	a b
```
then it looks like:
```
	a — Preprocessor finds unknown word, skips it.
	fix — “fix” is second word of line, so it memorizes that “a” is fixed to rest of line (“1”), then deletes line.
	Fixing line 2: “a” is fixed to “1”, so line becomes “b fix 1”
	Preprocessing line 2:
	b — Preprocessor finds unknown word, skips it.
	fix — “fix” is second word of line, so it memorizes that “b” is fixed to rest of line (“1”) and deletes line
	Fixing line 3: “a” is fixed to “1”, “b” is fixed to “1” so line becomes “1 1”
	Preprocessing line 3:
	1 — Preprocessor finds unknown word, skips it.
	1 — Preprocessor finds unknown word, skips it.
```
This was only an example to show how fixing works, it isn’t usually used in this manner.

9.2. Using Fixes for Nested Macro Declaration
Now let’s get back to declaring a macro inside a macro. 
First of all, we need to know how macros are preprocessed. 

You can quite easily work it out yourself: at macro declaration the macro’s body is saved,
and when a macro is expanded the preprocessor replaces the line containing the macro 
with that macro’s body, it internally declares equates to handle its arguments and
then continues preprocessing the macro body. 
(of course it’s more complicated than this, but this is enough for understanding fixes).

So what was the problem with declaring a macro inside a macro? 
The first time the compiler encountered a “}” inside the macro’s body it
interpreted it as the end of the macro’s body declaration, so there wasn’t any way to 
include “}” in a macro’s body. But we can easily fix :) this:
```
	macro a
	{
	  macro b
	  %_
		 display 'Never fix before something really needs to be fixed'
	  _%
	}
	%_ fix {
	_% fix }
	a
	b
```
Now preprocessing looks like this (simplified):

Preprocessor loads declaration of macro “a”
Preprocessor loads declaration of fixes “%_” and “_%”
Preprocessor expands macro “a”
Preprocessor loads macro “b” declaration (“_%” and “%_” are fixed in each line before being handled by rest of preprocessor)
Preprocessor expands macro “b”
Here you can see how important is the positioning of fixes’ declaration, 
because the macro’s body is fixed too before being loaded by the preprocessor. 
For example, this won’t work:
```
	%_ fix {
	_% fix }
	macro a
	{
	  macro b
	  %_
		 display 'Never fix before something really needs to be fixed, here you see it'
	  _%
	}
	a
	b
```
Because “%_” and “_%” will be fixed before loading macro “a”, 
so loading of the macro’s body will end at “_%” (by then, fixed to “}”)
and the second “}” will remain there.

**NOTE**: Character “`%`” isn’t a special character for FASM’s preprocessor, 
so you use it just like any other normal character (eg: “`a`” or “`9`”).
It has special meaning AFTER preprocessing, and only when it is the only char of a whole 
word (eg: “`%`” but not “`anything%anything`”).
We also need to fix other macro-releated operators:
```
	%_ fix {
	_% fix }
	%local fix local
	%forward fix forward
	%reverse fix revese
	%common fix common
	%tostring fix `
```
Only # is a special case, you can fix it, but there is an easier way. 
Every time the preprocessor finds multiple #s, it removes one, so it is something like 
(this won’t actually work):
```
	###### fix #####
	#####  fix ####
	####   fix ###
	###    fix ##
	##     fix #
```
So instead of using symbol fixed to “#” you can just use “##” etc.

9.3. Using Fixes for Moving Parts of Codes
You can also use fixes to move parts of code. 
In assembly programming this is useful, especially when you break code into 
modules but want to have data and code grouped in separate segment/section, 
but defined in a single file.

Right now this part of tutorial is TODO, I hope I will write it soon, 
for now you can look at JohnFound’s Fresh’s macro library,
file INCLUDE\MACRO\globals.inc.

I know fixes are confusing, and to understand them you have to learn the inner workings
of the preprocessor, but they give you great coding power. 
Privalov wanted FASM to be as powerful as possible, even at the cost of
comprehensibility.

Closing Remarks
Don’t forget to read FASM documentation. 
Almost everything from this tutorial is there, maybe written in a way that’s a little 
harder for learning but definitely a better reference. It is not so long, nor hard to 
remember — 99% of FASM users have learnt it from these docs and from the forum.
*/
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
  if count eqtype [0]   ;if count is a memory variable
    push cx
    mov cl,count
    shl dest,cl
    pop cx
  else                  ;if count is of another type
    shl dest,count      ;just use original shl
  end if
}

macro tmacro [params]
 {
  common macro params {
 }
MACRO fix tmacro
ENDM fix }


MACRO stoschar char
	mov al,char
	stosb
ENDM

section '.text' code readable executable
start:
	sub	rsp,8*5 	; reserve stack for API use and make stack dqword aligned
	count equ 0
	/*
	restore <name>
	You can also tell the preprocessor to stop replacing a particular equate.
	From this command onward, <name> will no longer be replaced as previously 
	specified by equ.
	
	Example:
	```
		mov eax,count
		count equ 10
		mov eax,count
		restore count
		mov eax,count
	```
	becomes:
	```
		mov eax,count
		mov eax,10
		mov eax,count
	```
	Note that replacements are “stacked”;
	this means that if you define two equates for one symbol, and then restore it (once),
	the first one will be used.

	Example:
	```
		mov eax,count
		count equ 1
		mov eax,count
		count equ 2
		mov eax,count
		count equ 3
		mov eax,count
		restore count
		mov eax,count
		restore count
		mov eax,count
		restore count
		mov eax,count
	```
	becomes:
	```
		mov eax,count
		mov eax,1
		mov eax,2
		mov eax,3
		mov eax,2
		mov eax,1
		mov eax,count
	```
	If you try to restore a non-existing equation nothing will happen.

	Example:
	```
		mov eax,count
		restore count
		mov eax,count
	```
	becomes:
	```
		mov eax,count
		mov eax,count
	```
	*/
	mov	r9d, count	
	restore count
		
	lea	r8,[_caption]
	lea	rdx,[_message]
	;mov	rcx,0
	moves rcx, 0
	mover rcx, 0
	
	call	[MessageBoxA]
	shift ax, 5
	
	pusha
	popa
	pushstr 'line1', 13, 10, 'line2', 'line2', 'line2', 'line2' 
	
	/*
	jmp near dword [0]
    jmp far dword [0]

    mov bx,ax       ; general register to general register
    mov [char],al   ; general register to memory
    mov dl,32       ; immediate value to general register
    mov cr3,ebx     ; general register to control register

    xchg ax,bx      ; swap two general registers
    xchg al,[char]  ; swap register with memory

    push ax         ; store general register
    push es         ; store segment register
    pushw [bx]      ; store memory
    push 1000h      ; store immediate value

    imul ax,[si],10 ; memory by immediate value to register

    bt  ax,15        ; test bit in register
    bts word [bx],15 ; test and set bit in memory

    bswap edx        ; swap bytes in register

    jmp 100h         ; direct near jump
    jmp 0FFFFh:0     ; direct far jump
    jmp ax           ; indirect near jump
    jmp pword [ebx]  ; indirect far jump

    movs byte [di],[si]        ; transfer byte
    movs word [es:di],[ss:si]  ; transfer word
    movsd                      ; transfer double word

    cmpxchg8b [bx]   ; compare and exchange 8 bytes

    movq2dq xmm0,mm1   ; move from MMX register to SSE register
    movdq2q mm0,xmm1   ; move from SSE register to MMX register

    enter 2048,0     ; enter and allocate 2048 bytes on stack

    mov [rip+3],sil    ; manual RIP-relative addressing

    blendvps xmm3,xmm7,xmm0 ; blend according to mask
    vgatherqps xmm0,[xmm2],xmm3        ; gather two floats
    vgatherqps xmm0,[ymm2+64],xmm3     ; gather four floats
    vfmsub231ps ymm1,ymm2,ymm3     ; multiply and subtract
    vfnmadd132sd xmm0,xmm5,[ebx]   ; multiply, negate and add
    vpermil2ps ymm0,ymm3,ymm7,ymm2,0  ; permute from two sources

    vscatterdps [eax+xmm1]{k1},xmm0    ; scatter four floats
    vscatterdpd [ymm3*8]{k3},zmm0      ; scatter eight doubles

    dd sum
    x = 1
    x = x+2
    sum = x


    if count>0
        mov cx,count
        rep movsb
    end if


    if count & ~ count mod 4
        mov cx,count/4
        rep movsd
    else if count>4
        mov cx,count/4
        rep movsd
        mov cx,count mod 4
        rep movsb
    else
        mov cx,count
        rep movsb
    end if

    repeat 8
        mov byte [bx],%
        inc bx
    end repeat


    s = x/2
    repeat 100
        if x/s = s
            break
        end if
        s = (s+x/s)/2
    end repeat

    repeat $-$$
        load a byte from $$+%-1
        store byte a xor c at $$+%-1
    end repeat

    GDTR dp ?
    virtual at GDTR
        GDT_limit dw ?
        GDT_address dd ?
    end virtual

    virtual at 0
        file 'a.txt':10h,1
        load char from 0
    end virtual

    virtual at 0 as 'asc'
        times 256 db %-1
    end virtual

    virtual at 0
        hex_digits::
        db '0123456789ABCDEF'
    end virtual
    load a byte from hex_digits:10

    bits = 16
    display 'Current offset is 0x'
    repeat bits/4
        d = '0' + $ shr (bits-%*4) and 0Fh
        if d > '9'
            d = d + 'A'-'9'-1
        end if
        display d
    end repeat
    display 13,10

    if ~ defined alpha
        alpha:
    end if

    if ~ defined alpha | defined @f
        alpha:
        @@:
    end if

    include 'macros.inc'

    d equ dword
    NULL equ d 0
    d equ edx

    d equ d,eax

    b equ byte
    w equ word
    d equ dword
    p equ pword
    f equ fword
    q equ qword
    t equ tword
    x equ dqword
    y equ qqword

    incl fix include

    macro tst {test al,0xFF}

    macro stos0
     {
        xor al,al
        stosb
     }

    macro align value { rb (value-1)-($+value-1) mod value }

    macro mov op1,op2
     {
      if op1 in <ds,es,fs,gs,ss> & op2 in <cs,ds,es,fs,gs,ss>
        push  op2
        pop   op1
      else
        mov   op1,op2
      end if
     }

    macro stoschar [char]
     {
        mov al,char
        stosb
     }

    macro movstr
     {
        local move
      move:
        lodsb
        stosb
        test al,al
        jnz move
     }

    macro strtbl name,[string]
     {
      common
        label name dword
      forward
        local label
        dd label
      forward
        label db string,0
     }

    push 3
    push 2
    push 1
    call foo

    macro invoke proc,[arg]
     { common stdcall [proc],arg }

    macro jif op1,cond,op2,label
     {
        cmp op1,op2
        j#cond label
     }

    macro label name
     {
        label name
        if ~ used name
          display `name # " is defined but not used.",13,10
        end if
     }

    macro message arg
     {
      if arg eqtype ""
        local str
        jmp   @f
        str   db arg,0Dh,0Ah,24h
        @@:
        mov   dx,str
      else
        mov   dx,arg
      end if
        mov   ah,9
        int   21h
     }

    macro ext instr
     {
      macro instr op1,op2,op3
       \{
        if op3 eq
          instr op1,op2
        else
          instr op1,op2
          instr op2,op3
        end if
       \}
     }

    ext add
    ext sub


    macro tmacro [params]
     {
      common macro params {
     }
    MACRO fix tmacro
    ENDM fix }


    MACRO stoschar char
        mov al,char
        stosb
    ENDM

    postpone
     {
      code_size = $
     }

    struc point x,y
     {
        .x dw x
        .y dw y
     }

    struc db [data]
     {
       common
        . db data
        .size = $ - .
     }

    rept 5 { in al,dx }

    rept 3 counter
     {
        byte#counter db counter
     }

    match +,+ { include 'first.inc' }
    match +,- { include 'second.inc' }
    match a b, 1+2+3 { db a }

    V fix {
      macro empty
       V
    V fix }
       V


    list equ

    macro append item
     {
       match any, list \{ list equ list,item \}
       match , list \{ list equ item \}
     }

    define a b+4
    define b 3
    rept 1 result:a*b+2 { define c result }

    rept 8 n:0 { pxor xmm#n,xmm#n }


   irps reg, al bx ecx
    { xor reg,reg }


    if 0
    a = 1
    b equ 2
    end if
    dd b


    extrn exit
    extrn '__imp__MessageBoxA@16' as MessageBox:dword

    extrn 'printf' as _printf
    printf = PLT _printf

    tester? = 0

        space:
        space.x = 1
        space.y = 2
        space.color:
        space.color.r = 0
        space.color.g = 0
        space.color.b = 0

        space:
        namespace space
                x = 1
                y = 2
                color:
                .r = 0
                .g = 0
                .b = 0
        end namespace


        first:
                .child = 1
                ..other = 0
        second:
                .child = 2
                ..another = ..other


        label character:byte
        label char:1

        byte? = 1       ; 8 bits
        word? = 2       ; 16 bits
        dword? = 4      ; 32 bits
        fword? = 6      ; 48 bits
        pword? = 6      ; 48 bits
        qword? = 8      ; 64 bits
        tbyte? = 10     ; 80 bits
        tword? = 10     ; 80 bits
        dqword? = 16    ; 128 bits
        xword? = 16     ; 128 bits
        qqword? = 32    ; 256 bits
        yword? = 32     ; 256 bits
        dqqword? = 64   ; 512 bits
        zword? = 64     ; 512 bits

        element A
        linpoly = A + A + 3
        vterm = linpoly scale 1 * linpoly element 1     ; vterm = 2 * A

        db 4 dup 90h            ; generate 4 bytes
        db 2 dup ('abc',10)     ; generate 8 bytes

        macro measured name,string
                local top
                name db string
                top: name.length = top - name
        end macro

        measured hello, 'Hello!'        ; hello.length = 6

        A equ 1
        A equ 2

        drop A
        drop A

        data1 dw 1
        buffer1 rb 10h          ; zeroed and present in the output

        org 400h
        data dw 2
        buffer2 rb 20h          ; not in the output

        section 1000h
        data3 dw 3
        buffer3 rb 30h          ; not in the output

        virtual at 0
                hex_digits::
                db '0123456789ABCDEF'
        end virtual
        load a:byte from hex_digits:10  ; a = 'A'

        db "Text"
        key = 7Bh
        repeat $-$$
                load a : byte from $$+%-1
                store a xor key : byte at $$+%-1
        end repeat

        load char : byte from const:0

        if $>10000h
                err 'segment too large'
        end if
        calminstruction please? cmd&
                match =do? =not? cmd, cmd
                jyes done
                assemble cmd
            done:
        end calminstruction

        please do not display 'Bye!'

        macro jmpi target
                if target-($+2) < 80h & target-($+2) >= -80h                    
                        db 0EBh
                        db target-($+1)
                else
                        db 0E9h
                        dw target-($+2)
                end if 
        end macro

        macro EX? first,second
                match (=SP?), first
                        match =HL?, second
                                db 0E3h
                        else match =IX?, second
                                db 0DDh,0E3h
                        else match =IY?, second
                                db 0FDh,0E3h
                        else
                                err "incorrect second argument"
                        end match
                else match =AF?, first
                        match =AF'?, second
                                db 08h
                        else
                                err "incorrect second argument"
                        end match
                else match =DE?, first
                        match =HL?, second
                                db 0EBh
                        else
                                err "incorrect second argument"
                        end match
                else
                        err "incorrect first argument"
                end match
        end macro

        EX (SP),HL

        macro INC? argument
                match [:r:], argument
                        db 100b + r shl 3
                else match (=HL?), argument
                        db 34h
                else match (=IX?+d), argument
                        db 0DDh,34h,d
                else match (=IY?+d), argument
                        db 0FDh,34h,d
                else
                        err "incorrect argument"
                end match
        end macro

        INC (IX+2)

        element IY? 

        element L? : register + 101b

        macro CALL? arguments&
                local cc,nn
                match condition =, target, arguments
                        cc = condition - CC
                        nn = target
                        db 0C4h + cc shl 3
                else
                        nn = arguments
                        db 0CDh                     
                end match
                dw nn
        end macro
	*/
.a:
	mov	ecx,eax
	call	[ExitProcess]

section '.data' data readable writeable

  _caption db 'Win64 assembly program',0
  _message db 'Hello World!',0
  define_bytes <1,1>,2

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
1506 */
