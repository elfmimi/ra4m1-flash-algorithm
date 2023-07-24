# Flash Algorithm for RA4M1

This is a CMSIS-Pack style flash algorithm for RA4M1 Series.
It can be used to generate new flash algoritms for usage with `probe-rs`.

## Dependencies

Run the following requirements:

```bash
cargo install cargo-generate cargo-binutils target-gen
rustup component add llvm-tools-preview
```

## Developing the algorithm

Use `gen-yaml.sh` or otherwise manualy execute `cargo build --release` and then `target-gen elf`.

You can find the generated YAML in `target/definition.yaml`.

You may not just want to use `target-gen test` because it will write garbage to `Option-Setting Memory`.

# License

This thingy is licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)

- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
