# Changelog

## [0.7.0]

I finally rewrote the whole program to be more idiomatic. Removing any unnecessary code, making the codebase more readable and maintainable. Sadly, in doing so I've to introduce some breaking change. I've listed the major changes below:

PR: [#79](https://github.com/numToStr/snm/pull/79)

### Breaking

-   Previously, installation directories were all prefixed with the v character, now it is gone.

```shell
# before
$SNM_DIR/releases/v14.17.4

# now
$SNM_DIR/releases/14.17.4
```

-   From this version ownwards, MSRV will be `1.54.0`

### Changes

-   Previously, releases were downloaded into memory because of the progress bar implementation. Now they all are downloaded temporarily inside the `$SNM_DIR/downloads` and when the installation is finished, the downloads will be automatically deleted. Keep in mind, it will not delete automatically if the command is terminated by `CTRL-C` and friends.
-   `prune` command now also deletes `$SNM_DIR/downloads` if it is not empty.
-   Better progress bar implementation. Now the lag is gone, at least in the unix environment.
-   Removed spinner from `snm ls-remote` command
-   Replaced `colored` with `console` for terminal color output, as console was already used with `indicatif`.
-   Added concrete types for user alias, release/alias/download dir, etc.
-   Fixed some bugs that I found along the way.

### Misc

-   Bump `semver` to `v1` [#78](https://github.com/numToStr/snm/pull/78)
-   Fix clippy lint errors [74f16c2](https://github.com/numToStr/snm/commit/74f16c2fe631839a7013fb09fc4b150992650646)

## [v0.6.0](https://github.com/numtostr/snm/tree/v0.6.0) (2021-05-15)

[Full Changelog](https://github.com/numtostr/snm/compare/v0.5.2...v0.6.0)

**Merged pull requests:**

-   feat: added spinner in snm {install,lts,latest} [\#66](https://github.com/numToStr/snm/pull/66) ([numToStr](https://github.com/numToStr))
-   fix: prevent downloading of same version again [\#64](https://github.com/numToStr/snm/pull/64) ([numToStr](https://github.com/numToStr))
-   breaking: rename flag `--download-only` to `--no-use` [\#63](https://github.com/numToStr/snm/pull/63) ([numToStr](https://github.com/numToStr))
-   chore: refactor downloader [\#62](https://github.com/numToStr/snm/pull/62) ([numToStr](https://github.com/numToStr))
-   feat: added spinner in `snm ls-remote` [\#61](https://github.com/numToStr/snm/pull/61) ([numToStr](https://github.com/numToStr))

## [v0.5.2](https://github.com/numtostr/snm/tree/v0.5.2) (2021-05-09)

[Full Changelog](https://github.com/numtostr/snm/compare/v0.5.1...v0.5.2)

**Merged pull requests:**

-   fix: `--download-only` doc string [\#59](https://github.com/numToStr/snm/pull/59) ([numToStr](https://github.com/numToStr))
-   chore: fix clippy lint errors [\#58](https://github.com/numToStr/snm/pull/58) ([numToStr](https://github.com/numToStr))
-   chore: improved install message [\#57](https://github.com/numToStr/snm/pull/57) ([numToStr](https://github.com/numToStr))

## [v0.5.1](https://github.com/numtostr/snm/tree/v0.5.1) (2021-05-07)

[Full Changelog](https://github.com/numtostr/snm/compare/v0.5.0...v0.5.1)

**Merged pull requests:**

-   fix: revert `armv7` to `arm` target [\#56](https://github.com/numToStr/snm/pull/56) ([numToStr](https://github.com/numToStr))
-   chore: separate crates by targets [\#55](https://github.com/numToStr/snm/pull/55) ([numToStr](https://github.com/numToStr))
-   build\(deps\): bump url from 2.2.0 to 2.2.2 [\#54](https://github.com/numToStr/snm/pull/54) ([dependabot[bot]](https://github.com/apps/dependabot))
-   build\(deps\): bump anyhow from 1.0.38 to 1.0.40 [\#53](https://github.com/numToStr/snm/pull/53) ([dependabot[bot]](https://github.com/apps/dependabot))
-   build\(deps\): bump serde_json from 1.0.62 to 1.0.64 [\#52](https://github.com/numToStr/snm/pull/52) ([dependabot[bot]](https://github.com/apps/dependabot))
-   build\(deps\): bump zip from 0.5.9 to 0.5.12 [\#51](https://github.com/numToStr/snm/pull/51) ([dependabot[bot]](https://github.com/apps/dependabot))
-   build\(deps\): bump tar from 0.4.32 to 0.4.33 [\#50](https://github.com/numToStr/snm/pull/50) ([dependabot[bot]](https://github.com/apps/dependabot))
-   build\(deps\): bump ureq from 2.0.1 to 2.1.1 [\#48](https://github.com/numToStr/snm/pull/48) ([dependabot[bot]](https://github.com/apps/dependabot))
-   build\(deps\): bump serde from 1.0.123 to 1.0.125 [\#47](https://github.com/numToStr/snm/pull/47) ([dependabot[bot]](https://github.com/apps/dependabot))

## [v0.5.0](https://github.com/numtostr/snm/tree/v0.5.0) (2021-05-07)

[Full Changelog](https://github.com/numtostr/snm/compare/v0.4.1...v0.5.0)

**Merged pull requests:**

-   feat: download progress bar [\#46](https://github.com/numToStr/snm/pull/46) ([numToStr](https://github.com/numToStr))
-   fix typo in downloader [\#45](https://github.com/numToStr/snm/pull/45) ([numToStr](https://github.com/numToStr))

## [v0.4.1](https://github.com/numtostr/snm/tree/v0.4.1) (2021-02-11)

[Full Changelog](https://github.com/numtostr/snm/compare/v0.3.0...v0.4.1)

**Closed issues:**

-   snm exec refuse more than 1 argument [\#43](https://github.com/numToStr/snm/issues/43)

**Merged pull requests:**

-   chore: package updates [\#44](https://github.com/numToStr/snm/pull/44) ([numToStr](https://github.com/numToStr))
-   fix shell examples in readme [\#42](https://github.com/numToStr/snm/pull/42) ([ScottLNorvell](https://github.com/ScottLNorvell))

## [v0.3.0](https://github.com/numtostr/snm/tree/v0.3.0) (2021-01-13)

[Full Changelog](https://github.com/numtostr/snm/compare/v0.2.0...v0.3.0)

**Closed issues:**

-   Support `engines` key from `package.json` [\#38](https://github.com/numToStr/snm/issues/38)

**Merged pull requests:**

-   Support `package.json` in `snm use` [\#40](https://github.com/numToStr/snm/pull/40) ([numToStr](https://github.com/numToStr))
-   Improved perf of `snm use` when reading `.nvmrc` or `.node-version` [\#39](https://github.com/numToStr/snm/pull/39) ([numToStr](https://github.com/numToStr))

## [v0.2.0](https://github.com/numtostr/snm/tree/v0.2.0) (2021-01-05)

[Full Changelog](https://github.com/numtostr/snm/compare/v0.1.9...v0.2.0)

**Closed issues:**

-   Add `tj/n` style version help in `snm --help` [\#37](https://github.com/numToStr/snm/issues/37)
-   Add `a` as an alias to `alias` command [\#36](https://github.com/numToStr/snm/issues/36)

**Merged pull requests:**

-   improved help and added missing doc comments [\#35](https://github.com/numToStr/snm/pull/35) ([numToStr](https://github.com/numToStr))
-   Upgrade ureq to v2 [\#34](https://github.com/numToStr/snm/pull/34) ([numToStr](https://github.com/numToStr))
-   Added bash install script for Linux and macOS [\#33](https://github.com/numToStr/snm/pull/33) ([numToStr](https://github.com/numToStr))

## [v0.1.9](https://github.com/numtostr/snm/tree/v0.1.9) (2021-01-03)

[Full Changelog](https://github.com/numtostr/snm/compare/v0.1.8...v0.1.9)

**Merged pull requests:**

-   pull_request workflow file [\#30](https://github.com/numToStr/snm/pull/30) ([numToStr](https://github.com/numToStr))
-   Revert "Added workflow on pull request" [\#29](https://github.com/numToStr/snm/pull/29) ([numToStr](https://github.com/numToStr))
-   fixed windows binary not working [\#28](https://github.com/numToStr/snm/pull/28) ([numToStr](https://github.com/numToStr))
-   Added workflow on pull request [\#27](https://github.com/numToStr/snm/pull/27) ([numToStr](https://github.com/numToStr))

## [v0.1.8](https://github.com/numtostr/snm/tree/v0.1.8) (2021-01-02)

[Full Changelog](https://github.com/numtostr/snm/compare/e3cd4480038682a828a16a0e48f5f5bafe1b1684...v0.1.8)

**Closed issues:**

-   Release 0.1.0 [\#25](https://github.com/numToStr/snm/issues/25)
-   lts/\* pattern in `snm unalias` [\#16](https://github.com/numToStr/snm/issues/16)
-   Allow alias and lts\* in `snm uninstall` [\#15](https://github.com/numToStr/snm/issues/15)
-   Add README [\#14](https://github.com/numToStr/snm/issues/14)
-   snm unalias [\#13](https://github.com/numToStr/snm/issues/13)
-   Make some defaults aliases [\#12](https://github.com/numToStr/snm/issues/12)
-   Allow aliases in `snm use` [\#10](https://github.com/numToStr/snm/issues/10)
-   Support .nvmrc and .node-version [\#9](https://github.com/numToStr/snm/issues/9)
-   Allow lts-\* and lts/\* installations [\#8](https://github.com/numToStr/snm/issues/8)
-   Bubble up the errors [\#6](https://github.com/numToStr/snm/issues/6)
-   GHA for release [\#5](https://github.com/numToStr/snm/issues/5)
-   Windows support? [\#4](https://github.com/numToStr/snm/issues/4)
-   Colorful output [\#2](https://github.com/numToStr/snm/issues/2)
-   Tests [\#1](https://github.com/numToStr/snm/issues/1)

**Merged pull requests:**

-   Added mod tests [\#26](https://github.com/numToStr/snm/pull/26) ([numToStr](https://github.com/numToStr))
-   Towards 0.1.0 release [\#24](https://github.com/numToStr/snm/pull/24) ([numToStr](https://github.com/numToStr))
-   Windows support [\#11](https://github.com/numToStr/snm/pull/11) ([numToStr](https://github.com/numToStr))
-   Error bubbles [\#7](https://github.com/numToStr/snm/pull/7) ([numToStr](https://github.com/numToStr))

\* _This Changelog was automatically generated by [github_changelog_generator](https://github.com/github-changelog-generator/github-changelog-generator)_
