//! Which single run through an exercise a response belongs to.
//!
//! Use this to resume, answer, abandon, or inspect one issued attempt. The app
//! allocates an ID when Study starts an exercise. Every answer within that run keeps
//! the same ID; a fresh repeat gets another ID even for the same target. A separate
//! submission ID identifies each answer command and its transport retries.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Which single run through an exercise a response belongs to.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct AttemptId(pub String);

impl From<&str> for AttemptId {
    fn from(value: &str) -> Self {
        Self(value.to_owned())
    }
}

impl fmt::Display for AttemptId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn several_answer_commands_belong_to_one_attempt_but_a_repeat_is_another() {
        use crate::SubmissionId;
        use std::collections::BTreeMap;

        let attempt = AttemptId::from("tactic-run-1");
        let repeat = AttemptId::from("tactic-run-2");
        let mut commands: BTreeMap<AttemptId, Vec<SubmissionId>> = BTreeMap::new();
        commands
            .entry(attempt.clone())
            .or_default()
            .extend([SubmissionId::from("move-1"), SubmissionId::from("move-2")]);
        commands
            .entry(repeat.clone())
            .or_default()
            .push(SubmissionId::from("repeat-move-1"));

        assert_eq!(commands[&attempt].len(), 2);
        assert_eq!(commands[&repeat].len(), 1);
    }
}
