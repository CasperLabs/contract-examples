# CasperLabs Smart Contract Examples

Each subdirectory contains an example of a smart contract definition and a transaction to call it.


## Building

To build the contracts you'll need to install [Rust](https://www.rust-lang.org/tools/install) and [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html).

You can compile all the contracts with `make all`, or individually with `make hello-name` for example. The build will produce WASM files, for example at `hello-name/define/target/wasm32-unknown-unknown/release/helloname.wasm`.


## Running a network locally

You can use `make up` to start a single local node in Docker using the `docker-compose.yml` file. It's a delibaretely simple setup so that you can see the effects of running deploys, rather than how the distributed consensus works. If you want to see more nodes working in tandem, with monitoring, have a look at the [docker setup](https://github.com/CasperLabs/CasperLabs/tree/dev/docker) in the main repository.


## Installing the client

To deploy the contracts you have to use the `casperlabs-client`, which can talk to a CasperLabs Node. Packages are published to http://repo.casperlabs.io/casperlabs/repo

For example you an install the client on Ubuntu as follows (please substitute `[VERSION]` with the what you can find in the repo):

```console
curl -o casperlabs-client.deb http://repo.casperlabs.io/casperlabs/repo/dev/casperlabs-client_[VERSION]_all.deb
sudo dpkg -i ./casperlabs-client.deb
```

Depending on your version of Ubuntu you may see errors about unmet dependencies (specifically `openjdk-11-jre-headless`), but if you have Java then try running `casperlabs-client -- --version`, it might work, otherwise please install OpenJDK version 11.

You can also install from tarball:

```console
curl -O http://repo.casperlabs.io/casperlabs/repo/dev/casperlabs-client-[VERSION].tgz
tar -xzf casperlabs-client-[VERSION].tgz
alias casperlabs-client=$PWD/casperlabs-client-[VERSION]/bin/casperlabs-client
casperlabs-client --version
```

Alternatively we can use the docker version of the client with `docker run` as the multi-node docker example does in the main repository, using `--network contract-examples_default`.


## Deploying a contract

Let's deploy one of the contracts.

NOTE: We're going to use the same WASM code for `payment` and `session` now because the node doesn't have the payment feature yet, but this is going to change in future releases.

```console
$ source .env
$ CONTRACT=hello-name/define/target/wasm32-unknown-unknown/release/helloname.wasm
$ casperlabs-client -- --host localhost --port $CL_GRPC_PORT_EXTERNAL deploy \
    --from 00000000000000000000 \
    --gas-limit 100000000 --gas-price 1 \
    --session $CONTRACT \
    --payment $CONTRACT
Success!
```

We can check that the node indeed got the deploy:

```console
$ docker logs --tail 1 contract-examples_node
15:45:10.908 [node-runner-47] INFO  i.c.casper.MultiParentCasperImpl - Received Deploy #1550504710551
```

Now let's trigger block proposal.

NOTE: This method is going to be removed in future releases.

```console
$ casperlabs-client -- --host localhost --port $CL_GRPC_PORT_EXTERNAL propose
Response: Success! Block b24c8311ce... created and added.
```

Checking the node logs again, we can see that a block was created:

```console
$ docker logs --tail 10 contract-examples_node
15:45:10.908 [node-runner-47] INFO  i.c.casper.MultiParentCasperImpl - Received Deploy #1550504710551
15:49:01.546 [node-runner-47] INFO  i.c.casper.MultiParentCasperImpl - 1 parents out of 1 latest blocks will be used.
15:49:01.909 [grpc-default-executor-0] INFO  i.c.casper.MultiParentCasperImpl - Block #1 created with effects:
Hash(a4669933ec...) :: Write(Contract(0061736d01..., {}))
15:49:01.999 [grpc-default-executor-0] INFO  i.c.casper.MultiParentCasperImpl - Attempting to add Block b24c8311ce... to DAG.
15:49:02.164 [grpc-default-executor-0] INFO  i.c.comm.transport.TcpTransportLayer - stream to List() blob
15:49:02.166 [grpc-default-executor-0] INFO  i.c.casper.util.comm.CommUtil$ - Sent Block #1 (b24c8311ce...) -- Sender ID 6736848c09... -- M Parent Hash ff79d178ea... -- Contents 1144ec89ef...-- Shard ID casperlabs to peers
15:49:02.167 [grpc-default-executor-0] INFO  i.c.casper.MultiParentCasperImpl - Added b24c8311ce...
15:49:02.173 [grpc-default-executor-0] INFO  i.c.casper.MultiParentCasperImpl - New fork-choice tip is block ff79d178ea....
15:49:02.177 [grpc-default-executor-0] INFO  i.c.casper.MultiParentCasperImpl - New last finalized block hash is ff79d178ea....
```

And the logs of the execution engine show the costs and the change in state:

```console
$ docker logs --tail 10 contract-examples_execution-engine
Server is listening on socket: .casperlabs/sockets/.casper-node.sock
Gas count: 968870
Gas left: 99031130
Effects applied. New state hash is: [17, 68, 236, 137, 239, 106, 183, 55, 199, 186, 163, 181, 36, 224, 113, 72, 54, 173, 98, 29, 224, 50, 96, 237, 173, 197, 16, 238, 78, 125, 8, 139]
```

Now let's try to call the contract.

```console
$ CONTRACT=hello-name/call/target/wasm32-unknown-unknown/release/helloworld.wasm
$ casperlabs-client -- --host localhost --port $CL_GRPC_PORT_EXTERNAL deploy \
    --from 00000000000000000000 \
    --gas-limit 100000000 --gas-price 1 \
    --session $CONTRACT \
    --payment $CONTRACT
Success!
$ casperlabs-client -- --host localhost --port $CL_GRPC_PORT_EXTERNAL propose
Response: Success! Block a32a34ae5b... created and added.
```

If we look at the node logs, we can see that the effects of the deploy have been included in the block:

```console
$ docker logs --tail 10 contract-examples_node
15:57:09.406 [node-runner-47] INFO  i.c.casper.MultiParentCasperImpl - 1 parents out of 1 latest blocks will be used.
15:57:09.523 [grpc-default-executor-2] INFO  i.c.casper.MultiParentCasperImpl - Block #2 created with effects:
URef(3a2bee5164...) :: Write(String(Hello, World))
15:57:09.578 [grpc-default-executor-2] INFO  i.c.casper.MultiParentCasperImpl - Attempting to add Block a32a34ae5b... to DAG.
15:57:09.666 [grpc-default-executor-2] INFO  i.c.comm.transport.TcpTransportLayer - stream to List() blob
15:57:09.667 [grpc-default-executor-2] INFO  i.c.casper.util.comm.CommUtil$ - Sent Block #2 (a32a34ae5b...) -- Sender ID 6736848c09... -- M Parent Hash b24c8311ce... -- Contents 74705169ca...-- Shard ID casperlabs to peers
15:57:09.667 [grpc-default-executor-2] INFO  i.c.casper.MultiParentCasperImpl - Added a32a34ae5b...
15:57:09.669 [grpc-default-executor-2] INFO  i.c.casper.MultiParentCasperImpl - New fork-choice tip is block b24c8311ce....
15:57:09.690 [grpc-default-executor-2] INFO  i.c.casper.MultiParentCasperImpl - Fault tolerance for block b24c8311ce... is 1.0; threshold is 0.0
15:57:09.696 [grpc-default-executor-2] INFO  i.c.casper.MultiParentCasperImpl - Removed 1 deploys from deploy history as we finalized block b24c8311ce....
15:57:09.697 [grpc-default-executor-2] INFO  i.c.casper.MultiParentCasperImpl - New last finalized block hash is b24c8311ce....
```