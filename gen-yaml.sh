#!/usr/bin/env sh
#
# Provide a single argument with the path to the elf.
cargo build --release
elf=target/thumbv7em-none-eabi/release/ra4m1-flash-algorithm
out_yaml=target/definition.yaml
sed -e 's/algorithm-test # \(.*\)$/\1/' template.yaml > "$out_yaml"
target-gen elf --fixed-load-address --update "$elf" "$out_yaml"
