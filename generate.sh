#!/usr/bin/env sh
svd2rust -i SKEAZN642.svd
rm -rf src
form -i lib.rs -o src/ && rm lib.rs
cargo fmt
