use semver::Version;
use snm_core::version::{DistVersion, ParseVersion};

#[test]
fn parse() {
    let ver = DistVersion::parse("14.17.4").unwrap();

    assert_eq!(
        ver,
        DistVersion(Version {
            major: 14,
            minor: 17,
            patch: 4,
            pre: vec![],
            build: vec![]
        })
    );
}

#[test]
fn parse_fail() {
    let major = DistVersion::parse("10").ok();
    let major_minor = DistVersion::parse("12.10").ok();

    assert_eq!(major, None);
    assert_eq!(major_minor, None);
}
