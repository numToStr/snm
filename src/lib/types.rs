use std::{
    convert::Infallible,
    fmt::{self, Display, Formatter},
    path::{Path, PathBuf},
    str::FromStr,
};

use console::style;

macro_rules! as_ref {
    ($impl: ident, $ty: ty) => {
        impl AsRef<$ty> for $impl {
            fn as_ref(&self) -> &$ty {
                &self.0
            }
        }
    };
}

macro_rules! neww {
    ($impl: ident) => {
        impl $impl {
            pub fn new(p: PathBuf) -> Self {
                Self(p)
            }
        }
    };
}

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

as_ref!(UserLts, str);

/// For user provided alias string inputs
#[derive(Debug, PartialEq, Eq)]
pub struct UserAlias(String);

impl UserAlias {
    pub const ACTIVE: &'static str = "active";

    pub fn new(s: &str) -> Self {
        Self(s.replace('/', "-").replace('\\', "-"))
    }
}

as_ref!(UserAlias, str);

impl Display for UserAlias {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl FromStr for UserAlias {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == Self::ACTIVE {
            anyhow::bail!("{} is not allowed", style(Self::ACTIVE).bold())
        }

        Ok(Self::new(s))
    }
}

/// For relase dir
pub struct ReleaseDir(PathBuf);

as_ref!(ReleaseDir, PathBuf);

neww!(ReleaseDir);

impl ReleaseDir {
    pub fn join<P: AsRef<Path>>(&self, p: P) -> Self {
        Self(self.0.join(p))
    }
}

/// For alias dir
pub struct AliasDir(PathBuf);

as_ref!(AliasDir, PathBuf);

neww!(AliasDir);

impl AliasDir {
    pub fn join<P: AsRef<Path>>(&self, p: P) -> Self {
        Self(self.0.join(p))
    }
}

/// For download dir
pub struct DownloadDir(PathBuf);

as_ref!(DownloadDir, PathBuf);

neww!(DownloadDir);

/// snm home directory
#[derive(Debug)]
pub struct SnmDir(PathBuf);

as_ref!(SnmDir, PathBuf);

impl Default for SnmDir {
    fn default() -> Self {
        Self(
            dirs_next::home_dir()
                .expect("Can't get home directory.")
                .join(".snm"),
        )
    }
}

impl Display for SnmDir {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.display())
    }
}

impl FromStr for SnmDir {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let path = PathBuf::from_str(s)?;
        Ok(Self(path))
    }
}
