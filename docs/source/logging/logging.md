# Logging

The driver uses the [tracing](https://github.com/tokio-rs/tracing) crate for all logs.\
There are two ways to view the logs:
- Create a `tracing` subscriber to which all logs will be written (recommended).
- Enable `log` feature on `tracing` crate and use some logger from `log` ecosystem. \
Only do this if you can't use `tracing` subscriber for some reason.

## Using tracing subscriber

To print the logs you can use the default subscriber:

```rust
# extern crate scylla;
# extern crate tokio;
# extern crate tracing;
# extern crate tracing_subscriber;
# use std::error::Error;
# use scylla::client::session::Session;
# use scylla::client::session_builder::SessionBuilder;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Install global collector configured based on RUST_LOG env var
    // This collector will receive logs from the driver
    tracing_subscriber::fmt::init();

    let uri = std::env::var("SCYLLA_URI")
        .unwrap_or_else(|_| "127.0.0.1:9042".to_string());

    info!("Connecting to {}", uri);

    let session: Session = SessionBuilder::new().known_node(uri).build().await?;
    session
        .query_unpaged(
            "CREATE KEYSPACE IF NOT EXISTS ks WITH REPLICATION = \
            {'class' : 'NetworkTopologyStrategy', 'replication_factor' : 1}",
            &[],
        )
        .await?;

    // This query should generate a warning message
    session.query_unpaged("USE ks", &[]).await?;

    Ok(())
}
```

To start this example execute:
```shell
RUST_LOG=info cargo run
```

The full [example](https://github.com/scylladb/scylla-rust-driver/tree/main/examples/logging.rs) is available in the `examples` folder.
You can run it from main folder of driver repository using `RUST_LOG=trace SCYLLA_URI=<scylla_ip>:9042 cargo run --example logging`.

## Using log

To collect tracing events using log collector you first need to enable `log` feature on `tracing` crate.
You can use `cargo add tracing -F log` or edit `Cargo.toml`:
```toml
tracing = { version = "0.1.40" , features = ["log"] }
```
then you can setup `env_logger` os some other logger and it will output logs from the driver:

```rust
# extern crate scylla;
# extern crate tokio;
# extern crate tracing;
# extern crate env_logger;
# use std::error::Error;
# use scylla::client::session::Session;
# use scylla::client::session_builder::SessionBuilder;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Setup `log` collector that uses RUST_LOG env variable to configure
    // verbosity.
    env_logger::init();

    let uri = std::env::var("SCYLLA_URI").unwrap_or_else(|_| "127.0.0.1:9042".to_string());
    info!("Connecting to {}", uri);

    let session: Session = SessionBuilder::new().known_node(uri).build().await?;
    session.query_unpaged("CREATE KEYSPACE IF NOT EXISTS examples_ks WITH REPLICATION = {'class' : 'NetworkTopologyStrategy', 'replication_factor' : 1}", &[]).await?;

    session.query_unpaged("USE examples_ks", &[]).await?;

    Ok(())
}
```

The full [example](https://github.com/scylladb/scylla-rust-driver/tree/main/examples/logging_log.rs) is available in the `examples` folder.
You can run it from main folder of driver repository using `RUST_LOG=trace SCYLLA_URI=<scylla_ip>:9042 cargo run --example logging_log`.