# chkrs

![Screenshot](./screenshot.png)

A toy project to demonstrate Typescript/Rust interop using Tauri. It's a simple checkers game with a CPU opponent that uses Monte Carlo Tree Search to make decisions.

## Development
Tests:
```bash
cd src-tauri
cargo test
```

Running the app:
```bash
cargo tauri dev
```


## Building
```bash
cargo tauri build
```