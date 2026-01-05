# chomper

Trim trailing newline and carriage return bytes from a file in-place.

## Install

Requires Rust 1.70+ (2021 edition).

From source:

```bash
cargo build --release
```

Binary location:

```bash
./target/release/chomper
```

Optional install into Cargo bin dir:

```bash
cargo install --path .
```

## Usage

```bash
chomper <file>
```

Exit codes:

- `0` success
- `1` runtime error (IO, permissions)
- `2` usage error

## Examples

Trim a file with trailing newlines:

```bash
printf "hello\n\n" > sample.txt
chomper sample.txt
cat -v sample.txt
```

Expected output:

```text
hello
```

Run directly with Cargo:

```bash
cargo run -- sample.txt
```
