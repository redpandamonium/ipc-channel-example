# ipc-channel-example
A toy example how to use the ipc-channel rust crate. There are two programs, a client and a server.
The client starts the server as its child process. Two ipc channels (bidirectional messaging) are established and messages are passed back and forth.
This example is for ipc-channel version 0.16, but may work in future versions.

Simply run `cargo build && cargo run --bin client -- --server-bin target/debug/server` to execute.
