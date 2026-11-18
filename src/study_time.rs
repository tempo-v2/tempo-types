//! Application-supplied time for one accepted operation.
//!
//! Use [`StudyTime`] when an outcome needs both a day for scheduling and an
//! absolute timestamp for history. The application chooses both once and passes
//! them into the domain; tests can supply deterministic time without a fake clock.

use serde::{Deserialize, Serialize};

use crate::StudyDay;

/// A configured Study day paired with the acceptance timestamp.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct StudyTime {
    /// The app's configured day, used for scheduling and daily policy.
    pub day: StudyDay,
    /// Milliseconds since the Unix epoch, used to timestamp the accepted event.
    pub unix_ms: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_outcomes_on_one_study_day_keep_their_distinct_timestamps() {
        let morning = StudyTime {
            day: StudyDay(20),
            unix_ms: 1_728_000_000,
        };
        let evening = StudyTime {
            day: StudyDay(20),
            unix_ms: 1_771_200_000,
        };

        // Scheduling can compare the configured days, while history preserves
        // the order within that day. Neither consumer needs to read a clock.
        assert_eq!(morning.day, evening.day);
        assert!(morning.unix_ms < evening.unix_ms);
    }
}
