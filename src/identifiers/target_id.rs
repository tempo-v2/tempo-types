//! Which learnable item accumulates history across attempts.
//!
//! Use this when referring to a repertoire position or a complete tactics exercise.
//! A seven-decision tactic still has one target. Chess supplies the target identity;
//! Queue selects it, Study carries it, and Learning state is stored against it.
//! Start a new attempt without changing the target ID. A content version identifies
//! which definition of that target was used.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Which learnable item accumulates history across attempts.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct TargetId(pub String);

impl From<&str> for TargetId {
    fn from(value: &str) -> Self {
        Self(value.to_owned())
    }
}

impl fmt::Display for TargetId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multiple_attempts_contribute_to_the_same_target_history() {
        use crate::AttemptId;
        use std::collections::BTreeMap;

        let target = TargetId::from("tactic:fork-17");
        let mut history: BTreeMap<TargetId, Vec<AttemptId>> = BTreeMap::new();
        history
            .entry(target.clone())
            .or_default()
            .push(AttemptId::from("first-try"));
        history
            .entry(target.clone())
            .or_default()
            .push(AttemptId::from("clean-repeat"));

        assert_eq!(history.len(), 1);
        assert_eq!(history[&target].len(), 2);
    }
}
