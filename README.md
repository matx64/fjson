# fjson

**fjson** is a _zero-dependency_ JSON Parser and Fixer. It takes _any_ input and produces valid JSON. No AI involved.

<a href="https://postimg.cc/18JJ7vJ5">
  <img src="https://i.postimg.cc/QMrwtP75/fjson-hero.webp" height="400" alt="fjson image" />
</a>

## Features

- Deserializes everything by default (root and nested).
- Repairs incomplete JSON by closing missing brackets and strings.
- Normalizes boolean and null values (e.g., True → true, FALSE → false).
- Normalizes numbers (removes trailing zeros, fixes invalid formats).
- Formatting (beautifier).
- Available as Web, CLI, Rust and WebAssembly libraries.

## Usage

**Web:** https://matx64.github.io/fjson

## License

Copyright © 2025-present, [fjson Contributors](https://github.com/matx64/fjson/graphs/contributors).

This project is [MIT](https://github.com/matx64/fjson/blob/main/LICENSE) licensed.
