#!/bin/bash

set -eu

# 100 digits per line
cargo run --release     \
    | sed 's/3/&\n/'    \
    | fold -b100

