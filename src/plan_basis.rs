//! Immutable identities captured with prepared Study projections.

use crate::{DatasetId, StudyDay};
use serde::{Deserialize, Serialize};

/// An allocated prepared order. Issued work outlives these generations.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PlanGeneration(pub u64);

/// Fence for changes to which work can be issued or its validity.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct MembershipEpoch(pub u64);

/// Order-only change that can keep an otherwise valid order available.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct RankRevision(pub u64);

/// Fence for counts, separate from ranking and plan generation identity.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SummaryEpoch(pub u64);

/// Fence a prepared order against the original facts used to build it.
///
/// Store advances membership for relevant authority changes, including changes
/// to the configured day boundary, content, or admission policy.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlanBasis {
    pub dataset: DatasetId,
    pub day: StudyDay,
    pub membership_epoch: MembershipEpoch,
    pub rank_revision: RankRevision,
}

/// The original count basis, with no dependence on plan ordering.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SummaryBasis {
    pub dataset: DatasetId,
    pub day: StudyDay,
    pub epoch: SummaryEpoch,
}
