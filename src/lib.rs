//! Small values shared across Tempo's independent components.
//!
//! Start with [`identifiers`] for **which thing**, [`counters`] for **which
//! version or day**, [`StudyTime`] for application-supplied time, and [`Grade`]
//! for a terminal outcome. Each type has its own file and a usage scenario.
//!
//! These wrappers distinguish meanings at compile time; they do not generate
//! IDs, validate strings, read a clock, schedule a review, or persist records.
//! Rich records stay in the component that owns their behavior. This crate
//! depends on no other Tempo crate.

pub mod counters;
pub mod grade;
pub mod identifiers;
pub mod plan_basis;
pub mod prelude;
pub mod study_time;
pub mod work_token;

pub use counters::{ContentVersion, MissRevision, Revision, StudyDay};
pub use grade::Grade;
pub use identifiers::{AttemptId, DatasetId, EventId, SubmissionId, TargetId};
pub use plan_basis::{
    MembershipEpoch, PlanBasis, PlanGeneration, RankRevision, SummaryBasis, SummaryEpoch,
};
pub use study_time::StudyTime;
pub use work_token::{DeferredWork, WorkKind, WorkObligation, WorkToken};
