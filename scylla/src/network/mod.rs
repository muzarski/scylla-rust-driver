//! This module holds entities that represent connections to the cluster
//! and management over those connections (connection pooling).
//! This includes two main abstractions:
//! - Connection - a single, possibly encrypted, connection to a Scylla node over CQL protocol,
//! - NodeConnectionPool - a manager that keeps a desired number of connections opened to each shard.

mod connection;
#[cfg(feature = "ssl")]
pub(crate) use connection::SslConfig;
pub(crate) use connection::{Connection, ConnectionConfig, VerifiedKeyspaceName};

mod connection_pool;

pub use connection_pool::PoolSize;
pub(crate) use connection_pool::{NodeConnectionPool, PoolConfig};
