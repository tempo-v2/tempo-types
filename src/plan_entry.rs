//! Shared values carried across the prepared-plan persistence boundary.

use crate::{PlanBasis, PlanGeneration, StudyDay, WorkToken};

/// Only Store's complete, published generation can be a serving head.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PlanHead {
    pub generation: PlanGeneration,
    pub basis: PlanBasis,
}

/// Ordering class for ready work. Equal priorities use stable target identity.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum PlanPriority {
    Correction,
    DueTest(StudyDay),
    Teach,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PlanEntryState {
    Ready,
    Issued,
    Deferred,
    Consumed,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PlanEntry {
    pub work: WorkToken,
    pub priority: PlanPriority,
    pub ordinal: u64,
    pub state: PlanEntryState,
}
