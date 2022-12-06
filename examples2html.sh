#!/usr/bin/env bash
echo "; example loader (auto-generated)" >docs/examples/all.asm
echo "all_main:" >>docs/examples/all.asm
function all_print_string {
	for (( i=${#1}-1; i>=0; i-- )); do
		echo "        push '${1:$i:1}'" >>docs/examples/all.asm
	done
}
COUNTER=0
echo "        push 12" >>docs/examples/all.asm
echo "        out" >>docs/examples/all.asm
for f in $(ls -A examples); do
  echo -e "s/PLACEHOLDER_EXAMPLES/<option value='$f'>$f $(head examples/$f -n 1)<\/option>PLACEHOLDER_EXAMPLES/"
done
for f in $(ls examples); do
	echo "        push 0" >>docs/examples/all.asm
	echo "        push 10" >>docs/examples/all.asm
	all_print_string "$COUNTER: $f $(head examples/$f -n 1)"
	echo "        outs" >>docs/examples/all.asm
	COUNTER=$(($COUNTER+1))
done
echo "        push 0" >>docs/examples/all.asm
all_print_string "select?"
echo "        outs" >>docs/examples/all.asm
echo ".bad:   in" >>docs/examples/all.asm
echo "        push 12" >>docs/examples/all.asm
echo "        out" >>docs/examples/all.asm
COUNTER=0
for f in $(ls examples); do
	echo "        dup 0" >>docs/examples/all.asm
	echo "        push $COUNTER" >>docs/examples/all.asm
	echo "        cmp =" >>docs/examples/all.asm
	FNORM=$(basename -s .asm -- $f)
	echo "        jmp ${FNORM}_stub" >>docs/examples/all.asm
	COUNTER=$(($COUNTER+1))
done
echo "        pop 1" >>docs/examples/all.asm
echo "        jmp .bad" >>docs/examples/all.asm
for f in $(ls examples); do
	FNORM=$(basename -s .asm -- $f)
	echo "${FNORM}_stub:" >>docs/examples/all.asm
	echo "        pop 1" >>docs/examples/all.asm
	echo "        call ${FNORM}_main" >>docs/examples/all.asm
	echo "        jmp all_main" >>docs/examples/all.asm
done
for f in $(ls examples); do
	cat examples/$f >>docs/examples/all.asm
done
echo "s/PLACEHOLDER_EXAMPLES/<option value='all.asm'>all.asm ; example loader (auto-generated)<\/option>/"
