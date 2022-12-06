;
charset_main:
        push 0
.loop:  dup 0
        out
        op ++
        dup 0
        push 128
        cmp !=
        jmp .loop
        ret
