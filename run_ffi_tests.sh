#!/bin/bash

# Generate bindgen
cbindgen -v --config cbindgen.toml --crate ffi-example --output ffi_example.h
cargo build --release
# Copy library to c_example
cp target/release/libffi_example.a .

# Build main binary
gcc c_example.c -L. -lffi_example -lm -o main

# Run main binary
export LD_LIBRARY_PATH=.
output=$(./main)
echo "$output"
echo "::set-output name=output::$output"


