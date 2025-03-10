# Date

Depending on feature flags, three different types can be used to interact with date.

Internally [date](https://docs.scylladb.com/stable/cql/types.html#dates) is represented as number of days since
-5877641-06-23 i.e. 2^31 days before unix epoch.

## CqlDate

Without any extra features enabled, only `value::CqlDate` is available. It's an
[`u32`](https://doc.rust-lang.org/std/primitive.u32.html) wrapper and it matches the internal date representation.

However, for most use cases other types are more practical. See following sections for `chrono` and `time`.

```rust
# extern crate scylla;
# extern crate futures;
# use scylla::client::session::Session;
# use std::error::Error;
# async fn check_only_compiles(session: &Session) -> Result<(), Box<dyn Error>> {
use scylla::value::CqlDate;
use futures::TryStreamExt;

// 1970-01-08
let to_insert = CqlDate((1 << 31) + 7);

// Insert date into the table
session
    .query_unpaged("INSERT INTO keyspace.table (a) VALUES(?)", (to_insert,))
    .await?;

// Read raw Date from the table
let mut iter = session.query_iter("SELECT a FROM keyspace.table", &[])
    .await?
    .rows_stream::<(CqlDate,)>()?;
while let Some((date_value,)) = iter.try_next().await? {
    // ...
}
# Ok(())
# }
```

## chrono::NaiveDate

If full range is not required and `chrono-04` feature is enabled,
[`chrono::NaiveDate`](https://docs.rs/chrono/0.4/chrono/naive/struct.NaiveDate.html) can be used.
[`chrono::NaiveDate`](https://docs.rs/chrono/0.4/chrono/naive/struct.NaiveDate.html) supports dates from
-262145-01-01 to 262143-12-31.

```rust
# extern crate chrono;
# extern crate scylla;
# extern crate futures;
# use scylla::client::session::Session;
# use std::error::Error;
# async fn check_only_compiles(session: &Session) -> Result<(), Box<dyn Error>> {
use chrono::NaiveDate;
use futures::TryStreamExt;

// 2021-03-24
let to_insert = NaiveDate::from_ymd_opt(2021, 3, 24).unwrap();

// Insert date into the table
session
    .query_unpaged("INSERT INTO keyspace.table (a) VALUES(?)", (to_insert,))
    .await?;

// Read NaiveDate from the table
let mut iter = session.query_iter("SELECT a FROM keyspace.table", &[])
    .await?
    .rows_stream::<(NaiveDate,)>()?;
while let Some((date_value,)) = iter.try_next().await? {
    // ...
}
# Ok(())
# }
```

## time::Date

Alternatively, the `time-03` feature can be used to enable support of
[`time::Date`](https://docs.rs/time/0.3/time/struct.Date.html).
[`time::Date`](https://docs.rs/time/0.3/time/struct.Date.html)'s value range depends on feature flags, see its
documentation to get more info.

```rust
# extern crate scylla;
# extern crate time;
# extern crate futures;
# use scylla::client::session::Session;
# use std::error::Error;
# async fn check_only_compiles(session: &Session) -> Result<(), Box<dyn Error>> {
use futures::TryStreamExt;
use time::{Date, Month};

// 2021-03-24
let to_insert = Date::from_calendar_date(2021, Month::March, 24).unwrap();

// Insert date into the table
session
    .query_unpaged("INSERT INTO keyspace.table (a) VALUES(?)", (to_insert,))
    .await?;

// Read Date from the table
let mut iter = session.query_iter("SELECT a FROM keyspace.table", &[])
    .await?
    .rows_stream::<(Date,)>()?;
while let Some((date_value,)) = iter.try_next().await? {
    // ...
}
# Ok(())
# }
```
