;
fizzbuzz_main:
        push 1.0
.loop:   dup 0
        push 100.0
        cmp >
        pop 1
        cmp !
        ret
        dup 0
        push 15.0
        op %
        push 0.0
        cmp !=
        jmp .f1
        call .fizz
        call .buzz
        jmp .join
.f1:    dup 0
        push 3.0
        op %
        push 0.0
        cmp !=
        jmp .f2
        call .fizz
        jmp .join
.f2:    dup 0
        push 5.0
        op %
        push 0.0
        cmp !=
        jmp .f3
        call .buzz
        jmp .join
.f3:    dup 0
        call .num
.join:  push 10
        out
        op ++
        jmp .loop
.fizz:  push 'F'
        out
        push 'i'
        out
        push 'z'
        out
        push 'z'
        out
        ret
.buzz:  push 'B'
        out
        push 'u'
        out
        push 'z'
        out
        push 'z'
        out
        ret
.num:   push 0 ; end marker
..loop: dup 1
        push 10
        op /
        op floor
        dup 0
        push 0
        cmp !=
        nop
        dup 2
        push 10
        op %
        push '0'
        op +
        cmp !
        jmp ..loop
        outs
        pop 1
        ret
