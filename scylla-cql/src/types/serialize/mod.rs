#![warn(missing_docs)]

//! Types and traits related to serialization of values to the CQL format.

use std::{error::Error, fmt::Display, sync::Arc};

use thiserror::Error;

pub mod batch;
pub mod raw_batch;
pub mod row;
pub mod value;
pub mod writers;

pub use writers::{CellValueBuilder, CellWriter, RowWriter};

/// An error indicating that a failure happened during serialization.
///
/// The error is type-erased so that the crate users can define their own
/// serialization impls and their errors. As for the impls defined or generated
/// by the driver itself, the following errors can be returned:
///
/// - [`row::BuiltinSerializationError`] is returned when serialization of
///   one of types with an impl built into the driver fails. It is also returned
///   from impls generated by the `SerializeRow` macro.
/// - [`value::BuiltinSerializationError`] is analogous to the above but is
///   returned from [`SerializeValue::serialize`](value::SerializeValue::serialize)
///   instead both in the case of builtin impls and impls generated by the
///   `SerializeValue` macro. It won't be returned by the `Session` directly,
///   but it might be nested in the [`row::BuiltinSerializationError`].
/// - [`row::ValueListToSerializeRowAdapterError`] is returned in case when
///   a list of named values encoded with the legacy `ValueList` trait is passed
///   as an argument to the statement, and rewriting it using the new
///   `SerializeRow` interface fails.
#[derive(Debug, Clone, Error)]
pub struct SerializationError(Arc<dyn Error + Send + Sync>);

impl SerializationError {
    /// Constructs a new `SerializationError`.
    #[inline]
    pub fn new(err: impl Error + Send + Sync + 'static) -> SerializationError {
        SerializationError(Arc::new(err))
    }
}

impl Display for SerializationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SerializationError: {}", self.0)
    }
}
