//! Which independent collection of Tempo data this record belongs to.
//!
//! Use this at a command or persistence boundary to keep a restored dataset separate
//! from the dataset that produced an old command. The application chooses the ID;
//! Store checks it. Restoring into a fresh dataset gives otherwise identical target
//! IDs a different namespace. This wrapper does not generate or validate IDs.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Which independent collection of Tempo data this record belongs to.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DatasetId(pub String);

impl From<&str> for DatasetId {
    fn from(value: &str) -> Self {
        Self(value.to_owned())
    }
}

impl fmt::Display for DatasetId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn restoring_a_target_into_a_new_dataset_does_not_reuse_the_old_namespace() {
        use crate::TargetId;
        use std::collections::BTreeMap;

        // A persistence adapter can key a record by dataset and target together.
        let target = TargetId::from("repertoire:position-42");
        let original = DatasetId::from("before-restore");
        let restored = DatasetId::from("after-restore");
        let mut records = BTreeMap::new();
        records.insert((original.clone(), target.clone()), "original history");
        records.insert((restored.clone(), target.clone()), "restored history");

        assert_eq!(records.len(), 2);
        assert_eq!(records[&(original, target.clone())], "original history");
        assert_eq!(records[&(restored, target)], "restored history");
    }
}
