# Query result

`Session::query_unpaged`, `Session::query_single_page`, `Session::execute_unpaged` and `Session::execute_single_page`
return a `QueryResult` with rows represented as `Option<Vec<Row>>`.

> ***Note***\
> Using unpaged queries for SELECTs is discouraged in general.
> Query results may be so big that it is not preferable to fetch them all at once.
> Even with small results, if there are a lot of tombstones, then there can be similar bad consequences.
> However, `query_unpaged` will return all results in one, possibly giant, piece
> (unless a timeout occurs due to high load incurred by the cluster).
> This:
> - increases latency,
> - has large memory footprint,
> - puts high load on the cluster,
> - is more likely to time out (because big work takes more time than little work,
>   and returning one large piece of data is more work than returning one chunk of data).

> To sum up, **for SELECTs** (especially those that may return a lot of data) **prefer paged queries**,
> e.g. with `Session::query_iter()` (see [Paged queries](paged.md)).

## Parsing using convenience methods

By calling [`QueryResult::into_rows_result`](https://docs.rs/scylla/latest/scylla/response/query_result/struct.QueryResult.html#method.into_rows_result),
one can obtain  [`QueryRowsResult`](https://docs.rs/scylla/latest/scylla/response/query_result/struct.QueryRowsResult.html).
`QueryRowsResult` provides convenience methods for parsing rows.
Here are a few of them:
* `rows::<RowT>()` - returns the rows parsed as the given type
* `maybe_first_row::<RowT>()` - returns the first received row or `None` if there are no rows
* `first_row::<RowT>()` - returns the first received row; fails if there are no rows
* `single_row::<RowT>()` - same as `first_row`, but fails when there is more than one row

Additionally, [`QueryResult`](https://docs.rs/scylla/latest/scylla/response/query_result/struct.QueryResult.html) has a method `result_not_rows()`, which ensures that query response was not `rows` and thus helps avoid bugs.

```rust
# extern crate scylla;
# use scylla::client::session::Session;
# use std::error::Error;
# async fn check_only_compiles(session: &Session) -> Result<(), Box<dyn Error>> {
// Parse row as a single column containing an int value
let result = session
    .query_unpaged("SELECT a from ks.tab", &[])
    .await?
    .into_rows_result()?;

for row in result.rows::<(i32,)>()? {
    let (int_value,): (i32,) = row?;
}

// first_row gets the first row and parses it as the given type
let first_int_val: (i32,) = session
    .query_unpaged("SELECT a from ks.tab", &[])
    .await?
    .into_rows_result()?
    .first_row::<(i32,)>()?;

// result_not_rows fails when the response is rows
session.query_unpaged("INSERT INTO ks.tab (a) VALUES (0)", &[]).await?.result_not_rows()?;
# Ok(())
# }
```
For more see [`QueryResult`](https://docs.rs/scylla/latest/scylla/response/query_result/struct.QueryResult.html)
and [`QueryRowsResult`](https://docs.rs/scylla/latest/scylla/response/query_result/struct.QueryRowsResult.html)

### `NULL` values
`NULL` values will return an error when parsed as a Rust type. 
To properly handle `NULL` values parse column as an `Option<>`:
```rust
# extern crate scylla;
# use scylla::client::session::Session;
# use std::error::Error;
# async fn check_only_compiles(session: &Session) -> Result<(), Box<dyn Error>> {

// Parse row as two columns containing an int and text which might be null
let rows_result = session
    .query_unpaged("SELECT a, b from ks.tab", &[])
    .await?
    .into_rows_result()?;

for row in rows_result.rows::<(i32, Option<&str>)>()? {
    let (int_value, str_or_null): (i32, Option<&str>) = row?;
}
# Ok(())
# }
```

### Parsing row as a custom struct
It is possible to receive row as a struct with fields matching the columns.\
The struct must:
* have the same number of fields as the number of queried columns
* have field types matching the columns being received
* derive `DeserializeRow`

Field names don't need to match column names.
```rust
# extern crate scylla;
# use scylla::client::session::Session;
# use std::error::Error;
# async fn check_only_compiles(session: &Session) -> Result<(), Box<dyn Error>> {
use scylla::DeserializeRow;
use scylla::deserialize::row::DeserializeRow;

#[derive(DeserializeRow)]
struct MyRow {
    age: i32,
    name: Option<String>,
}

// Parse row as two columns containing an int and text which might be null
let result_rows = session
    .query_unpaged("SELECT a, b from ks.tab", &[])
    .await?
    .into_rows_result()?;

for row in result_rows.rows::<MyRow>()? {
    let my_row: MyRow = row?;
}
# Ok(())
# }
```

### Other data types
For parsing other data types see [Data Types](../data-types/data-types.md)
