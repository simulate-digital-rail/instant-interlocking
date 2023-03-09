# Code Generation

A tool to generate interlocking code from driveway descriptions. Given a list of driveways as 
track elements and their target states (optionally also topology and placement information if
you want to use the graphical frontend), this tool will generate a functional interlocking in
safe Rust.

## Code Generation Process

The code generation tool receives its input in the form of a JSON file containing a list of
driveways which consist of track elements and their target states. These are parsed and transformed
into an internal representation. This representation is transformed into Rust code using the
[`quote`](https://docs.rs/quote) crate and exported as a new cargo project.

## Example invocations

For gRPC:

```
cargo run -- -e -o ixl grpc --addr 127.0.0.1:6007 --topology topology.json --placement placement.json
```

For CLI:

```
cargo run -- -e -o ixl cli
```