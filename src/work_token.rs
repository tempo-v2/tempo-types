//! Identity shared by issued Study attempts and disposable prepared-order rows.

use crate::{ContentVersion, MissRevision, StudyDay, TargetId};
use serde::{Deserialize, Serialize};

/// The retained Study operation that may consume one work item.
/// Its enum order is only for identity comparisons; Queue owns priority.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkKind {
    Teach,
    Test,
    Correction,
}

/// A target's exact work identity within a dataset, independent of plan day.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct WorkObligation {
    pub target: TargetId,
    pub version: ContentVersion,
    pub kind: WorkKind,
    /// Learning's miss revision observed by this work. Newer misses have new keys.
    pub revision: MissRevision,
}

/// The immutable work key issued out of a particular Study day.
///
/// The enclosing attempt or plan supplies dataset identity. A plan generation is
/// a disposable storage address and is never included in a work key.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct WorkToken {
    pub day: StudyDay,
    pub obligation: WorkObligation,
}

/// Deferral of the same obligation for the day the Skip was accepted.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct DeferredWork {
    pub day: StudyDay,
    pub obligation: WorkObligation,
}

impl WorkToken {
    /// Use the local acceptance day while retaining the original issued token.
    pub fn deferred_on(&self, day: StudyDay) -> DeferredWork {
        DeferredWork {
            day,
            obligation: self.obligation.clone(),
        }
    }
}

impl DeferredWork {
    /// Match this day's candidate only if it still represents the same obligation.
    pub fn matches(&self, work: &WorkToken) -> bool {
        self.day == work.day && self.obligation == work.obligation
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn work_token_preserves_issued_day_and_defers_only_the_same_obligation() {
        let old = WorkToken {
            day: StudyDay(10),
            obligation: WorkObligation {
                target: "target".into(),
                version: ContentVersion(2),
                kind: WorkKind::Test,
                revision: MissRevision(5),
            },
        };
        let current = WorkToken {
            day: StudyDay(11),
            ..old.clone()
        };
        let deferral = old.deferred_on(current.day);
        assert_ne!(old, current);
        assert!(deferral.matches(&current));
        assert!(!deferral.matches(&old));

        let mut newer = current.clone();
        newer.obligation.revision = MissRevision(6);
        assert!(!deferral.matches(&newer));
        newer.obligation.revision = current.obligation.revision;
        newer.obligation.kind = WorkKind::Correction;
        assert!(!deferral.matches(&newer));
        newer.obligation.kind = WorkKind::Test;
        newer.obligation.version = ContentVersion(3);
        assert!(!deferral.matches(&newer));
    }
}
