#!/bin/sh

cd rust
cargo lipo --release
cbindgen src/lib.rs -l c > rust_bridge_header.h

# mkdir ~/Documents/NoCrunchTime/bitplode_united/BitplodeApp/include
mkdir /Users/kimanzimati/Documents/NoCrunchTime/bitplode_united/BitplodeApp/libs
cp rust_bridge_header.h ~/Documents/NoCrunchTime/bitplode_united/BitplodeApp/BitplodeApp/rust_bridge_header.h
cp target/universal/release/librust_bridge_header.a /Users/kimanzimati/Documents/NoCrunchTime/bitplode_united/BitplodeApp/libs
