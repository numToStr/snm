use super::SnmRes;

pub mod dist_version;
pub mod user_version;

pub(super) trait ParseVersion<'a> {
    type Item;
    fn parse(v: &'a str) -> SnmRes<Self::Item>;
}
