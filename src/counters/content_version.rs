//! Which semantic definition of a target an attempt was issued against.
//!
//! Use this alongside a target ID when grading or applying an external observation.
//! A changed answer/definition requires a new version; a metadata-only edit does not.
//! Chess import planning chooses versions. The app and Store reject a command whose
//! expected version no longer matches. This is separate from the response revision
//! that advances after each accepted answer.

use serde::{Deserialize, Serialize};

/// Which semantic definition of a target an attempt was issued against.
#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
#[serde(transparent)]
pub struct ContentVersion(pub u64);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_issued_exercise_can_select_its_exact_definition_after_an_update() {
        use crate::TargetId;
        use std::collections::BTreeMap;

        let target = TargetId::from("repertoire:position-8");
        let issued_version = ContentVersion(1);
        let current_version = ContentVersion(2);
        let definitions = BTreeMap::from([
            ((target.clone(), issued_version), "old response"),
            ((target.clone(), current_version), "revised response"),
        ]);

        assert_eq!(
            definitions[&(target.clone(), issued_version)],
            "old response"
        );
        assert_ne!(issued_version, current_version);
        // Loading the old definition is useful for history. The app must still
        // reject a new answer against it once version 2 becomes current.
        assert_eq!(definitions[&(target, current_version)], "revised response");
    }
}
