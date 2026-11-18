//! The shared outcome vocabulary understood by both Study and Learning.
//!
//! Reach for [`Grade`] when an attempt or external observation has an outcome
//! that Learning should process. Per-move correctness belongs in grader feedback;
//! a grade does not contain a due date, scheduler memory, or a retry instruction.

use serde::{Deserialize, Serialize};

/// A terminal learning outcome, independent of the exercise's subject.
///
/// Study can emit `Again` on the first miss while presentation continues. A later
/// correction does not replace that result. `Good` requires a completed, clean
/// attempt. Learning decides what either outcome means for scheduling.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Grade {
    /// The complete attempt met the grader's success criterion.
    Good,
    /// The attempt or an external observation recorded a failure.
    Again,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_consumer_can_describe_the_outcome_without_knowing_the_grader() {
        // A reporting adapter needs the outcome, not a chess move or scheduler.
        fn outcome_label(grade: Grade) -> &'static str {
            match grade {
                Grade::Good => "clean attempt",
                Grade::Again => "miss recorded",
            }
        }

        assert_eq!(outcome_label(Grade::Good), "clean attempt");
        assert_eq!(outcome_label(Grade::Again), "miss recorded");
    }
}
