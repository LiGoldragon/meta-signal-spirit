//! Strict Rust projection of the authored meta Interface.
//!
//! Generation gaps (licensed breakage per hqu.14 replacement-kills):
//! - rust-logos does not yet emit `#[derive(...)]` attributes (no Clone,
//!   rkyv, nota, etc.)
//! - rust-logos does not yet emit `Display` / `Error` implementations
//!   for Refusal types (manually supplied below)
//! - The old schema-rust pipeline generated full derives, accessor methods,
//!   signal-frame integration, rkyv serialization, nota encoding, short
//!   headers, route enums, and From impls; this bare projection does not
//!   yet replicate that surface
//!
// psyche-grasp: unseen

#![allow(dead_code, non_camel_case_types)]

/// Rust mapping of the Ethos `Unit` builtin — a type with exactly one
/// inhabitant, isomorphic to `()`.
pub type Unit = ();

/// Rust mapping of the Ethos `Integer` builtin.
pub type Integer = i64;

include!("generated.rs");

// Generation gap: the Refusal role trait requires `std::error::Error`,
// which requires `Debug + Display`. rust-logos does not yet emit these.
impl std::fmt::Debug for ConfigureRejectionReason {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ArchiveTargetUnwritable => formatter.write_str("ArchiveTargetUnwritable"),
            Self::InternalError => formatter.write_str("InternalError"),
        }
    }
}

impl std::fmt::Debug for ConfigureRejection {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("ConfigureRejection")
            .field("reason", &self.0)
            .finish_non_exhaustive()
    }
}

impl std::fmt::Debug for Rejected {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_tuple("Rejected")
            .field(&self.0)
            .finish()
    }
}

impl std::fmt::Display for Rejected {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "configure rejected: {:?}", self.0)
    }
}

impl std::error::Error for Rejected {}
