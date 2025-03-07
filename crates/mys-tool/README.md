# mys-tool

`mys-tool` contains assorted debugging utilities for MySocial.

You can build and run `mys-tool` from source with:
```sh
cargo run --bin mys-tool -- <args>
```

## `anemo` tools

You can use the anemo CLI tools to ping or call an RPC on an Anemo server. Note that (for now) this uses randomly generated keys, so a server or method that restricts access to allowlisted peers will reject connections from this tool.

Anemo networks are identified by a "server name" that the client must match. Server names you may want to use:
- MySocial discovery and state sync: `mys`

### ping

Example command to ping an anemo server:

```sh
SERVER_NAME="mys"; \
ADDRESS="1.2.3.4:5678"; \
cargo run --bin mys-tool -- anemo ping --server-name "$SERVER_NAME" "$ADDRESS"
```

### call

`mys-tool` has been preconfigured to support RPC calls using [RON (Rusty Object Notation)](https://crates.io/crates/ron) for the following servivces:
- Narwhal: `PrimaryToPrimary` and `WorkerToWorker`
- MySocial: `Discovery` and `StateSync`

Example command to send an RPC:

```sh
SERVER_NAME="mys"; \
ADDRESS="1.2.3.4:5678"; \
SERVICE_NAME="StateSync"; \
METHOD_NAME="GetCheckpointSummary"; \
REQUEST="BySequenceNumber(123)"; \
cargo run --bin mys-tool -- \
    anemo call --server-name "$SERVER_NAME" "$ADDRESS" "$SERVICE_NAME" "$METHOD_NAME" "$REQUEST"
```
