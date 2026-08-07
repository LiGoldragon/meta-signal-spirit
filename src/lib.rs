//! Meta-signal contract for privileged spirit lifecycle and policy.
//!
//! `schema/meta.schema` is the authored Interface source, rescued from the
//! condemned `spirit-ethos` repository and re-spelled to the blessed 5-section
//! form. `build.rs` authorizes it through `SemaBootstrapAuthority` and
//! generates the Rust projection through `CommitBootstrap`.
//!
// psyche-grasp: unseen

pub mod schema;

pub use crate::schema::meta::*;

/// Canonical textual projection of the meta Interface.
pub const META_INTERFACE_SOURCE: &str = include_str!("../schema/meta.schema");
