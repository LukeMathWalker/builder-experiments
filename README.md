# Implementation PoC

This repository shows an implementation of the service builder API suggested in [this RFC proposal](https://github.com/awslabs/smithy-rs/pull/1859).
The new API can be found in the `*-experimental` crates, in particular [`pokemon-service-server-sdk-experimental/src/service.rs`](pokemon-service-server-sdk-experimental/src/service.rs).

Use `cargo run --bin pokemon-service-experimental` to test the error messages returned by the new API proposal.
