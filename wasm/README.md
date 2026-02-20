# fjson-wasm

WebAssembly bindings for the fjson JSON parser and fixer.

## Installation

Build with [wasm-pack](https://github.com/drager/wasm-pack):

```bash
wasm-pack build . --target web --out-dir pkg --release
```

Or install wasm-pack globally:

```bash
cargo install wasm-pack
wasm-pack build . --target web --out-dir pkg --release
```

## For the fjson Web Demo

If contributing to fjson itself, build from the repository root:

```bash
wasm-pack build wasm --target web --out-dir ../web/pkg --release
```

## API

### `fix(input: string) -> string`

Parse and fix JSON input. Returns formatted, valid JSON.

## Usage

### JavaScript

```javascript
import init, { fix } from './pkg/fjson_wasm.js';

async function run() {
  await init();
  
  const broken = '{"user" "foo", "age": 0020, }';
  const fixed = fix(broken);
  console.log(fixed);
}

run();
```

### HTML

```html
<script type="module">
  import init, { fix } from './pkg/fjson_wasm.js';
  
  await init();
  
  const input = document.getElementById('input').value;
  const output = fix(input);
  document.getElementById('output').value = output;
</script>
```

## Features

- Zero JavaScript dependencies
- Fast WASM performance
- Same API as Rust library
- Works in all modern browsers
- Automatically deserializes nested JSON strings

## Notes

- The WASM module must be initialized with `init()` before use
- Build output is in the `pkg/` directory
- Supports the same forgiving parsing behavior as the Rust library
