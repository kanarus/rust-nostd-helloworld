This is a minimal sample to show how to print `Hello, world!` on x86_64 Linux by latest **no std** Rust at 03/2025.

## How to run

Just `cargo run` or `cargo run --release`.

## Tips

### emit binary to current directory

```sh
cargo build --release -Z unstable-options --artifact-dir .
```

### emit assembly

```sh
cargo rustc --release -- --emit asm -C llvm-args=-x86-asm-syntax=intel
```

`-o ＜name＞` option is available to override output place.
