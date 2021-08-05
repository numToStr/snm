use std::{
    fmt::{self, Display, Formatter},
    str::FromStr,
};

/// For user provided lts codename string inputs
#[derive(Debug, PartialEq, Eq)]
pub struct UserLts(String);

impl UserLts {
    const PREFIX_SLASH: &'static str = "lts/";
    pub const PREFIX: &'static str = "lts-";

    pub fn new(s: &str) -> Self {
        Self(s[4..].to_lowercase())
    }

    pub fn is_lts(s: &str) -> bool {
        s.starts_with(Self::PREFIX_SLASH) || s.starts_with(Self::PREFIX)
    }
}

impl Display for UserLts {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", Self::PREFIX, self.0)
    }
}

impl AsRef<str> for UserLts {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

/// For user provided alias string inputs
#[derive(Debug, PartialEq, Eq)]
pub struct UserAlias(String);

impl UserAlias {
    pub fn new(s: &str) -> Self {
        Self(s.replace('/', "-").replace('\\', "-"))
    }
}

impl AsRef<str> for UserAlias {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Display for UserAlias {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl FromStr for UserAlias {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(s))
    }
}
