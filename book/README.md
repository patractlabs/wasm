# The PatractLabs WASM Book

This book tells about the solution of replacing the `wasmi` in `pallet-contrat`.


## Why?

We are trying to use [wasm3][wasm3] to replace [wasmi][wasmi] in substrate, for the possibility of faster executing speed.


## Execution

| Project                                                  | Interpreter | JIT | AoT | VM? |
|----------------------------------------------------------|-------------|-----|-----|-----|
| [wac](https://github.com/kanaka/wac)                     | x           |     |     |     |
| [wasm3](https://github.com/wasm3/wasm3)                  | x           |     |     |     |
| [wasmi](https://github.com/paritytech/wasmi)             | x           |     |     |     |
| [wasmtime](https://github.com/bytecodealliance/wasmtime) |             | x   |     |     |
| [wasmer](https://github.com/wasmerio/wasmer)             |             | x   | ?   | x   |
| [wavm](https://github.com/WAVM/WAVM)                     | x           | x   | x   | x   |

## Benchmark

> TODO


## LICENSE

MIT

[wasm3]: https://github.com/wasm3/wasm3
[wasmi]: https://github.com/paritytech/wasmi
