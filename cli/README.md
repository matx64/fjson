# fjson-cli

Command-line interface for the fjson JSON parser and fixer. Reads from stdin, writes to stdout.

## Installation

```bash
cargo install fjson-cli
```

## Usage

Pipe JSON through stdin:

```sh
cat input.json | fjson-cli
```

Or pipe directly:

```sh
echo '{"key": "value"}' | fjson-cli
```

## Examples

Fix truncated JSON:

```sh
echo '{"request_id": 123, "status": "success", "data": {"items": [{"id": 1}, {"id": 2' | fjson-cli
```

Process log files with truncated JSON:

```sh
tail -n 100 application.log | grep json | fjson-cli > fixed.json
```

Beautify minified JSON:

```sh
curl -s https://api.example.com/data | fjson-cli
```

## Notes

- Reads from stdin, outputs to stdout
- Never fails on invalid input - always produces valid JSON
- Automatically closes missing brackets and strings
- Deserializes nested JSON strings
