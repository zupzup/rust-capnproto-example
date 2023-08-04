# rust-capnproto-example

Example using Rust and Capnproto 

## Testing

You can run this example by opening 3 shells and running one of the following commands in each of them, in order:

* `cargo run -- sj` - runs the JSON server
* `cargo run -- sc` - runs the Cap'n Proto server
* `cargo run -- c` - runs the client that sends data to the servers

The client creates a data structure and serializes it using JSON and Cap'n Proto. Then it sends it to the server.
The servers both deserialize the incoming data. In both cases the data size and time to execute de- and serialization are measured and logged.
