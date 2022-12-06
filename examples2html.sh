#!/usr/bin/env bash
echo "; example loader (auto-generated)" >www/examples/all.asm
echo "all_main:" >>www/examples/all.asm
function all_print_string {
	for (( i=${#1}-1; i>=0; i-- )); do
		echo "        push '${1:$i:1}'" >>www/examples/all.asm
	done
}
COUNTER=0
echo "        push 12" >>www/examples/all.asm
echo "        out" >>www/examples/all.asm
for f in $(ls -A examples); do
  echo -e "s/PLACEHOLDER_EXAMPLES/<option value='$f'>$f $(head examples/$f -n 1)<\/option>PLACEHOLDER_EXAMPLES/"
done
for f in $(ls examples); do
	echo "        push 0" >>www/examples/all.asm
	echo "        push 10" >>www/examples/all.asm
	all_print_string "$COUNTER: $f $(head examples/$f -n 1)"
	echo "        outs" >>www/examples/all.asm
	COUNTER=$(($COUNTER+1))
done
echo "        push 0" >>www/examples/all.asm
all_print_string "select?"
echo "        outs" >>www/examples/all.asm
echo ".bad:   in" >>www/examples/all.asm
echo "        push 12" >>www/examples/all.asm
echo "        out" >>www/examples/all.asm
COUNTER=0
for f in $(ls examples); do
	echo "        dup 0" >>www/examples/all.asm
	echo "        push $COUNTER" >>www/examples/all.asm
	echo "        cmp =" >>www/examples/all.asm
	FNORM=$(basename -s .asm -- $f)
	echo "        jmp ${FNORM}_stub" >>www/examples/all.asm
	COUNTER=$(($COUNTER+1))
done
echo "        pop 1" >>www/examples/all.asm
echo "        jmp .bad" >>www/examples/all.asm
for f in $(ls examples); do
	FNORM=$(basename -s .asm -- $f)
	echo "${FNORM}_stub:" >>www/examples/all.asm
	echo "        pop 1" >>www/examples/all.asm
	echo "        call ${FNORM}_main" >>www/examples/all.asm
	echo "        jmp all_main" >>www/examples/all.asm
done
for f in $(ls examples); do
	cat examples/$f >>www/examples/all.asm
done
echo "s/PLACEHOLDER_EXAMPLES/<option value='all.asm'>all.asm ; example loader (auto-generated)<\/option>/"
