; truth machine
truth_main:
        in
        dup 0
        push 0
        cmp =
        jmp .eq0
        push 1
        cmp =
        jmp .eq1
        push 'I'
        out
        push 'n'
        out
        push 'v'
        out
        push 'a'
        out
        push 'l'
        out
        push 'i'
        out
        push 'd'
        out
        push 10
        out
        ret
.eq0:   push '0'
        out
        pop 1
        ret
.eq1:   push '1'
        out
        jmp .eq1
