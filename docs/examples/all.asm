; example loader (auto-generated)
all_main:
        push 12
        out
        push 0
        push 10
        push ';'
        push ' '
        push 'm'
        push 's'
        push 'a'
        push '.'
        push 't'
        push 'e'
        push 's'
        push 'r'
        push 'a'
        push 'h'
        push 'c'
        push ' '
        push ':'
        push '0'
        outs
        push 0
        push 10
        push 'r'
        push 'e'
        push 'l'
        push 'b'
        push 'm'
        push 'e'
        push 's'
        push 's'
        push 'a'
        push 's'
        push 'i'
        push 'd'
        push '-'
        push 'f'
        push 'l'
        push 'e'
        push 's'
        push ' '
        push ';'
        push ' '
        push 'm'
        push 's'
        push 'a'
        push '.'
        push 's'
        push 'i'
        push 'd'
        push ' '
        push ':'
        push '1'
        outs
        push 0
        push 10
        push ';'
        push ' '
        push 'm'
        push 's'
        push 'a'
        push '.'
        push 'z'
        push 'z'
        push 'u'
        push 'b'
        push 'z'
        push 'z'
        push 'i'
        push 'f'
        push ' '
        push ':'
        push '2'
        outs
        push 0
        push 10
        push 'e'
        push 'n'
        push 'i'
        push 'h'
        push 'c'
        push 'a'
        push 'm'
        push ' '
        push 'h'
        push 't'
        push 'u'
        push 'r'
        push 't'
        push ' '
        push ';'
        push ' '
        push 'm'
        push 's'
        push 'a'
        push '.'
        push 'h'
        push 't'
        push 'u'
        push 'r'
        push 't'
        push ' '
        push ':'
        push '3'
        outs
        push 0
        push '?'
        push 't'
        push 'c'
        push 'e'
        push 'l'
        push 'e'
        push 's'
        outs
.bad:   in
        push 12
        out
        dup 0
        push 0
        cmp =
        jmp charset_stub
        dup 0
        push 1
        cmp =
        jmp dis_stub
        dup 0
        push 2
        cmp =
        jmp fizzbuzz_stub
        dup 0
        push 3
        cmp =
        jmp truth_stub
        pop 1
        jmp .bad
charset_stub:
        pop 1
        call charset_main
        jmp all_main
dis_stub:
        pop 1
        call dis_main
        jmp all_main
fizzbuzz_stub:
        pop 1
        call fizzbuzz_main
        jmp all_main
truth_stub:
        pop 1
        call truth_main
        jmp all_main
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
; self-disassembler
dis_main:
        push 0
        push '?'
        push 'r'
        push 'd'
        push 'd'
        push 'a'
        outs
        in
        push 10
        out
.loop:  dup 0
        call .num5
        out
        out
        out
        out
        out
        push ' '
        out
        dup 0
        progr
        dup 0
        call .num5
        out
        out
        out
        out
        out
        push ' '
        out
        push '|'
        out
        push ' '
        out
        dup 0
        push 10000
        op %
        dup 1
        push 0
        dup 1
        push 0
        cmp =
        jmp .p_null
        dup 1
        push 1
        cmp =
        jmp .p_cmp1
        dup 1
        push 2
        cmp =
        jmp .p_cmp2
        dup 1
        push 3
        cmp =
        jmp .p_cmp3
        dup 1
        push 4
        cmp =
        jmp .p_cmp4
        dup 1
        push 5
        cmp =
        jmp .p_cmp5
        dup 1
        push 6
        cmp =
        jmp .p_cmp6
        dup 1
        push 7
        cmp =
        jmp .p_cmp7
        dup 1
        push 8
        cmp =
        jmp .p_ret
        dup 1
        push 9
        cmp =
        jmp .p_in
        dup 1
        push 1000
        cmp >=
        jmp .large
        dup 1
        push 10
        cmp =
        jmp .p_out
        dup 1
        push 11
        cmp =
        jmp .p_memr
        dup 1
        push 12
        cmp =
        jmp .p_memw
        dup 1
        push 13
        cmp =
        jmp .p_progr
        dup 1
        push 14
        cmp =
        jmp .p_progw
        dup 1
        push 15
        cmp =
        jmp .p_valr
        dup 1
        push 16
        cmp =
        jmp .p_valw
        dup 1
        push 100
        cmp =
        jmp .p_uinc
        dup 1
        push 101
        cmp =
        jmp .p_udec
        dup 1
        push 102
        cmp =
        jmp .p_ulog10
        dup 1
        push 103
        cmp =
        jmp .p_ulog2
        dup 1
        push 104
        cmp =
        jmp .p_uln
        dup 1
        push 105
        cmp =
        jmp .p_uexp10
        dup 1
        push 106
        cmp =
        jmp .p_uexp2
        dup 1
        push 107
        cmp =
        jmp .p_uexp
        dup 1
        push 108
        cmp =
        jmp .p_uneg
        dup 1
        push 110
        cmp =
        jmp .p_usin
        dup 1
        push 111
        cmp =
        jmp .p_ucos
        dup 1
        push 112
        cmp =
        jmp .p_utan
        dup 1
        push 113
        cmp =
        jmp .p_usec
        dup 1
        push 114
        cmp =
        jmp .p_ucsc
        dup 1
        push 115
        cmp =
        jmp .p_ucot
        dup 1
        push 116
        cmp =
        jmp .p_uceil
        dup 1
        push 117
        cmp =
        jmp .p_ufloor
        dup 1
        push 118
        cmp =
        jmp .p_uround
        dup 1
        push 119
        cmp =
        jmp .p_usign
        dup 1
        push 120
        cmp =
        jmp .p_usinh
        dup 1
        push 121
        cmp =
        jmp .p_ucosh
        dup 1
        push 122
        cmp =
        jmp .p_utanh
        dup 1
        push 123
        cmp =
        jmp .p_usech
        dup 1
        push 124
        cmp =
        jmp .p_ucsch
        dup 1
        push 125
        cmp =
        jmp .p_ucoth
        dup 1
        push 126
        cmp =
        jmp .p_usqr
        dup 1
        push 127
        cmp =
        jmp .p_usqrt
        dup 1
        push 130
        cmp =
        jmp .p_uasin
        dup 1
        push 131
        cmp =
        jmp .p_uacos
        dup 1
        push 132
        cmp =
        jmp .p_uatan
        dup 1
        push 133
        cmp =
        jmp .p_uasec
        dup 1
        push 134
        cmp =
        jmp .p_uacsc
        dup 1
        push 135
        cmp =
        jmp .p_uacot
        dup 1
        push 140
        cmp =
        jmp .p_uasinh
        dup 1
        push 141
        cmp =
        jmp .p_uacosh
        dup 1
        push 142
        cmp =
        jmp .p_uatanh
        dup 1
        push 143
        cmp =
        jmp .p_uasech
        dup 1
        push 144
        cmp =
        jmp .p_uacsch
        dup 1
        push 145
        cmp =
        jmp .p_uacoth
        dup 1
        push 200
        cmp =
        jmp .p_badd
        dup 1
        push 201
        cmp =
        jmp .p_bsub
        dup 1
        push 202
        cmp =
        jmp .p_bmul
        dup 1
        push 203
        cmp =
        jmp .p_bdiv
        dup 1
        push 204
        cmp =
        jmp .p_bmod
        dup 1
        push 205
        cmp =
        jmp .p_bexp
        dup 1
        push 206
        cmp =
        jmp .p_blog
        dup 1
        push 207
        cmp =
        jmp .p_bnrt
        dup 1
        push 208
        cmp =
        jmp .p_batan2
.large: dup 1
        push 2000
        cmp <
        jmp .p_pop
        dup 1
        push 20000
        cmp <
        jmp .p_jmp
        dup 1
        push 30000
        cmp <
        jmp .p_call
        dup 1
        push 40000
        cmp <
        jmp .p_push
        jmp .p_dup
.exit:  pop 3
        op ++
        dup 0
        push 10000
        cmp >=
        ret
        push 10
        out
        jmp .loop
.p_null:
        push 'l'
        push 'l'
        push 'u'
        push 'n'
        jmp .join
.p_cmp1:
        push '>'
        jmp .g_cmp
.p_cmp2:
        push '='
        jmp .g_cmp
.p_cmp3:
        push '='
        push '>'
        jmp .g_cmp
.p_cmp4:
        push '<'
        jmp .g_cmp
.p_cmp5:
        push '='
        push '!'
        jmp .g_cmp
.p_cmp6:
        push '='
        push '<'
        jmp .g_cmp
.p_cmp7:
        push 0
        push '!'
.g_cmp: push ' '
        push 'p'
        push 'm'
        push 'c'
        jmp .join
.p_ret: push 't'
        push 'e'
        push 'r'
        jmp .join
.p_in:  push 'n'
        push 'i'
        jmp .join
.p_out: push 't'
        push 'u'
        push 'o'
        jmp .join
.p_memr:
        push 'r'
        jmp .g_mem
.p_memw:
        push 'w'
.g_mem: push 'm'
        push 'e'
        push 'm'
        jmp .join
.p_progr:
        push 'r'
        jmp .g_prog
.p_progw:
        push 'w'
.g_prog:
        push 'g'
        push 'o'
        push 'r'
        push 'p'
        jmp .join
.p_valr:
        push 'r'
        jmp .g_val
.p_valw:
        push 'w'
.g_val:
        push 'l'
        push 'a'
        push 'v'
        jmp .join
.p_uinc:
        push '+'
        push '+'
        jmp .g_op
.p_udec:
        push '-'
        push '-'
        jmp .g_op
.p_ulog10:
        push '0'
        push '1'
        push 'g'
        push 'o'
        push 'l'
        jmp .g_op
.p_ulog2:
        push '2'
        push 'g'
        push 'o'
        push 'l'
        jmp .g_op
.p_uln: push 'n'
        push 'l'
        jmp .g_op
.p_uexp10:
        push '^'
        push '0'
        push '1'
        jmp .g_op
.p_uexp2:
        push '^'
        push '2'
        jmp .g_op
.p_uexp:
        push '^'
        push 'e'
        jmp .g_op
.p_uneg:
        push '-'
        push '1'
        jmp .g_op
.p_usin:
        call .g_sin
        jmp .g_op
.p_ucos:
        call .g_cos
        jmp .g_op
.p_utan:
        call .g_tan
        jmp .g_op
.p_usec:
        call .g_sec
        jmp .g_op
.p_ucsc:
        call .g_csc
        jmp .g_op
.p_ucot:
        call .g_cot
        jmp .g_op
.p_uceil:
        push 'l'
        push 'i'
        push 'e'
        push 'c'
        jmp .g_op
.p_ufloor:
        push 'r'
        push 'o'
        push 'o'
        push 'l'
        push 'f'
        jmp .g_op
.p_uround:
        push 'd'
        push 'n'
        push 'u'
        push 'o'
        push 'r'
        jmp .g_op
.p_usign:
        push 'n'
        push 'g'
        push 'i'
        push 's'
        jmp .g_op
.p_usinh:
        push 'h'
        jmp .p_usin
.p_ucosh:
        push 'h'
        jmp .p_ucos
.p_utanh:
        push 'h'
        jmp .p_utan
.p_usech:
        push 'h'
        jmp .p_usec
.p_ucsch:
        push 'h'
        jmp .p_ucsc
.p_ucoth:
        push 'h'
        jmp .p_ucot
.p_usqr:
.p_usqrt:
.p_uasin:
        call .g_sin
        push 'a'
        jmp .g_op
.p_uacos:
        call .g_cos
        push 'a'
        jmp .g_op
.p_uatan:
        call .g_tan
        push 'a'
        jmp .g_op
.p_uasec:
        call .g_sec
        push 'a'
        jmp .g_op
.p_uacsc:
        call .g_csc
        push 'a'
        jmp .g_op
.p_uacot:
        call .g_cot
        push 'a'
        jmp .g_op
.p_uasinh:
        push 'h'
        jmp .p_uasin
.p_uacosh:
        push 'h'
        jmp .p_uacos
.p_uatanh:
        push 'h'
        jmp .p_uatan
.p_uasech:
        push 'h'
        jmp .p_uasec
.p_uacsch:
        push 'h'
        jmp .p_uacsc
.p_uacoth:
        push 'h'
        jmp .p_uacot
.p_badd:
        push '+'
        jmp .g_op
.p_bsub:
        push '-'
        jmp .g_op
.p_bmul:
        push '*'
        jmp .g_op
.p_bdiv:
        push '/'
        jmp .g_op
.p_bmod:
        push '%'
        jmp .g_op
.p_bexp:
        push '^'
        jmp .g_op
.p_blog:
        push 'g'
        push 'o'
        push 'l'
        jmp .g_op
.p_bnrt:
        push 't'
        push 'r'
        push 'n'
        jmp .g_op
.p_batan2:
        push '2'
        push 'n'
        push 'a'
        push 't'
        push 'a'
.g_op:  push ' '
        push 'p'
        push 'o'
        jmp .join
.p_pop: dup 2
        push 999
        op -
        call .num
        push ' '
        push 'p'
        push 'o'
        push 'p'
        jmp .join
.p_jmp: dup 2
        call .num
        push '@'
        push ' '
        push 'p'
        push 'm'
        push 'j'
        jmp .join
.p_call:
        dup 2
        call .num
        push '@'
        push ' '
        push 'l'
        push 'l'
        push 'a'
        push 'c'
        jmp .join
.p_push:
        push 2
        dup 3
        memw
        dup 2
        call .num
        push ' '
        push ';'
        push ' '
        push '''
        push 2
        memr
        valr
        dup 0
        push 0
        cmp =
        jmp ..fix
        push '''
..unfix:
        push ' '
        push 'h'
        push 's'
        push 'u'
        push 'p'
        jmp .join
..fix:  pop 2
        push '0'
        jmp ..unfix
.p_dup:
        dup 2
        call .num
        push ' '
        push 'p'
        push 'u'
        push 'd'
.join:  outs
        jmp .exit
.g_sin: push 'n'
        push 'i'
        push 's'
        ret
.g_cos: push 's'
        push 'o'
        push 'c'
        ret
.g_tan: push 'n'
        push 'a'
        push 't'
        ret
.g_sec: push 'c'
        push 'e'
        push 's'
        ret
.g_csc: push 'c'
        push 's'
        push 'c'
        ret
.g_cot: push 't'
        push 'o'
        push 'c'
        ret
.num5:  call .num
..l:    push 1
        memr
        push 0
        cmp =
        ret
        push ' '
        push 1
        push 1
        memr
        op --
        memw
        jmp ..l
.num:   dup 0
        push 0
        cmp =
        jmp ..z
        push 0
        dup 1
        memw
        push 1
        push 5
        memw
..l:    push 0
        cmp =
        ret
        push 0
        memr
        push 10
        op %
        push '0'
        op +
        push 1
        push 1
        memr
        op --
        memw
        push 0
        push 0
        memr
        push 10
        op /
        op floor
        memw
        push 0
        memr
        jmp ..l
..z:    push '0'
        op +
        push 1
        push 4
        memw
        ret
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
.num:    push 0 ; end marker
..loop:  dup 1
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
        ret
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
