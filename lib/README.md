# fjson

**fjson** is a _zero-dependency_ JSON Parser and Fixer. It takes _any_ input and produces valid JSON. No AI involved.

## Features

- Deserializes everything by default (root and nested).
- Repairs incomplete JSON by closing missing brackets and strings.
- Normalizes boolean and null values (e.g., True → true, FALSE → false).
- Normalizes numbers (removes trailing zeros, fixes invalid formats).
- Formatting (beautifier).
- Zero external dependencies.

## Installation

Add to `Cargo.toml`:

```toml
[dependencies]
fjson-core = "0.1"
```

## API

### `fix(input: impl Into<String>) -> String`

Parse and fix JSON input. Returns formatted, valid JSON.

## Examples

### Basic Usage

```rust
fn main() {
    let broken = r#"{ "user" "foo", "age": 0020, }"#;
    let fixed = fjson_core::fix(broken);
    println!("{}", fixed);
}
```

Output:

```json
{
  "user": "foo",
  "age": 20
}
```

### Truncated Logs

```rust
let truncated_log = r#"{"request_id": 123, "status": "success", "data": {"items": [{"id": 1}, {"id": 2}"#;
let fixed = fjson_core::fix(truncated_log);
```

### Nested JSON Deserialization

```rust
let nested = r#"{"payload": "{\"user\": \"alice\", \"role\": \"admin\"}"}"#;
let fixed = fjson_core::fix(nested);
```

Output:

```json
{
  "payload": {
    "user": "alice",
    "role": "admin"
  }
}
```

## Use Cases

- **Truncated logs** - Recover valid JSON from log entries that were cut off
- **Minified JSON** - Beautify compressed JSON
- **Partial responses** - Handle incomplete API responses
- **Data validation** - Quick fix for malformed JSON from unreliable sources

## License

Copyright © 2025-present, [fjson Contributors](https://github.com/matx64/fjson/graphs/contributors).

This project is [MIT](https://github.com/matx64/fjson/blob/main/LICENSE) licensed.
