main:
	push 0.0
loop:
	dup 0
	push 100.0
	cmp >=
	pop 1
	cmp !
	ret
	dup 0
	push 15.0
	op %
	push 0.0
	cmp !=
	jmp .f1
	push 'z'
	push 'z'
	push 'u'
	push 'B'
	push 'z'
	push 'z'
	push 'i'
	push 'F'
	outc
	outc
	outc
	outc
	outc
	outc
	outc
	outc
	jmp .join
.f1:
	dup 0
	push 3.0
	op %
	push 0.0
	cmp !=
	jmp .f2
	push 'z'
	push 'z'
	push 'i'
	push 'F'
	outc
	outc
	outc
	outc
	jmp .join
.f2:
	dup 0
	push 5.0
	op %
	push 0.0
	cmp !=
	jmp .f3
	push 'z'
	push 'z'
	push 'u'
	push 'B'
	outc
	outc
	outc
	outc
	jmp .join
.f3:
	dup 0
	out
.join:
	push 10
	outc
	op ++
	jmp loop
