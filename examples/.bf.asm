; interpreter - nonworking
bf_main:
        push 0
        push 10
        push 'd'
        push 'n'
        push 'e'
        push ':'
        push '1'
        push '-'
        push ' '
        push ','
        push '7'
        push ' '
        push '.'
        push '6'
        push ' '
        push ']'
        push '5'
        push ' '
        push '['
        push '4'
        push ' '
        push '>'
        push '3'
        push ' '
        push '<'
        push '2'
        push ' '
        push '-'
        push '1'
        push ' '
        push '+'
        push '0'
        outs
        push 8
        memw
.input: push 0
        memr
        in
        memw
        push 0
        memr
        memr
        dup 0
        push -1
        cmp =
        jmp ..end
        dup 0
        push 4
        cmp =
        jmp ..open
        dup 0
        push 5
        cmp =
        jmp ..close
        pop 1
..join: push 0
        push 0
        memr
        op ++
        memw
        jmp .input
..open:
        pop 1
        push 0
        memr
        jmp ..join
..close:
        push 0
        memr
        dup 2
        op 1-
        memw
        dup 1
        push 0
        memr
        memw
        pop 2
        jmp ..join
..end:  pop 1
        push 1
        push 8
        memw
        push 2
        push 8
        memw
.eval:  push 1
        memr
        memr
        op floor
        dup 0
        push 0
        cmp =
        jmp ..add
        dup 0
        push 1
        cmp =
        jmp ..sub
        dup 0
        push 2
        cmp =
        jmp ..left
        dup 0
        push 3
        cmp =
        jmp ..right
        dup 0
        push 6
        cmp =
        jmp ..out
        dup 0
        push 7
        cmp =
        jmp ..in
        dup 0
        push 0
        cmp <
        jmp ..close
        jmp ..open
..add:  push 2
        memr
        push 2
        memr
        memr
        op floor
        push 2
        memr
        memr
        push 0.00390625
        op +
        push 1
        op %
        op +
        memw
        jmp ..join
..sub:  push 2
        memr
        push 2
        memr
        memr
        op floor
        push 2
        memr
        memr
        push 0.00390625
        op -
        push 1
        op %
        op +
        memw
        jmp ..join
..left: push 2
        push 2
        memr
        op --
        memw
        jmp ..join
..right:
        push 2
        push 2
        memr
        op ++
        memw
        jmp ..join
..open: push 2
        memr
        memr
        push 1
        op %
        push 0
        cmp !=
        jmp ..join
        push 1
        push 1
        memr
        memr
        memw
        jmp ..join
..close:
        push 2
        memr
        memr
        push 1
        op %
        push 0
        cmp =
        jmp ..join
        push 1
        push 1
        memr
        memr
        op -
        memw
        jmp ..join
..out:  push 2
        memr
        memr
        push 1
        op %
        push 256
        op *
        out
        jmp ..join
..in:   push 2
        memr
        push 2
        memr
        memr
        op floor
        in
        push 256
        op /
        op +
        memw
..join: pop 1
        push 1
        push 1
        memr
        op ++
        memw
        jmp .eval
