use std::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
pub struct UserLts(String);

impl UserLts {
    const PREFIX_SLASH: &'static str = "lts/";
    pub const PREFIX: &'static str = "lts-";

    pub fn new(s: &str) -> UserLts {
        Self(s[4..].to_lowercase())
    }

    pub fn is_lts(s: &str) -> bool {
        s.starts_with(Self::PREFIX_SLASH) || s.starts_with(Self::PREFIX)
    }
}

impl Display for UserLts {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", Self::PREFIX, self.0)
    }
}

impl AsRef<str> for UserLts {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
