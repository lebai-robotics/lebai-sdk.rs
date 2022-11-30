## Python

```bash
docker run -it ghcr.io/pyo3/maturin --name maturin bash

maturin publish --features ffi_py
```

## Lua

cargo build --release --features ffi_lua
