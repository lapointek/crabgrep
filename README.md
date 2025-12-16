# cgrep

A fast, efficient, and easy-to-use command-line search tool built in Rust. Supports pattern matching, file searching, and colored output for enhanced readability.

## Installation

### From source

```
git clone https://github.com/lapointek/cgrep
cd cgrep
cargo build --release
```

binary is available at:

```
./target/release/cgrep
```

Optionally move it into your PATH:

```
sudo mv target/release/cgrep /usr/local/bin
```

## How to use

### Basic usage

```
cgrep <pattern> <file>
```

Example:

```
cgrep error log.txt
```

## Development

Run tests:

```
cargo test
```
