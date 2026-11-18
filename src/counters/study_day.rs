//! Which configured calendar day a due date or outcome belongs to.
//!
//! Use this for day-level scheduling and daily queue eligibility. The app computes
//! it using the user's configured day boundary and passes it inward; components do
//! not inspect the machine's clock or timezone. It is an ordered day index, not a
//! Unix timestamp. Consecutive days have consecutive values, including below zero.

use serde::{Deserialize, Serialize};

/// Which configured calendar day a due date or outcome belongs to.
#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
#[serde(transparent)]
pub struct StudyDay(pub i32);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn due_today_includes_overdue_items_but_excludes_tomorrow() {
        let today = StudyDay(50);
        let due_days = [StudyDay(49), today, StudyDay(51)];
        let eligible: Vec<_> = due_days.into_iter().filter(|due| *due <= today).collect();

        assert_eq!(eligible, vec![StudyDay(49), StudyDay(50)]);
        // Queue owns full eligibility. This is the day comparison it can use
        // without knowing UTC offsets or how the application chose day 50.
    }
}
