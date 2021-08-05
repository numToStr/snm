mod dist_version;
pub use dist_version::*;

mod user_version;
pub use user_version::*;

use super::SnmRes;

pub(super) trait ParseVersion<'a> {
    type Item;
    fn parse(v: &'a str) -> SnmRes<Self::Item>;
}
