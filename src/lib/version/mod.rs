mod dist_version;
pub use dist_version::*;

mod user_version;
pub use user_version::*;

use super::SnmRes;

pub trait ParseVersion {
    type Item;
    fn parse(v: &str) -> SnmRes<Self::Item>;
}
