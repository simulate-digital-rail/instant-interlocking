# Code generation

## Example invocations

For gRPC:

```
cargo run --package code_generation -- -e -o ixl grpc --axum-addr 127.0.0.1:6007 --grpc-addr 127.0.0.1:6006 --ws-port 8080 --topology topology.json --placement placement.json
```

For CLI:

```
cargo run --package code_generation -- -e -o ixl cli
```