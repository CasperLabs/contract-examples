# CasperLabs Smart Contract Examples

Each subdirectory contains an example of a smart contract definition and a transaction to call it.

## Building

To build the contracts you'll need to install [Rust](https://www.rust-lang.org/tools/install) and [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html).

You can compile all the contracts with `make all`, or individually with `make hello-name` for example. The build will produce WASM files, for example at `hello-name/define/target/wasm32-unknown-unknown/release/helloname.wasm`.

## Installing the client

To deploy the contracts you have to use the `casperlabs-client`, which can talk to a CasperLabs Node.
Alternatively we can use the docker version of the client.

TODO: Add instructions about how to install debian packages.


## Running a network locally

TODO: Add a docker-compose file that brings up a local node and exposes the gRPC ports.


## Deploying a contract

TODO: Example of dpeloying one of the contracts, then calling it, and looking at the logs to see what happened. Visualizing the DAG.

