/// Derive macro for the [`FromRow`](crate::frame::response::cql_to_rust::FromRow) trait
/// which deserializes a row to given Rust structure.
///
/// It is supported for structs with either named or unnamed fields.
/// It works only for simple structs without generics etc.
///
/// ---
///
#[deprecated(
    since = "0.15.0",
    note = "Legacy deserialization API is inefficient and is going to be removed soon"
)]
pub use scylla_cql::macros::FromRow;

/// #[derive(FromUserType)] allows to parse struct as a User Defined Type
///
/// Works only on simple structs without generics etc
///
/// ---
///
#[deprecated(
    since = "0.15.0",
    note = "Legacy deserialization API is inefficient and is going to be removed soon"
)]
pub use scylla_cql::macros::FromUserType;

/// #[derive(IntoUserType)] allows to pass struct a User Defined Type Value in queries
///
/// Works only on simple structs without generics etc
///
/// ---
///
#[deprecated(
    since = "0.15.1",
    note = "Legacy serialization API is not type-safe and is going to be removed soon"
)]
#[allow(deprecated)]
pub use scylla_cql::macros::IntoUserType;

/// Derive macro for the [`SerializeValue`](crate::serialize::value::SerializeValue) trait
/// which serializes given Rust structure as a User Defined Type (UDT).
///
/// At the moment, only structs with named fields are supported.
///
/// Serialization will fail if there are some fields in the Rust struct that don't match
/// to any of the UDT fields.
///
/// If there are fields in UDT that are not present in Rust definition:
/// - serialization will succeed in "match_by_name" flavor (default). Missing
///   fields in the middle of UDT will be sent as NULLs, missing fields at the end will not be sent
///   at all.
/// - serialization will succeed if suffix of UDT fields is missing. If there are missing fields in the
///   middle it will fail. Note that if "skip_name_checks" is enabled, and the types happen to match,
///   it is possible for serialization to succeed with unexpected result.
///
/// This behavior is the default to support ALTERing UDTs by adding new fields.
/// You can forbid excess fields in the UDT using `forbid_excess_udt_fields` attribute.
///
/// In case of failure, either [`BuiltinTypeCheckError`](crate::serialize::value::BuiltinTypeCheckError)
/// or [`BuiltinSerializationError`](crate::serialize::value::BuiltinSerializationError)
/// will be returned.
///
/// # Example
///
/// A UDT defined like this:
///
/// ```text
/// CREATE TYPE ks.my_udt (a int, b text, c blob);
/// ```
///
/// ...can be serialized using the following struct:
///
/// ```rust
/// # use scylla::SerializeValue;
/// #[derive(SerializeValue)]
/// struct MyUdt {
///     a: i32,
///     b: Option<String>,
///     // No "c" field - it is not mandatory by default for all fields to be present
/// }
/// ```
///
/// # Struct attributes
///
/// `#[scylla(flavor = "flavor_name")]`
///
/// Allows to choose one of the possible "flavors", i.e. the way how the
/// generated code will approach serialization. Possible flavors are:
///
/// - `"match_by_name"` (default) - the generated implementation _does not
///   require_ the fields in the Rust struct to be in the same order as the
///   fields in the UDT. During serialization, the implementation will take
///   care to serialize the fields in the order which the database expects.
/// - `"enforce_order"` - the generated implementation _requires_ the fields
///   in the Rust struct to be in the same order as the fields in the UDT.
///   If the order is incorrect, type checking/serialization will fail.
///   This is a less robust flavor than `"match_by_name"`, but should be
///   slightly more performant as it doesn't need to perform lookups by name.
///
/// `#[scylla(crate = crate_name)]`
///
/// By default, the code generated by the derive macro will refer to the items
/// defined by the driver (types, traits, etc.) via the `::scylla` path.
/// For example, it will refer to the [`SerializeValue`](crate::serialize::value::SerializeValue) trait
/// using the following path:
///
/// ```rust,ignore
/// use ::scylla::_macro_internal::SerializeValue;
/// ```
///
/// Most users will simply add `scylla` to their dependencies, then use
/// the derive macro and the path above will work. However, there are some
/// niche cases where this path will _not_ work:
///
/// - The `scylla` crate is imported under a different name,
/// - The `scylla` crate is _not imported at all_ - the macro actually
///   is defined in the `scylla-macros` crate and the generated code depends
///   on items defined in `scylla-cql`.
///
/// It's not possible to automatically resolve those issues in the procedural
/// macro itself, so in those cases the user must provide an alternative path
/// to either the `scylla` or `scylla-cql` crate.
///
/// `#[scylla(skip_name_checks)]`
///
/// _Specific only to the `enforce_order` flavor._
///
/// Skips checking Rust field names against names of the UDT fields. With this
/// annotation, the generated implementation will allow mismatch between Rust
/// struct field names and UDT field names, i.e. it's OK if i-th field has a
/// different name in Rust and in the UDT. Fields are still being type-checked.
///
/// `#[scylla(forbid_excess_udt_fields)]`
///
/// Forces Rust struct to have all the fields present in UDT, otherwise
/// serialization fails.
///
/// # Field attributes
///
/// `#[scylla(rename = "name_in_the_udt")]`
///
/// Serializes the field to the UDT struct field with given name instead of
/// its Rust name.
///
/// `#[scylla(skip)]`
///
/// Don't use the field during serialization.
///
/// ---
///
pub use scylla_cql::macros::SerializeValue;

/// Derive macro for the [`SerializeRow`](crate::serialize::row::SerializeRow) trait
/// which serializes given Rust structure into bind markers for a CQL statement.
///
/// At the moment, only structs with named fields are supported.
///
/// Serialization will fail if there are some bind markers/columns in the statement
/// that don't match to any of the Rust struct fields, _or vice versa_.
///
/// In case of failure, either [`BuiltinTypeCheckError`](crate::serialize::row::BuiltinTypeCheckError)
/// or [`BuiltinSerializationError`](crate::serialize::row::BuiltinSerializationError)
/// will be returned.
///
/// # Example
///
/// A UDT defined like this:
/// Given a table and a query:
///
/// ```text
/// CREATE TABLE ks.my_t (a int PRIMARY KEY, b text, c blob);
/// INSERT INTO ks.my_t (a, b, c) VALUES (?, ?, ?);
/// ```
///
/// ...the values for the query can be serialized using the following struct:
///
/// ```rust
/// # use scylla::SerializeRow;
/// #[derive(SerializeRow)]
/// struct MyValues {
///     a: i32,
///     b: Option<String>,
///     c: Vec<u8>,
/// }
/// ```
///
/// # Struct attributes
///
/// `#[scylla(flavor = "flavor_name")]`
///
/// Allows to choose one of the possible "flavors", i.e. the way how the
/// generated code will approach serialization. Possible flavors are:
///
/// - `"match_by_name"` (default) - the generated implementation _does not
///   require_ the fields in the Rust struct to be in the same order as the
///   columns/bind markers. During serialization, the implementation will take
///   care to serialize the fields in the order which the database expects.
/// - `"enforce_order"` - the generated implementation _requires_ the fields
///   in the Rust struct to be in the same order as the columns/bind markers.
///   If the order is incorrect, type checking/serialization will fail.
///   This is a less robust flavor than `"match_by_name"`, but should be
///   slightly more performant as it doesn't need to perform lookups by name.
///
/// `#[scylla(crate = crate_name)]`
///
/// By default, the code generated by the derive macro will refer to the items
/// defined by the driver (types, traits, etc.) via the `::scylla` path.
/// For example, it will refer to the [`SerializeRow`](crate::serialize::row::SerializeRow) trait
/// using the following path:
///
/// ```rust,ignore
/// use ::scylla::_macro_internal::SerializeRow;
/// ```
///
/// Most users will simply add `scylla` to their dependencies, then use
/// the derive macro and the path above will work. However, there are some
/// niche cases where this path will _not_ work:
///
/// - The `scylla` crate is imported under a different name,
/// - The `scylla` crate is _not imported at all_ - the macro actually
///   is defined in the `scylla-macros` crate and the generated code depends
///   on items defined in `scylla-cql`.
///
/// It's not possible to automatically resolve those issues in the procedural
/// macro itself, so in those cases the user must provide an alternative path
/// to either the `scylla` or `scylla-cql` crate.
///
/// `#[scylla(skip_name_checks)]
///
/// _Specific only to the `enforce_order` flavor._
///
/// Skips checking Rust field names against names of the columns / bind markers.
/// With this annotation, the generated implementation will allow mismatch
/// between Rust struct field names and the column / bind markers, i.e. it's
/// OK if i-th Rust struct field has a different name than the column / bind
/// marker. The values are still being type-checked.
///
/// # Field attributes
///
/// `#[scylla(rename = "column_or_bind_marker_name")]`
///
/// Serializes the field to the column / bind marker with given name instead of
/// its Rust name.
///
/// `#[scylla(skip)]`
///
/// Don't use the field during serialization.
///
/// ---
///
pub use scylla_cql::macros::SerializeRow;

/// Derive macro for the `DeserializeValue` trait that generates an implementation
/// which deserializes a User Defined Type with the same layout as the Rust
/// struct.
///
/// At the moment, only structs with named fields are supported.
///
/// This macro properly supports structs with lifetimes, meaning that you can
/// deserialize UDTs with fields that borrow memory from the serialized response.
///
/// # Example
///
/// A UDT defined like this:
///
/// ```text
/// CREATE TYPE ks.my_udt (a i32, b text, c blob);
/// ```
///
/// ...can be deserialized using the following struct:
///
/// ```rust
/// # use scylla_cql::macros::DeserializeValue;
/// #[derive(DeserializeValue)]
/// # #[scylla(crate = "scylla_cql")]
/// struct MyUdt<'a> {
///     a: i32,
///     b: Option<String>,
///     c: &'a [u8],
/// }
/// ```
///
/// # Attributes
///
/// The macro supports a number of attributes that customize the generated
/// implementation. Many of the attributes were inspired by procedural macros
/// from `serde` and try to follow the same naming conventions.
///
/// ## Struct attributes
///
/// `#[scylla(crate = "crate_name")]`
///
/// By default, the code generated by the derive macro will refer to the items
/// defined by the driver (types, traits, etc.) via the `::scylla` path.
/// For example, it will refer to the [`DeserializeValue`](crate::deserialize::DeserializeValue)
/// trait using the following path:
///
/// ```rust,ignore
/// use ::scylla::_macro_internal::DeserializeValue;
/// ```
///
/// Most users will simply add `scylla` to their dependencies, then use
/// the derive macro and the path above will work. However, there are some
/// niche cases where this path will _not_ work:
///
/// - The `scylla` crate is imported under a different name,
/// - The `scylla` crate is _not imported at all_ - the macro actually
///   is defined in the `scylla-macros` crate and the generated code depends
///   on items defined in `scylla-cql`.
///
/// It's not possible to automatically resolve those issues in the procedural
/// macro itself, so in those cases the user must provide an alternative path
/// to either the `scylla` or `scylla-cql` crate.
///
///
/// `#[scylla(flavor = "flavor_name")]`
///
/// Allows to choose one of the possible "flavors", i.e. the way how the
/// generated code will approach deserialization. Possible flavors are:
///
/// - `"match_by_name"` (default) - the generated implementation _does not
///   require_ the fields in the Rust struct to be in the same order as the
///   fields in the UDT. During deserialization, the implementation will take
///   care to deserialize the fields in the order which the database expects.
/// - `"enforce_order"` - the generated implementation _requires_ the fields
///   in the Rust struct to be in the same order as the fields in the UDT.
///   If the order is incorrect, type checking/deserialization will fail.
///   This is a less robust flavor than `"match_by_name"`, but should be
///   slightly more performant as it doesn't need to perform lookups by name.
///   The UDT field names will still be checked during the type check phase.
///
/// #[(scylla(skip_name_checks))]
///
/// This attribute only works when used with `flavor = "enforce_order"`.
///
/// If set, the generated implementation will not verify the UDT field names at
/// all. Because it only works with `enforce_order`, it will deserialize first
/// UDT field into the first struct field, second UDT field into the second
/// struct field and so on. It will still verify that the UDT field types
/// and struct field types match.
///
/// #[(scylla(forbid_excess_udt_fields))]
///
/// By default, the generated deserialization code ignores excess UDT fields.
/// I.e., `enforce_order` flavour ignores excess UDT fields in the suffix
/// of the UDT definition, and the default unordered flavour ignores excess
/// UDT fields anywhere.
/// If more strictness is desired, this flag makes sure that no excess fields
/// are present and forces error in case there are some.
///
/// ## Field attributes
///
/// `#[scylla(skip)]`
///
/// The field will be completely ignored during deserialization and will
/// be initialized with `Default::default()`.
///
/// `#[scylla(allow_missing)]`
///
/// If the UDT definition does not contain this field, it will be initialized
/// with `Default::default()`.
///
/// `#[scylla(default_when_null)]`
///
/// If the value of the field received from DB is null, the field will be
/// initialized with `Default::default()`.
///
/// `#[scylla(rename = "field_name")]`
///
/// By default, the generated implementation will try to match the Rust field
/// to a UDT field with the same name. This attribute instead allows to match
/// to a UDT field with provided name.
pub use scylla_macros::DeserializeValue;

/// Derive macro for the `DeserializeRow` trait that generates an implementation
/// which deserializes a row with a similar layout to the Rust struct.
///
/// At the moment, only structs with named fields are supported.
///
/// This macro properly supports structs with lifetimes, meaning that you can
/// deserialize columns that borrow memory from the serialized response.
///
/// # Example
///
/// Having a table defined like this:
///
/// ```text
/// CREATE TABLE ks.my_table (a PRIMARY KEY, b text, c blob);
/// ```
///
/// results of a query "SELECT * FROM ks.my_table"
/// or "SELECT a, b, c FROM ks.my_table"
/// can be deserialized using the following struct:
///
/// ```rust
/// # use scylla_cql::macros::DeserializeRow;
/// #[derive(DeserializeRow)]
/// # #[scylla(crate = "scylla_cql")]
/// struct MyRow<'a> {
///     a: i32,
///     b: Option<String>,
///     c: &'a [u8],
/// }
/// ```
///
/// In general, the struct must match the queried names and types,
/// not the table itself. For example, the query
/// "SELECT a AS b FROM ks.my_table" executed against
/// the aforementioned table can be deserialized to the struct:
/// ```rust
/// # use scylla_cql::macros::DeserializeRow;
/// #[derive(DeserializeRow)]
/// # #[scylla(crate = "scylla_cql")]
/// struct MyRow {
///     b: i32,
/// }
/// ```
///
/// # Attributes
///
/// The macro supports a number of attributes that customize the generated
/// implementation. Many of the attributes were inspired by procedural macros
/// from `serde` and try to follow the same naming conventions.
///
/// ## Struct attributes
///
/// `#[scylla(crate = "crate_name")]`
///
/// By default, the code generated by the derive macro will refer to the items
/// defined by the driver (types, traits, etc.) via the `::scylla` path.
/// For example, it will refer to the [`DeserializeValue`](crate::deserialize::DeserializeValue)
/// trait using the following path:
///
/// ```rust,ignore
/// use ::scylla::_macro_internal::DeserializeValue;
/// ```
///
/// Most users will simply add `scylla` to their dependencies, then use
/// the derive macro and the path above will work. However, there are some
/// niche cases where this path will _not_ work:
///
/// - The `scylla` crate is imported under a different name,
/// - The `scylla` crate is _not imported at all_ - the macro actually
///   is defined in the `scylla-macros` crate and the generated code depends
///   on items defined in `scylla-cql`.
///
/// It's not possible to automatically resolve those issues in the procedural
/// macro itself, so in those cases the user must provide an alternative path
/// to either the `scylla` or `scylla-cql` crate.
///
/// `#[scylla(flavor = "flavor_name")]`
///
/// Allows to choose one of the possible "flavors", i.e. the way how the
/// generated code will approach deserialization. Possible flavors are:
///
/// - `"match_by_name"` (default) - the generated implementation _does not
///   require_ the fields in the Rust struct to be in the same order as the
///   columns in the row. During deserialization, the implementation will take
///   care to deserialize the columns in the order which the database provided.
/// - `"enforce_order"` - the generated implementation _requires_ the fields
///   in the Rust struct to be in the same order as the columns in the row.
///   If the order is incorrect, type checking/deserialization will fail.
///   This is a less robust flavor than `"match_by_name"`, but should be
///   slightly more performant as it doesn't need to perform lookups by name.
///   The generated code will still check that the column and field names match.
///
/// #[(scylla(skip_name_checks))]
///
/// This attribute only works when used with `flavor = "enforce_order"`.
///
/// If set, the generated implementation will not verify the column names at
/// all. Because it only works with `enforce_order`, it will deserialize first
/// column into the first field, second column into the second field and so on.
/// It will still still verify that the column types and field types match.
///
/// ## Field attributes
///
/// `#[scylla(skip)]`
///
/// The field will be completely ignored during deserialization and will
/// be initialized with `Default::default()`.
///
/// `#[scylla(rename = "field_name")]`
///
/// By default, the generated implementation will try to match the Rust field
/// to a column with the same name. This attribute allows to match to a column
/// with provided name.
pub use scylla_macros::DeserializeRow;

/// #[derive(ValueList)] allows to pass struct as a list of values for a query
///
/// ---
///
#[deprecated(
    since = "0.15.1",
    note = "Legacy serialization API is not type-safe and is going to be removed soon"
)]
pub use scylla_cql::macros::ValueList;

#[deprecated(
    since = "0.15.0",
    note = "Legacy deserialization API is inefficient and is going to be removed soon"
)]
#[allow(deprecated)]
pub use scylla_cql::macros::impl_from_cql_value_from_method;

#[allow(deprecated)]
pub use scylla_cql::macros::impl_serialize_row_via_value_list;
#[allow(deprecated)]
pub use scylla_cql::macros::impl_serialize_value_via_value;

// Reexports for derive(IntoUserType)
pub use bytes::{BufMut, Bytes, BytesMut};
