# CasperLabs Smart Contract Examples

Each subdirectory contains an example of a smart contract definition and a companion contract that calls it.

## Prerequisites

* [`rustup`](https://rustup.rs/)

After installing `rustup`, run the following commands from the root of this repo:

```
rustup toolchain install $(cat rust-toolchain)
rustup target add --toolchain $(cat rust-toolchain) wasm32-unknown-unknown
```

## Building

To build all the contracts:

```
cargo build --release
```

To build a specific contract and its companion:

```
cargo build --release -p hello-define -p hello-call
```

After building a contract, you will find the corresponding wasm file in `target/wasm32-unknown-unknown/release`.

**NOTE**: The `--release` flag is currently necessary in order to build optimized wasm files that can be deployed from a CasperLabs Node.

## Using

To deploy a compiled contract to a CasperLabs node, please see the CasperLabs [Developer Documentation](https://github.com/CasperLabs/CasperLabs/blob/dev/docs/CONTRACTS.md).
