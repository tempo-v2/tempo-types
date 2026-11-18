//! Convenience imports for the complete small shared vocabulary.
//!
//! Use `use tempo_types::prelude::*` in a small adapter or example that needs
//! most of these values. Prefer explicit imports in domain modules so readers
//! can see which identities and versions the module actually uses. All items
//! are also available directly from `tempo_types`.

pub use crate::counters::{ContentVersion, MissRevision, Revision, StudyDay};
pub use crate::grade::Grade;
pub use crate::identifiers::{AttemptId, DatasetId, EventId, SubmissionId, TargetId};
pub use crate::study_time::StudyTime;
