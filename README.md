# fjson

**fjson** is a JSON Parser and Fixer. It takes _any_ input and produces valid JSON. No AI involved.

Web: https://matx64.github.io/fjson

## Features

- Deserializes everything by default (root and nested).
- Repairs incomplete JSON by closing missing brackets and strings.
- Normalizes boolean and null values (e.g., True → true, FALSE → false)
- Normalizes numbers (removes trailing zeros, fixes invalid formats).
- Available as CLI, Web and WebAssembly library.

## Roadmap

- [x] Parser + Fixer
- [x] WebAssembly library
- [x] CLI tool, Web + CI/CD
- [ ] Deserialization
- [ ] Formatting (beautifier)

## License

Copyright © 2025-present, [fjson Contributors](https://github.com/matx64/fjson/graphs/contributors).

This project is [MIT](https://github.com/matx64/fjson/blob/main/LICENSE) licensed.
