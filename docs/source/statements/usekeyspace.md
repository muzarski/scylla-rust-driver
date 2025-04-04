# USE keyspace

Using a keyspace allows to omit keyspace name in statements.

For example in `cqlsh` one could write:
```sql
cqlsh> SELECT * FROM my_keyspace.table;

 a     | b     |
-------+-------+
 12345 | 54321 |

(1 rows)
cqlsh> USE my_keyspace;
cqlsh:my_keyspace> SELECT * FROM table;

 a     | b     |
-------+-------+
 12345 | 54321 |

(1 rows)

```
Tables from other keyspaces can still easily be accessed by using their keyspace names.
```sql
cqlsh:my_keyspace> SELECT * FROM other_keyspace.other_table;
```

In the driver this can be achieved using `Session::use_keyspace`:
```rust
# extern crate scylla;
# use scylla::client::session::Session;
# use std::error::Error;
# async fn check_only_compiles(session: &Session) -> Result<(), Box<dyn Error>> {
session
    .query_unpaged("INSERT INTO my_keyspace.tab (a) VALUES ('test1')", &[])
    .await?;

session.use_keyspace("my_keyspace", false).await?;

// Now we can omit keyspace name in the statement
session
    .query_unpaged("INSERT INTO tab (a) VALUES ('test2')", &[])
    .await?;
# Ok(())
# }
```

The first argument is the keyspace name.\
The second argument states whether this name is case sensitive.

It is also possible to send raw use keyspace statement using `Session::query_*` instead of `Session::use_keyspace` such as:

```rust
# extern crate scylla;
# use scylla::client::session::Session;
# use std::error::Error;
# async fn check_only_compiles(session: &Session) -> Result<(), Box<dyn Error>> {
session.query_unpaged("USE my_keyspace", &[]).await?;
# Ok(())
# }
```

This method has a slightly worse latency than `Session::use_keyspace` - there are two roundtrips needed instead of one.
Therefore, `Session::use_keyspace` is the preferred method for setting keyspaces.

### Multiple use keyspace requests at once
Don't run multiple `use_keyspace` requests at once. 
This could end up with a part of connections using one keyspace and another part using another.

### Case sensitivity

In CQL a keyspace name can be case insensitive (without `"`) or case sensitive (with `"`).\
If the second argument to `use_keyspace` is set to `true` this keyspace name will be wrapped in `"`.\
It is best to avoid the problem altogether and just not create two keyspaces with the same name but different cases.

Let's see what happens when there are two keyspaces with the same name but different cases: `my_keyspace` and `MY_KEYSPACE`:

```rust
# extern crate scylla;
# use scylla::client::session::Session;
# use std::error::Error;
# async fn check_only_compiles(session: &Session) -> Result<(), Box<dyn Error>> {
// lowercase name without case sensitivity will use my_keyspace
session.use_keyspace("my_keyspace", false).await?;

// lowercase name with case sensitivity will use my_keyspace
session.use_keyspace("my_keyspace", true).await?;

// uppercase name without case sensitivity will use my_keyspace
session.use_keyspace("MY_KEYSPACE", false).await?;

// uppercase name with case sensitivity will use MY_KEYSPACE
session.use_keyspace("MY_KEYSPACE", true).await?;
# Ok(())
# }
```
