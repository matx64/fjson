# fjson-web

Web interface for the fjson JSON parser and fixer.

## Building

Build the wasm library targeting web with [wasm-pack](https://github.com/drager/wasm-pack):

```sh
wasm-pack build ../wasm --target web --out-dir pkg --release
```

## Running Locally

Serve the `web/` directory with any static file server:

```sh
python3 -m http.server 8000
# or
npx serve .
```

Then open http://localhost:8000

## Features

- Real-time JSON fixing and formatting
- Copy to clipboard functionality
- Clean, responsive interface
- Works entirely in the browser (no server needed)
- Supports all modern browsers

## Live Demo

https://matx64.github.io/fjson
