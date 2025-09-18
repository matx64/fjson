# fjson

**fjson** is a JSON Parser and Fixer. It takes _any_ input and produces valid JSON. No AI involved.

## Features

- Deserializes everything by default (root and nested).
- Repairs incomplete JSON by closing missing brackets and strings.
- Normalizes boolean and null values (e.g., True → true, FALSE → false)
- Normalizes numbers (removes trailing zeros, fixes invalid formats).
- Available as both CLI and WebAssembly library.

## Roadmap

- [ ] Parser + Fixer
- [ ] Deserialization
- [ ] Formatting (beautifier)
- [ ] Test suite
- [ ] WebAssembly version
- [ ] CLI version

## License

Copyright © 2025-present, [fjson Contributors](https://github.com/matx64/fjson/graphs/contributors).

This project is [MIT](https://github.com/matx64/fjson/blob/main/LICENSE) licensed.
