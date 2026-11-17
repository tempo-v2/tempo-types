use serde::{Deserialize, Serialize};

use crate::counters::StudyDay;

/// The app supplies the configured Study day and acceptance timestamp.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct StudyTime {
    pub day: StudyDay,
    pub unix_ms: u64,
}
