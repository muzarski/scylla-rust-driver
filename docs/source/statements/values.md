# Statement values
Statement text is constant, but the values may change.
You can pass changing values to a statement by specifying a list of variables as bound values.\
Each `?` in statement text will be filled with the matching value. 

> **Never** pass values by adding strings, this could lead to [SQL Injection](https://en.wikipedia.org/wiki/SQL_injection)

Each list of values to send must implement the trait `SerializeRow`.\
By default this can be a slice `&[]`, a tuple `()` (max 16 elements) of values to send,
or a custom struct which derives from `SerializeRow`.

A few examples:
```rust
# extern crate scylla;
# use scylla::SerializeRow;
# use scylla::client::session::Session;
# use scylla::value::CqlValue;
# use std::error::Error;
# use std::collections::HashMap;
# async fn check_only_compiles(session: &Session) -> Result<(), Box<dyn Error>> {
// Empty slice means that there are no values to send
session.query_unpaged("INSERT INTO ks.tab (a) VALUES(1)", &[]).await?;

// Empty tuple/unit also means that there are no values to send
session.query_unpaged("INSERT INTO ks.tab (a) VALUES(1)", ()).await?;

// Sending three integers using a slice:
session
    .query_unpaged("INSERT INTO ks.tab (a, b, c) VALUES(?, ?, ?)", [1_i32, 2, 3].as_ref())
    .await?;

// Sending an integer and a string using a tuple
session
    .query_unpaged("INSERT INTO ks.tab (a, b) VALUES(?, ?)", (2_i32, "Some text"))
    .await?;

// Sending an integer and a string using a named struct.
// Names of fields must match names of columns in request,
// but having them in the same order is not required.
// If the fields are in the same order, you can use attribute:
// `#[scylla(flavor = "enforce_order")]`
// in order to skip sorting the fields and just check if they
// are in the same order. See documentation of this macro
// for more information.
#[derive(SerializeRow)]
struct IntString {
    a: i32,
    b: String,
}

let int_string = IntString {
    a: 42_i32,
    b: "hello".to_owned(),
};

session
    .query_unpaged("INSERT INTO ks.tab (a, b) VALUES(?, ?)", int_string)
    .await?;

// You can use named bind markers in statement if you want
// your names in struct to be different than column names.
#[derive(SerializeRow)]
struct IntStringCustom {
    first_value: i32,
    second_value: String,
}

let int_string_custom = IntStringCustom {
    first_value: 42_i32,
    second_value: "hello".to_owned(),
};

session
    .query_unpaged("INSERT INTO ks.tab (a, b) VALUES(:first_value, :second_value)", int_string_custom)
    .await?;

// Sending a single value as a tuple requires a trailing coma (Rust syntax):
session.query_unpaged("INSERT INTO ks.tab (a) VALUES(?)", (2_i32,)).await?;

// Each value can also be sent using a reference:
session
    .query_unpaged("INSERT INTO ks.tab (a, b) VALUES(?, ?)", &(&2_i32, &"Some text"))
    .await?;

// A map of named values can also be provided:
let mut vals: HashMap<&str, CqlValue> = HashMap::new();
vals.insert("avalue", CqlValue::Text("hello".to_string()));
vals.insert("bvalue", CqlValue::Int(17));
session
    .query_unpaged("INSERT INTO ks.tab (a, b) VALUES(:avalue, :bvalue)", &vals)
    .await?;

# Ok(())
# }
```

### `NULL` values
Null values can be sent using `Option<>` - sending a `None` will make the value `NULL`:
```rust
# extern crate scylla;
# use scylla::client::session::Session;
# use std::error::Error;
# async fn check_only_compiles(session: &Session) -> Result<(), Box<dyn Error>> {
let null_i32: Option<i32> = None;
session
    .query_unpaged("INSERT INTO ks.tab (a) VALUES(?)", (null_i32,))
    .await?;
# Ok(())
# }
```

### `Unset` values
When performing an insert with values which might be `NULL`, it's better to use `Unset`.\
Database treats inserting `NULL` as a delete operation and will generate a tombstone.
Using `Unset` results in better performance:

```rust
# extern crate scylla;
# use scylla::client::session::Session;
# use std::error::Error;
# async fn check_only_compiles(session: &Session) -> Result<(), Box<dyn Error>> {
use scylla::value::{MaybeUnset, Unset};

// Inserting a null results in suboptimal performance
let null_i32: Option<i32> = None;
session
    .query_unpaged("INSERT INTO ks.tab (a) VALUES(?)", (null_i32,))
    .await?;

// Using MaybeUnset enum is better
let unset_i32: MaybeUnset<i32> = MaybeUnset::Unset;
session
    .query_unpaged("INSERT INTO ks.tab (a) VALUES(?)", (unset_i32,))
    .await?;

// If we are sure that a value should be unset we can simply use Unset
session
    .query_unpaged("INSERT INTO ks.tab (a) VALUES(?)", (Unset,))
    .await?;
# Ok(())
# }
```
See the [issue](https://issues.apache.org/jira/browse/CASSANDRA-7304) for more information about `Unset`

### Other data types
See [Data Types](../data-types/data-types.md) for instructions on sending other data types
