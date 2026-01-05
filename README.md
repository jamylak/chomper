# chomper

Trim trailing newline and carriage return bytes from a file in-place.

## Install

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
```

Expected output:

```text
hello
```
