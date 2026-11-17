//! Distinguish independent versions and the configured Study day.
//!
//! These numbers are deliberately different types. A response revision cannot
//! stand in for a content version, and a miss revision cannot stand in for either.
//! [`StudyDay`] shares the numeric-wrapper folder for compatibility, but is a day
//! coordinate rather than a mutation counter. The owning component validates
//! and advances values; the wrappers have no hidden mutation or clock access.

pub mod content_version;
pub mod miss_revision;
pub mod revision;
pub mod study_day;

pub use content_version::ContentVersion;
pub use miss_revision::MissRevision;
pub use revision::Revision;
pub use study_day::StudyDay;
