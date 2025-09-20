# fjson-cli

The **fjson** cli tool.

## Examples

```sh
cat input.json | cargo run --

echo '{"menu": {
  "id": "file",
  "value": "File",
  "popup": {
    "menuitem": [
      {"value": "New", "onclick": "CreateNewDoc()"},
      {"value": "Open", "onclick": "OpenDoc()"},
      {"value": "Close", "onclick": "CloseDoc()"}
    ]
  }
}}' | cargo run --
```
