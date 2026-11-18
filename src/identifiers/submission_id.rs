//! Which answer command a stored reply can be replayed for.
//!
//! Use this for transport idempotency. The caller creates one ID for one command
//! and reuses both the ID and the unchanged payload if its response is lost. A new
//! answer gets a new ID. The application and Store own receipt lookup and payload
//! comparison; the wrapper alone cannot enforce either rule.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Which answer command a stored reply can be replayed for.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SubmissionId(pub String);

impl From<&str> for SubmissionId {
    fn from(value: &str) -> Self {
        Self(value.to_owned())
    }
}

impl fmt::Display for SubmissionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_transport_retry_finds_the_original_reply_by_submission_id() {
        use std::collections::BTreeMap;

        let original = SubmissionId::from("answer-command-13");
        let retry = original.clone();
        let next_answer = SubmissionId::from("answer-command-14");
        let receipts = BTreeMap::from([(original, "reply for accepted answer")]);

        assert_eq!(receipts.get(&retry), Some(&"reply for accepted answer"));
        assert_eq!(receipts.get(&next_answer), None);
        // The real receipt also stores the command, so reusing an ID with a
        // different payload is rejected rather than treated as a retry.
    }
}
