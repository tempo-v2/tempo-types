//! Identify the dataset, target, attempt, command, or observation being discussed.
//!
//! Each ID is a distinct type even though all wrap a string. Pick the type whose
//! lifetime matches the thing you mean; open its module for a concrete scenario.
//! The type does not enforce uniqueness or a string format.
//!
//! | Question | Type |
//! | --- | --- |
//! | Which independent collection of data? | [`DatasetId`] |
//! | Which learnable item, across many attempts? | [`TargetId`] |
//! | Which run through that item? | [`AttemptId`] |
//! | Which answer command, including retries? | [`SubmissionId`] |
//! | Which external observation? | [`EventId`] |
//!
//! The compiler prevents an accidental ID substitution:
//!
//! ```compile_fail
//! use tempo_types::{AttemptId, SubmissionId};
//! fn resume_attempt(_attempt: AttemptId) {}
//! resume_attempt(SubmissionId::from("answer-command"));
//! ```

pub mod attempt_id;
pub mod dataset_id;
pub mod event_id;
pub mod submission_id;
pub mod target_id;

pub use attempt_id::AttemptId;
pub use dataset_id::DatasetId;
pub use event_id::EventId;
pub use submission_id::SubmissionId;
pub use target_id::TargetId;
