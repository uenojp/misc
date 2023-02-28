#!/bin/bash

set eu

check() {
    expected=$(tsort <<< "$1")
    actual=$(./target/release/tsort-rs <<< "$1")

    printf "Input\n%s\n" "$1"
    [[ "$expected" == "$actual" ]] \
        && printf "OK\n\n" \
        || printf "Expected\n%s,\nbut got\n%s\n" "$expected" "$actual"
}

cargo build --release

check "A B"

check "A B
B C
C D"

check "A B
A C
B D
B F
C B
C D
C F
F D"
