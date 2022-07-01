#!/bin/bash

set -eux

(
    cd data
    rm 'training.data' 'test.data' || true
    N="${1:-5}"
    L=$(wc -l < wdbc.data)

    shuf < wdbc.data | pee "head -$N > test.data" "tail -$((L - N)) > training.data"
)

cargo run \
    | awk '{if(NR>1&&$2!=$3){printf "\x1b[31m"$0"\x1b[0m\n";c+=1}else{print}}END{print "error rate " c/(NR-1)*100 "%"}'

