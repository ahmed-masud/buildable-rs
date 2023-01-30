//! Builder binding for Stores and Store managers
//! This ensures that every manager is buildable.
//! We use this in conjunction with builder pattern to create
//! a new object.

// #[cfg(feature = "serde")]
// use serde::{Deserialize, Serialize};

/// Build function trait for storage drivers.


pub trait Buildable {
    /// A builder type for this driver.
    type Builder: Default;

    /// Build Errors.
    type BuilderError: std::error::Error;

    /// Provide a builder for this concrete type.
    fn builder() -> Self::Builder {
        Self::Builder::default()
    }

    /// Build from the builder.
    fn build(builder: Self::Builder) -> Result<Self, Self::BuilderError>
    where
        Self: Sized;
}
