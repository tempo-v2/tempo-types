use serde::{Deserialize, Serialize};
use std::fmt;

macro_rules! identifier {
    ($name:ident) => {
        #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
        #[serde(transparent)]
        pub struct $name(pub String);
        impl From<&str> for $name {
            fn from(value: &str) -> Self {
                Self(value.to_owned())
            }
        }
        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.0.fmt(f)
            }
        }
    };
}

identifier!(DatasetId);
identifier!(TargetId);
identifier!(AttemptId);
identifier!(SubmissionId);
identifier!(EventId);
