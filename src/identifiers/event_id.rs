//! Which external observation has already been processed.
//!
//! Use this when ingesting a game-derived mistake or another external outcome.
//! The app carries this identity to the learning-history write so repeated delivery
//! can be recognized. It is not a human attempt or answer command; do not invent
//! an attempt ID for a game event. The adapter must supply a stable ID.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Which external observation has already been processed.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct EventId(pub String);

impl From<&str> for EventId {
    fn from(value: &str) -> Self {
        Self(value.to_owned())
    }
}

impl fmt::Display for EventId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn repeated_delivery_of_one_game_observation_can_be_recognized() {
        use std::collections::BTreeSet;

        let game_miss = EventId::from("game-91:position-24:miss");
        let mut seen = BTreeSet::new();
        assert!(seen.insert(game_miss.clone()));
        assert!(!seen.insert(game_miss));
        assert!(seen.insert(EventId::from("game-92:position-24:miss")));
        assert_eq!(seen.len(), 2);
    }
}
