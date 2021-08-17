use snm_core::{
    fetcher::{Lts, Release},
    types::{UserAlias, UserLts},
    version::{DistVersion, ParseVersion, UserVersion},
};
use std::str::FromStr;

#[test]
fn major() {
    let s = "10";
    let ver = UserVersion::from_str(s).unwrap();

    assert_eq!(ver, UserVersion::Major(10));
}

#[test]
fn major_minor() {
    let s = "10.15";
    let ver = UserVersion::from_str(s).unwrap();

    assert_eq!(ver, UserVersion::MajorMinor(10, 15));
}

#[test]
fn full() {
    let s = "10.15.10";
    let ver = UserVersion::from_str(s).unwrap();

    assert_eq!(ver, UserVersion::Semver(DistVersion::parse(s).unwrap()));
}

#[test]
fn lts() {
    let s = "lts/yo";
    let v = UserVersion::from_str(s).unwrap();
    assert_eq!(v, UserVersion::Lts(UserLts::new(s)))
}

#[test]
fn alias() {
    let s = "i-am-alias";
    let v = UserVersion::from_str(s).unwrap();
    assert_eq!(v, UserVersion::Alias(UserAlias::new(s)))
}

#[test]
fn range() {
    let r = ">=14.13";
    let v = UserVersion::parse(r).unwrap();

    let rel_gt = Release {
        version: DistVersion::parse("14.19.10").unwrap(),
        lts: Lts::No,
    };

    let rel_lt = Release {
        version: DistVersion::parse("13.10.4").unwrap(),
        lts: Lts::No,
    };

    assert!(v.match_release(&rel_gt));
    assert!(!v.match_release(&rel_lt));
}

#[test]
fn match_release_semver() {
    let ver = UserVersion::parse("10").unwrap();
    let release = Release {
        lts: Lts::No,
        version: DistVersion::parse("10.12.13").unwrap(),
    };

    assert!(ver.match_release(&release));
}

#[test]
fn match_release_semver_fail() {
    let ver = UserVersion::parse("12").unwrap();
    let release = Release {
        version: DistVersion::parse("18.12.13").unwrap(),
        lts: Lts::No,
    };

    assert!(!ver.match_release(&release));
}

#[test]
fn match_release_lts() {
    let ver = UserVersion::parse("lts/boron").unwrap();
    let release = Release {
        version: DistVersion::parse("12.10.13").unwrap(),
        lts: Lts::Yes("boron".to_string()),
    };

    assert!(ver.match_release(&release));
}

#[test]
fn match_release_lts_fail() {
    let ver = UserVersion::parse("lts/boron").unwrap();
    let release = Release {
        version: DistVersion::parse("14.10.13").unwrap(),
        lts: Lts::Yes("carbon".to_string()),
    };

    assert!(!ver.match_release(&release));
}

#[test]
fn from_file_nvmrc() {
    // Check .nvmrc in project root
    let from_file = UserVersion::from_file().unwrap();

    let version = UserVersion::Major(14);

    let release = Release {
        version: DistVersion::parse("14.10.13").unwrap(),
        lts: Lts::No,
    };

    assert_eq!(from_file, version);
    assert!(from_file.match_release(&release));
}
