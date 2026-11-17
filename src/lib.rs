//! Small shared vocabulary. Domain records belong to their owning crates.
//! This crate defines small shared vocabulary types used across the Tempo project.
pub mod counters;
pub mod identifiers;
pub mod prelude;
pub mod study_time;

pub use crate::counters::*;
pub use crate::identifiers::*;
pub use crate::study_time::*;
