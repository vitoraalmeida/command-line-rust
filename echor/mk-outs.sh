#!/usr/bin/env bash

# executa o comando echo com diferentes argumentos para que possamos
# comparar com os resultados do echor com os mesmos argumentos

OUTDIR="tests/expected"
[[ ! -d "$OUTDIR" ]] && mkdir -p "$OUTDIR"

echo "Hello there" > $OUTDIR/hello1.txt
echo "Hello" "there" > $OUTDIR/hello2.txt
echo -n "Hello  there" > $OUTDIR/hello1.n.txt
echo -n "Hello" "there" > $OUTDIR/hello2.n.txt
