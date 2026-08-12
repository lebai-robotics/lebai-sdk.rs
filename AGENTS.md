# AGENTS.md

Rust SDK for Lebai cobots: the JSON-RPC robot API exposed as Lua / Python / WASM bindings. The whole public SDK surface is defined in one place, `src/lib.rs`, inside `#[cmod::cmod] mod lebai_sdk` — the `cmod` crate auto-generates bindings for every language from that file. Workspace member `proto/` is a separately published crate (`lebai-proto`).

## Build prerequisites (fresh clone)

- `git submodule update --init` — `proto/lebai-proto` is not checked out by default and the build hard-fails without it.
- `protoc` must be installed: `proto/build.rs` runs prost + pbjson + pbjsonrpc codegen at compile time; generated code is not committed.
- Verify with `cargo check`. CI (`.github/workflows/rust.yml`) only runs `cargo build --release --features mdns` on push/PR — there are no test steps.

## Tests need hardware

Do not expect `cargo test` to pass locally. `tests/thread.rs` connects to a real robot at `192.168.2.101` (sim port), and `tests/lua/` + `tests/python/` need a simulator at `127.0.0.1` sim port. Tests are manual smoke tests, not CI-runnable.

## Feature flags

- `ffi_lua`, `ffi_py`, `ffi_py_asyncio`, `ffi_wasm`, `module`, `mdns`.
- `mdns` gates `discover_devices` (returns "not support" without it).
- CI combos to mirror: rust `--features mdns`; lua `ffi_lua,module,mdns`; python (maturin) `ffi_py,module,mdns`; wasm (wasm-pack) `ffi_wasm,module`.
- Cross-compile toolchains (armv7/aarch64/riscv64) are preconfigured in `.cargo/config.toml`; macos linker flags live there too.

## Architecture

- `src/rpc/*.rs` — one file per proto domain (motion, io, task, system, ...), each an `impl Robot` block that calls the pbjsonrpc-generated client: `self.c.<rpc_method>(Some(req))` over the WebSocket `Client` in `src/rpc/mod.rs`.
- Pose types (`Pose`, `JointPose`, `CartesianPose`) and several others (`KinData`, `LedStyle`, ...) are handwritten serde wrappers in `proto/src/serde/` over the prost-generated messages; add new wrappers there, not as device JSON.
- `lebai_sdk.pyi` is a hand-maintained Python stub — keep it in sync when changing `src/lib.rs`.
- Adding an RPC method means touching all three layers: a generated message in proto (`.proto` file in the submodule), the `Robot` method in `src/rpc/*.rs`, and the `#[cmod::function]` wrapper in `src/lib.rs`.

## Conventions

- `rustfmt.toml`: `max_width = 150`, `merge_derives = false`; Rust edition 2024.
- Release flow: bump `version` in `Cargo.toml` (and `proto/Cargo.toml`), commit as `release: vX.Y.Z`, tag `v*`. Tags trigger crates.io publish (proto first, `continue-on-error`), PyPI wheel upload, npm publish (wasm-pack), and zipped GitHub release artifacts.
- Errors are propagated as `cmod::Result` strings; there is no error enum.