use colored::*;
use std::fmt::{self, Display, Formatter};

pub enum Echo<'a> {
    AliasUse(&'a String),
    VersionUse(&'a String),
    AliasNotFound(&'a String),
    VersionNotFound(&'a String),
}

impl Display for Echo<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::AliasUse(v) => write!(f, "Using alias: {}", v.bold()),
            Self::VersionUse(v) => write!(f, "Using version: {}", v.to_string().bold()),
            Self::AliasNotFound(v) => write!(f, "Unable to find alias - {}", v.bold()),
            Self::VersionNotFound(v) => {
                write!(f, "Unable to find version - {}", v.to_string().bold())
            }
        }
    }
}
