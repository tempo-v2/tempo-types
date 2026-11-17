use serde::{Deserialize, Serialize};

macro_rules! counter {
    ($name:ident, $repr:ty) => {
        #[derive(
            Clone,
            Copy,
            Debug,
            Default,
            PartialEq,
            Eq,
            PartialOrd,
            Ord,
            Hash,
            Serialize,
            Deserialize,
        )]
        #[serde(transparent)]
        pub struct $name(pub $repr);
    };
}

counter!(ContentVersion, u64);
counter!(Revision, u64);
counter!(MissRevision, u64);
counter!(StudyDay, i32);
