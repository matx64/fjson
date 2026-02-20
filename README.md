# fjson

**fjson** is a _zero-dependency_ JSON Parser and Fixer. It takes _any_ input and produces valid JSON. No AI involved.

<a href="https://postimg.cc/18JJ7vJ5">
  <img src="https://i.postimg.cc/QMrwtP75/fjson-hero.webp" height="400" alt="fjson image" />
</a>

## Use Cases

- **Truncated logs** - Fix JSON that was cut off due to log size limits or truncation
- **Minified JSON** - Beautify and validate compressed JSON
- **Partial data** - Recover valid JSON from incomplete API responses
- **Nested JSON strings** - Automatically deserialize JSON stored inside string values
- **Quick validation** - Check and fix JSON on the fly without strict parsing errors

## Features

- **Forgiving parser** - Never fails, always produces valid JSON
- **Auto-repair** - Closes missing brackets, strings, completes partial values
- **Nested deserialization** - Recursively parses JSON inside string values
- **Number normalization** - Removes trailing zeros, fixes invalid formats
- **Case-insensitive** - Normalizes TRUE/false/NULL to true/false/null
- **Formatting** - Beautifies minified JSON output
- **Zero dependencies** - Core library has no external dependencies
- **Multiple targets** - Web, CLI, Rust library, and WebAssembly

## Installation

### CLI

```bash
cargo install fjson-cli
```

Then pipe JSON through stdin:

```bash
cat input.json | fjson-cli
echo '{"key": "value"}' | fjson-cli
```

### Rust Library

Add to `Cargo.toml`:

```toml
[dependencies]
fjson-core = "0.1"
```

### WebAssembly

```bash
cargo install wasm-pack
wasm-pack build wasm --target web --out-dir ../web/pkg --release
```

## Usage

**Web:** https://matx64.github.io/fjson

**Library:** https://crates.io/crates/fjson-core

**CLI and WebAssembly:** Check the latest [Release Docs](https://github.com/matx64/fjson/releases/latest).

## Project Structure

- **lib** (`fjson-core`) - Core parsing library with zero external dependencies
- **cli** (`fjson-cli`) - Command-line interface for stdin/stdout processing
- **wasm** (`fjson-wasm`) - WebAssembly bindings for browser usage
- **web** - Web interface demo

## License

Copyright © 2025-present, [fjson Contributors](https://github.com/matx64/fjson/graphs/contributors).

This project is [MIT](https://github.com/matx64/fjson/blob/main/LICENSE) licensed.
