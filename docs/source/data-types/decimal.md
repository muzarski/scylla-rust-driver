# Decimal
`Decimal` is represented as `value::CqlDecimal` or [`bigdecimal::BigDecimal`](https://docs.rs/bigdecimal/latest/bigdecimal/struct.BigDecimal.html)

## value::CqlDecimal

Without any feature flags, the user can interact with `decimal` type by making use of `value::CqlDecimal` or `value::CqlDecimalBorrowed` which are very simple wrappers representing the value as signed binary number in big-endian order with a 32-bit scale.

```rust
# extern crate scylla;
# extern crate futures;
# use scylla::client::session::Session;
# use std::error::Error;
# async fn check_only_compiles(session: &Session) -> Result<(), Box<dyn Error>> {
use futures::TryStreamExt;
use scylla::value::CqlDecimal;
use std::str::FromStr;

// Insert a decimal (123.456) into the table
let to_insert: CqlDecimal =
        CqlDecimal::from_signed_be_bytes_and_exponent(vec![0x01, 0xE2, 0x40], 3);
session
    .query_unpaged("INSERT INTO keyspace.table (a) VALUES(?)", (to_insert,))
    .await?;

// Read a decimal from the table
let mut iter = session.query_iter("SELECT a FROM keyspace.table", &[])
    .await?
    .rows_stream::<(CqlDecimal,)>()?;
while let Some((decimal_value,)) = iter.try_next().await? {
    println!("{:?}", decimal_value);
}
# Ok(())
# }
```

## bigdecimal::BigDecimal

To make use of `bigdecimal::Bigdecimal` type, user should enable `bigdecimal-04` crate feature.

```rust
# extern crate scylla;
# extern crate bigdecimal;
# extern crate futures;
# use scylla::client::session::Session;
# use std::error::Error;
# async fn check_only_compiles(session: &Session) -> Result<(), Box<dyn Error>> {
use futures::TryStreamExt;
use bigdecimal::BigDecimal;
use std::str::FromStr;

// Insert a decimal into the table
let to_insert: BigDecimal = BigDecimal::from_str("12345.0")?;
session
    .query_unpaged("INSERT INTO keyspace.table (a) VALUES(?)", (to_insert,))
    .await?;

// Read a decimal from the table
let mut iter = session.query_iter("SELECT a FROM keyspace.table", &[])
    .await?
    .rows_stream::<(BigDecimal,)>()?;
while let Some((decimal_value,)) = iter.try_next().await? {
    println!("{:?}", decimal_value);
}
# Ok(())
# }
```