//! Which response state of an attempt or snapshot a command expects.
//!
//! Use this for optimistic concurrency: a command echoes the version of state it
//! read, and the owner accepts it only if that state is still current. Study advances
//! an attempt revision after an accepted answer or abandonment; Store also uses
//! Revision for snapshot commits. It is not a content version or miss revision.
//! Increments and overflow checks belong to the owning operation.

use serde::{Deserialize, Serialize};

/// Which response state of an attempt or snapshot a command expects.
#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
#[serde(transparent)]
pub struct Revision(pub u64);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_commands_from_the_same_view_cannot_both_match_the_new_revision() {
        let view_revision = Revision(3);
        let second_command_expected = view_revision;
        let after_first_command = Revision(view_revision.0.checked_add(1).unwrap());

        assert_eq!(after_first_command, Revision(4));
        assert_ne!(second_command_expected, after_first_command);
        // This comparison is what Study/Store use to reject a stale command;
        // changing its submission ID does not make its expected revision fresh.
    }
}
