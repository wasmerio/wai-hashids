# WAI Hashids

This is an example demonstrating the popular HashIds implementation with automatic bindings via WAI into Javascript and Python.

## How to build

```bash
$ rustup target add wasm32-unknown-unknown
$ cargo build --target=wasm32-unknown-unknown --release
```

## Publish into Wasmer

```bash
$ wasmer publish
```
