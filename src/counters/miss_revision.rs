//! Which generation of recorded misses a fresh attempt has observed.
//!
//! Use this in Learning state and in the app's metadata around an issued attempt.
//! Learning advances it for a new miss. A successful attempt can clear a repeat only
//! if it was issued against the current miss revision, so an older in-flight attempt
//! cannot erase a newer game-derived miss. It is not a per-move response revision.

use serde::{Deserialize, Serialize};

/// Which generation of recorded misses a fresh attempt has observed.
#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
#[serde(transparent)]
pub struct MissRevision(pub u64);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_success_started_before_a_later_miss_is_visibly_stale() {
        let issued_before_game = MissRevision(4);
        let after_game_miss = MissRevision(5);
        let issued_after_game = after_game_miss;

        assert_ne!(issued_before_game, after_game_miss);
        assert_eq!(issued_after_game, after_game_miss);
        // Learning owns the clearing rule. The wrapper gives the two markers
        // a meaning distinct from attempt revisions and content versions.
    }
}
