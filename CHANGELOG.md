<!-- markdownlint-disable MD024 -->
# Changelog

## [v0.1.50](https://github.com/MAHDTech/agent-skills/compare/v0.1.49...ff4647076b3bcf582898df696579739d9225cace) (2026-10-11)

### Features

* **skills:** add cargo-crap skill for CRAP metric analysis and quality gates
(#210)
([7deb8de](https://github.com/MAHDTech/agent-skills/commit/7deb8deaf0afbdadc516d1a85f9f019f561054db)),
closes [#210](https://github.com/MAHDTech/agent-skills/issues/210)

### Fixes

* Exclude CHANGELOG.md (#216)
([998d519](https://github.com/MAHDTech/agent-skills/commit/998d519f3429b9801969fee2e1e52dc824b91e72)),
closes [#216](https://github.com/MAHDTech/agent-skills/issues/216)
* **tars-run-factory:** align skill with native supervisor protocol and
cross-harness execution (#213)
([d1250c4](https://github.com/MAHDTech/agent-skills/commit/d1250c437da864f1b6f9233036aaa64890969d5a)),
closes [#213](https://github.com/MAHDTech/agent-skills/issues/213)
* **errors:** introduce dedicated sync/target error variants and eliminate
installer validation erasure (#155) (#206)
([70d6046](https://github.com/MAHDTech/agent-skills/commit/70d6046288e0ebcfddd9e5928977c5b6e5af50a8)),
closes [#155](https://github.com/MAHDTech/agent-skills/issues/155)
[#206](https://github.com/MAHDTech/agent-skills/issues/206)
[#155](https://github.com/MAHDTech/agent-skills/issues/155)
[#155](https://github.com/MAHDTech/agent-skills/issues/155)
[#155](https://github.com/MAHDTech/agent-skills/issues/155)
[#155](https://github.com/MAHDTech/agent-skills/issues/155)
* **clippy:** resolve workspace clippy warnings under -D warnings (#154)
(#205)
([efc17ab](https://github.com/MAHDTech/agent-skills/commit/efc17ab4080c17a28c1b4859c52a7a10b1e330f9)),
closes [#154](https://github.com/MAHDTech/agent-skills/issues/154)
[#205](https://github.com/MAHDTech/agent-skills/issues/205)

### [v0.1.49](https://github.com/MAHDTech/agent-skills/compare/v0.1.48...v0.1.49) (2026-10-02)

#### Features

* **skills:** add egui, bevy-egui, and cargo-doc skills (#209)
([c270472](https://github.com/MAHDTech/agent-skills/commit/c270472fe7531ff06ece49a52ccd01141bab7f1f)),
closes [#209](https://github.com/MAHDTech/agent-skills/issues/209)

### [v0.1.48](https://github.com/MAHDTech/agent-skills/compare/v0.1.47...v0.1.48) (2026-09-22)

#### Fixes

*  ask cli (#208)
([be9d4da](https://github.com/MAHDTech/agent-skills/commit/be9d4dabb810c8bd540939e209219a873aeb6033)),
closes [#208](https://github.com/MAHDTech/agent-skills/issues/208)

### [v0.1.47](https://github.com/MAHDTech/agent-skills/compare/v0.1.46...v0.1.47) (2026-09-20)

#### Fixes

* **skills-core:** prevent dropping root boundary under relative parents
during path normalization (#203)
([36ed062](https://github.com/MAHDTech/agent-skills/commit/36ed06201b827becf86e583eacff7891b3e9f1ea)),
closes [#203](https://github.com/MAHDTech/agent-skills/issues/203)

### [v0.1.46](https://github.com/MAHDTech/agent-skills/compare/v0.1.45...v0.1.46) (2026-09-19)

#### Fixes

* **artifacts:** add -- option separator to git add in stage_files (#152)
(#202)
([1acf753](https://github.com/MAHDTech/agent-skills/commit/1acf753a8a99a3aa3240268d64066be504d4d47a)),
closes [#152](https://github.com/MAHDTech/agent-skills/issues/152)
[#202](https://github.com/MAHDTech/agent-skills/issues/202)

### [v0.1.45](https://github.com/MAHDTech/agent-skills/compare/v0.1.44...v0.1.45) (2026-09-19)

#### Fixes

* **tui:** prevent terminal state leak with RAII drop guard (#151) (#201)
([aebec42](https://github.com/MAHDTech/agent-skills/commit/aebec421ded6aeb13e15fd62dccadfe8af6d76a2)),
closes [#151](https://github.com/MAHDTech/agent-skills/issues/151)
[#201](https://github.com/MAHDTech/agent-skills/issues/201)

### [v0.1.44](https://github.com/MAHDTech/agent-skills/compare/v0.1.43...v0.1.44) (2026-09-19)

#### Fixes

* **security:** include primary dependency lockfiles in trivy scans (#150)
(#200)
([a9e32b0](https://github.com/MAHDTech/agent-skills/commit/a9e32b0e2e6a2acb2fcb090f0a7dbc5cea756cc5)),
closes [#150](https://github.com/MAHDTech/agent-skills/issues/150)
[#200](https://github.com/MAHDTech/agent-skills/issues/200)

### [v0.1.43](https://github.com/MAHDTech/agent-skills/compare/v0.1.42...v0.1.43) (2026-09-19)

#### Fixes

* **actions:** eliminate script injection in report-status action (#149)
(#199)
([c13a60e](https://github.com/MAHDTech/agent-skills/commit/c13a60eeccea32c92ebcb675ea8277184a85b5ff)),
closes [#149](https://github.com/MAHDTech/agent-skills/issues/149)
[#199](https://github.com/MAHDTech/agent-skills/issues/199)

### [v0.1.42](https://github.com/MAHDTech/agent-skills/compare/v0.1.41...v0.1.42) (2026-09-19)

#### Fixes

* **dashboard:** escape shortcode body in diagram.html (#148) (#198)
([e713f23](https://github.com/MAHDTech/agent-skills/commit/e713f2319a957b2229da62da1b47c5a4e9ef128a)),
closes [#148](https://github.com/MAHDTech/agent-skills/issues/148)
[#198](https://github.com/MAHDTech/agent-skills/issues/198)

### [v0.1.41](https://github.com/MAHDTech/agent-skills/compare/v0.1.40...v0.1.41) (2026-09-19)

#### Fixes

* **installer:** enforce symlink boundary validation in copy_dir_all (#197)
([c1a5204](https://github.com/MAHDTech/agent-skills/commit/c1a5204042befe7e4ffdf7a5af1883516a8b0d95)),
closes [#197](https://github.com/MAHDTech/agent-skills/issues/197)

### [v0.1.40](https://github.com/MAHDTech/agent-skills/compare/v0.1.39...v0.1.40) (2026-09-19)

#### Fixes

* **installer:** implement atomic directory replacement and symlink-aware
rollback in AtomicSwapCoordinator (#146) (#196)
([cf76779](https://github.com/MAHDTech/agent-skills/commit/cf76779ece9bf7ff67cab4f95ee95cdd032ae046)),
closes [#146](https://github.com/MAHDTech/agent-skills/issues/146)
[#196](https://github.com/MAHDTech/agent-skills/issues/196)

### [v0.1.39](https://github.com/MAHDTech/agent-skills/compare/v0.1.38...v0.1.39) (2026-09-17)

#### Features

* **ask-cli:** implement dashboard site build, serve, css, and lint
subcommands (#135)
([71dc33b](https://github.com/MAHDTech/agent-skills/commit/71dc33bb053f45b17335d6f7b2baf2586c819e94)),
closes [#135](https://github.com/MAHDTech/agent-skills/issues/135)

### [v0.1.38](https://github.com/MAHDTech/agent-skills/compare/v0.1.37...v0.1.38) (2026-09-15)

#### Features

* Add NAI skill (#136)
([ca81e1d](https://github.com/MAHDTech/agent-skills/commit/ca81e1d1bf15e6ebf4d0799ca6e57181ca7d8e89)),
closes [#136](https://github.com/MAHDTech/agent-skills/issues/136)

### [v0.1.37](https://github.com/MAHDTech/agent-skills/compare/v0.1.36...v0.1.37) (2026-09-15)

#### Fixes

* gitignore TARS
([9eea3bd](https://github.com/MAHDTech/agent-skills/commit/9eea3bd95c578edba594697accbe4ccf3e500c49))

### [v0.1.36](https://github.com/MAHDTech/agent-skills/compare/v0.1.35...v0.1.36) (2026-09-14)

#### Features

* **ask-cli:** implement skills management, dashboard, and tui launch
subcommands (#127)
([1b582fe](https://github.com/MAHDTech/agent-skills/commit/1b582fe029a2c442413ce9bbc8b12d0b480fa763)),
closes [#127](https://github.com/MAHDTech/agent-skills/issues/127)
[#86](https://github.com/MAHDTech/agent-skills/issues/86)
[#86](https://github.com/MAHDTech/agent-skills/issues/86)

### [v0.1.35](https://github.com/MAHDTech/agent-skills/compare/v0.1.34...v0.1.35) (2026-09-14)

#### Features

* **ask-cli:** scaffold CLI crate, Clap command-line parser, and global
options (closes #85) (#126)
([2e4a473](https://github.com/MAHDTech/agent-skills/commit/2e4a473484aba18e50841319af689832f3e52588)),
closes [#85](https://github.com/MAHDTech/agent-skills/issues/85)
[#126](https://github.com/MAHDTech/agent-skills/issues/126)
* **skills-tui:** implement interactive linter and execution runner views
(closes #84) (#125)
([6054d54](https://github.com/MAHDTech/agent-skills/commit/6054d54d7a75f73bb6972757ce7c760aa9ba7fbd)),
closes [#84](https://github.com/MAHDTech/agent-skills/issues/84)
[#125](https://github.com/MAHDTech/agent-skills/issues/125)

### [v0.1.34](https://github.com/MAHDTech/agent-skills/compare/v0.1.33...v0.1.34) (2026-09-13)

#### Features

* **skills-core:** implement generated-artifact sync with archive section
(closes #101) (#123)
([00ca0bc](https://github.com/MAHDTech/agent-skills/commit/00ca0bcccbce68398c39c54bbbf39df9d16c7ab4)),
closes [#101](https://github.com/MAHDTech/agent-skills/issues/101)
[#123](https://github.com/MAHDTech/agent-skills/issues/123)

### [v0.1.33](https://github.com/MAHDTech/agent-skills/compare/v0.1.32...v0.1.33) (2026-09-13)

#### Features

* **skills-tui:** implement interactive skill explorer and inspector views
(#122)
([72bbb70](https://github.com/MAHDTech/agent-skills/commit/72bbb70cab4e01ab921101f07a8cc2f55dafa0ab)),
closes [#122](https://github.com/MAHDTech/agent-skills/issues/122)

### [v0.1.32](https://github.com/MAHDTech/agent-skills/compare/v0.1.31...v0.1.32) (2026-09-13)

#### Features

* **skills-core:** model skills-archive tree, archived metadata lint, and
grouped cross-reference policy (closes #100) (#121)
([434f2f6](https://github.com/MAHDTech/agent-skills/commit/434f2f6be49acd46c4755258dea6f85884c9813a)),
closes [#100](https://github.com/MAHDTech/agent-skills/issues/100)
[#121](https://github.com/MAHDTech/agent-skills/issues/121)

### [v0.1.31](https://github.com/MAHDTech/agent-skills/compare/v0.1.30...v0.1.31) (2026-09-13)

#### Features

* **skills-tui:** scaffold TUI crate, event loop, terminal lifecycle, and
state manager (#115)
([51cb89a](https://github.com/MAHDTech/agent-skills/commit/51cb89a9ac7e04fd0922a88d788d2963d357f4ab)),
closes [#115](https://github.com/MAHDTech/agent-skills/issues/115)
[#82](https://github.com/MAHDTech/agent-skills/issues/82)

### [v0.1.30](https://github.com/MAHDTech/agent-skills/compare/v0.1.29...v0.1.30) (2026-09-12)

#### Features

* **skills-core:** implement telemetry, skill registry metrics, and dashboard
aggregation engine (closes #81) (#114)
([b5132d4](https://github.com/MAHDTech/agent-skills/commit/b5132d4a3bc20ec82ed6b965136fae6f27ea15eb)),
closes [#81](https://github.com/MAHDTech/agent-skills/issues/81)
[#114](https://github.com/MAHDTech/agent-skills/issues/114)

### [v0.1.29](https://github.com/MAHDTech/agent-skills/compare/v0.1.28...v0.1.29) (2026-09-11)

#### Features

* **nix-shell:** add ephemeral nix-shell skill for on-demand CLI tools (#113)
([17b4173](https://github.com/MAHDTech/agent-skills/commit/17b417364ca5093af8ad055a8bf659818a9f913f)),
closes [#113](https://github.com/MAHDTech/agent-skills/issues/113)

### [v0.1.28](https://github.com/MAHDTech/agent-skills/compare/v0.1.27...v0.1.28) (2026-09-11)

#### Fixes

* **tars-run-factory:** calibrate pre-flight token check to use tars-agy
doctor (#112)
([e583795](https://github.com/MAHDTech/agent-skills/commit/e583795f998f44d9a1d60439af8b0b46308b80f8)),
closes [#112](https://github.com/MAHDTech/agent-skills/issues/112)

### [v0.1.27](https://github.com/MAHDTech/agent-skills/compare/v0.1.26...v0.1.27) (2026-09-11)

#### Features

* **factory:** migrate supervisor to native rust factory (#111)
([bb22760](https://github.com/MAHDTech/agent-skills/commit/bb22760b0d15aa66208596b181afad0d01a62f9c)),
closes [#111](https://github.com/MAHDTech/agent-skills/issues/111)

### [v0.1.26](https://github.com/MAHDTech/agent-skills/compare/v0.1.25...v0.1.26) (2026-09-10)

#### Fixes

* **factory:** stop on explicit content-filter errors (#110)
([c944667](https://github.com/MAHDTech/agent-skills/commit/c9446678f40f6ad2b77aff50826410bc3e72afa6)),
closes [#110](https://github.com/MAHDTech/agent-skills/issues/110)

### [v0.1.25](https://github.com/MAHDTech/agent-skills/compare/v0.1.24...v0.1.25) (2026-09-10)

#### Features

* Add new MCP skill (#109)
([6a7c637](https://github.com/MAHDTech/agent-skills/commit/6a7c63791e178201bd6e3911357713544c879be3)),
closes [#109](https://github.com/MAHDTech/agent-skills/issues/109)

### [v0.1.24](https://github.com/MAHDTech/agent-skills/compare/v0.1.23...v0.1.24) (2026-09-09)

#### Features

* **skills-core:** implement multi-target skill synchronization engine (closes
#80) (#99)
([b11e993](https://github.com/MAHDTech/agent-skills/commit/b11e9939da89c7e8c174541a5f8ad12d9bbee403)),
closes [#80](https://github.com/MAHDTech/agent-skills/issues/80)
[#99](https://github.com/MAHDTech/agent-skills/issues/99)

### [v0.1.23](https://github.com/MAHDTech/agent-skills/compare/v0.1.22...v0.1.23) (2026-09-09)

#### Fixes

* **tars-run-factory:** reject partial timeout success (#107)
([9ab2b3f](https://github.com/MAHDTech/agent-skills/commit/9ab2b3f6fc3811c36cf9968e6cec957291d038ce)),
closes [#107](https://github.com/MAHDTech/agent-skills/issues/107)

### [v0.1.22](https://github.com/MAHDTech/agent-skills/compare/v0.1.21...v0.1.22) (2026-09-08)

#### Fixes

* **tars-run-factory:** allow bounded host shutdown cleanup (#106)
([1f114df](https://github.com/MAHDTech/agent-skills/commit/1f114df5ffb6093a5eb3f408e3943863eb6346b9)),
closes [#106](https://github.com/MAHDTech/agent-skills/issues/106)

### [v0.1.21](https://github.com/MAHDTech/agent-skills/compare/v0.1.20...v0.1.21) (2026-09-08)

#### Features

* **tars-run-factory:** supervise stream turns and durable recovery (#105)
([b6cd80e](https://github.com/MAHDTech/agent-skills/commit/b6cd80e9696a2ae3e5765f4ab10a1c3d5108b933)),
closes [#105](https://github.com/MAHDTech/agent-skills/issues/105)

### [v0.1.20](https://github.com/MAHDTech/agent-skills/compare/v0.1.19...v0.1.20) (2026-09-08)

#### Fixes

* **tars-run-factory:** recover incidental refusals within existing
permissions
([ce83161](https://github.com/MAHDTech/agent-skills/commit/ce831619338438ea1382e0b5296a37460c5ec740))

### [v0.1.19](https://github.com/MAHDTech/agent-skills/compare/v0.1.18...v0.1.19) (2026-09-08)

#### Fixes

* factory skill
([d13453e](https://github.com/MAHDTech/agent-skills/commit/d13453eff0cbc9a8eae962f216211ee349d3d66a))
* factory skill
([5d22455](https://github.com/MAHDTech/agent-skills/commit/5d22455807b3a4f6ebf8ceff15696a636549ef6b))

### [v0.1.18](https://github.com/MAHDTech/agent-skills/compare/v0.1.17...v0.1.18) (2026-09-07)

#### Features

* tars-goal skill (#104)
([c222737](https://github.com/MAHDTech/agent-skills/commit/c222737cdcd6f286b7475960762563a5d8789c25)),
closes [#104](https://github.com/MAHDTech/agent-skills/issues/104)

### [v0.1.17](https://github.com/MAHDTech/agent-skills/compare/v0.1.16...v0.1.17) (2026-09-05)

#### Features

* tars run factory
([126c886](https://github.com/MAHDTech/agent-skills/commit/126c88620d3bc32c3c64dd3443d25bfe1c074b0f))
* tars run factory
([cb57d00](https://github.com/MAHDTech/agent-skills/commit/cb57d0068fdf9fd92f4d60b3f4be2c200bc8758b))

### [v0.1.16](https://github.com/MAHDTech/agent-skills/compare/v0.1.15...v0.1.16) (2026-09-04)

#### Features

* **skills:** add skills-archive tree and archive the tars-backlog skills
(#103)
([088043b](https://github.com/MAHDTech/agent-skills/commit/088043b32c40f6a615c7b68b837fed864084b3da)),
closes [#103](https://github.com/MAHDTech/agent-skills/issues/103)
[#100](https://github.com/MAHDTech/agent-skills/issues/100)
[#101](https://github.com/MAHDTech/agent-skills/issues/101)
[#102](https://github.com/MAHDTech/agent-skills/issues/102)

### [v0.1.15](https://github.com/MAHDTech/agent-skills/compare/v0.1.14...v0.1.15) (2026-09-04)

#### Features

* **tooling:** add tars-run-factory skill
([6cd3872](https://github.com/MAHDTech/agent-skills/commit/6cd3872b18aedd36aadb3541f6d26956a1497779))

### [v0.1.14](https://github.com/MAHDTech/agent-skills/compare/v0.1.13...v0.1.14) (2026-09-04)

#### Features

* **skills-core:** implement skill installer and environment bridge (#98)
([bc83f31](https://github.com/MAHDTech/agent-skills/commit/bc83f3122948b0f8c8a01695a51b45faef8703f0)),
closes [#98](https://github.com/MAHDTech/agent-skills/issues/98)
[#79](https://github.com/MAHDTech/agent-skills/issues/79)

### [v0.1.13](https://github.com/MAHDTech/agent-skills/compare/v0.1.12...v0.1.13) (2026-09-02)

#### Features

* **skills-core:** implement remote skill downloader and registry client (#97)
([25672ff](https://github.com/MAHDTech/agent-skills/commit/25672ff3dc3a0e57f29d895d4e39a5a012651e86)),
closes [#97](https://github.com/MAHDTech/agent-skills/issues/97)

### [v0.1.12](https://github.com/MAHDTech/agent-skills/compare/v0.1.11...v0.1.12) (2026-09-02)

#### Features

* **skills-core:** implement skill linting and static analysis engine (#96)
([90e6096](https://github.com/MAHDTech/agent-skills/commit/90e6096ab78563c0b15dbd06fb3cdc831344a430)),
closes [#96](https://github.com/MAHDTech/agent-skills/issues/96)

### [v0.1.11](https://github.com/MAHDTech/agent-skills/compare/v0.1.10...v0.1.11) (2026-09-02)

#### Features

* **skills-core:** implement skill markdown and frontmatter parser engine
(#95)
([c2801db](https://github.com/MAHDTech/agent-skills/commit/c2801db43964471ce21a959f535b8e0af968e761)),
closes [#95](https://github.com/MAHDTech/agent-skills/issues/95)

### [v0.1.10](https://github.com/MAHDTech/agent-skills/compare/v0.1.9...v0.1.10) (2026-09-02)

#### Features

* **skills-core:** scaffold crate structure, custom error types, and core
domain models (#94)
([5a5d1d3](https://github.com/MAHDTech/agent-skills/commit/5a5d1d35463cd30bbbaa72fc115ae8e8d6479a37)),
closes [#94](https://github.com/MAHDTech/agent-skills/issues/94)

### [v0.1.9](https://github.com/MAHDTech/agent-skills/compare/v0.1.8...v0.1.9) (2026-09-02)

#### Features

* Add Skill for Herdr (#70)
([4433980](https://github.com/MAHDTech/agent-skills/commit/443398076084bbd58e2da20e9a59c84ca0399c5b)),
closes [#70](https://github.com/MAHDTech/agent-skills/issues/70)

### [v0.1.8](https://github.com/MAHDTech/agent-skills/compare/v0.1.7...v0.1.8) (2026-08-22)

#### Fixes

* dashboard now builds (#63)
([b5fc7d5](https://github.com/MAHDTech/agent-skills/commit/b5fc7d5ffd877c96d7adb7238dafa450d78484c3)),
closes [#63](https://github.com/MAHDTech/agent-skills/issues/63)

### [v0.1.7](https://github.com/MAHDTech/agent-skills/compare/v0.1.6...v0.1.7) (2026-08-22)

#### Features

* skill improvements (#61)
([cc6f6ff](https://github.com/MAHDTech/agent-skills/commit/cc6f6ffb185a83d6819cad146cbc30f65e32cc3f)),
closes [#61](https://github.com/MAHDTech/agent-skills/issues/61)

### [v0.1.6](https://github.com/MAHDTech/agent-skills/compare/v0.1.5...v0.1.6) (2026-08-21)

#### Fixes

* **tars-backlog:** harden ticket format, repair duty and gate safety (#60)
([e37113d](https://github.com/MAHDTech/agent-skills/commit/e37113d01b215a9fcd5f15df0100cabd219fc7cb)),
closes [#60](https://github.com/MAHDTech/agent-skills/issues/60)

### [v0.1.5](https://github.com/MAHDTech/agent-skills/compare/v0.1.4...v0.1.5) (2026-08-12)

#### Fixes

* **tars-backlog-implement:** stop spokes writing to the parent repository
(#55)
([b59632a](https://github.com/MAHDTech/agent-skills/commit/b59632a1bec85d7ad7d766b6d2b1a5bdb3f620db)),
closes [#55](https://github.com/MAHDTech/agent-skills/issues/55)

### [v0.1.4](https://github.com/MAHDTech/agent-skills/compare/v0.1.3...v0.1.4) (2026-08-08)

#### Features

* New Skills (#54)
([26f20a8](https://github.com/MAHDTech/agent-skills/commit/26f20a87c910f2c312896ffe88a14ebd16483ac1)),
closes [#54](https://github.com/MAHDTech/agent-skills/issues/54)
* worktrees vs clones (#50)
([3188ca4](https://github.com/MAHDTech/agent-skills/commit/3188ca48bee398f215991c8ab76c5ad7117e399f)),
closes [#50](https://github.com/MAHDTech/agent-skills/issues/50)

### [v0.1.3](https://github.com/MAHDTech/agent-skills/compare/v0.1.2...v0.1.3) (2026-07-25)

#### Fixes

* Additional Fixes (#44)
([64b3bb1](https://github.com/MAHDTech/agent-skills/commit/64b3bb14c747f5137b5e12d391295cede03f87ae)),
closes [#44](https://github.com/MAHDTech/agent-skills/issues/44)

### [v0.1.2](https://github.com/MAHDTech/agent-skills/compare/v0.1.1...v0.1.2) (2026-07-20)

#### Fixes

* Dashboard pre-commit hook (#43)
([d4633e7](https://github.com/MAHDTech/agent-skills/commit/d4633e70e13d70b28187083457846e251e6a6e25)),
closes [#43](https://github.com/MAHDTech/agent-skills/issues/43)

### [v0.1.1](https://github.com/MAHDTech/agent-skills/compare/v0.1.0...v0.1.1) (2026-07-19)

#### Fixes

* massive cleanup (#42)
([d63ad01](https://github.com/MAHDTech/agent-skills/commit/d63ad016770a9808011aafb8135ba211cf759b13)),
closes [#42](https://github.com/MAHDTech/agent-skills/issues/42)

## v0.1.0 (2026-07-19)

### Fixes

* **github-workflows:** correct custom shell syntax in github actions
([6d39af4](https://github.com/MAHDTech/agent-skills/commit/6d39af48ea3d85bc6574dadb9fdab710c9896f5e))
* **dashboard-site:** resolve broken relative URL paths for jetbrainsmono
fonts in foundation.css
([424619e](https://github.com/MAHDTech/agent-skills/commit/424619eb7aa0105e0efdae4a5458362b85eb096e))
* **skills-cli:** resolve assets/ links relative to job.url
([6bac319](https://github.com/MAHDTech/agent-skills/commit/6bac31938cc3acd89727a0caca2e8f0f56f5fb18))
* **configs-pipelines:** resolve bad file descriptor error on trap handling in
codeql-run.sh
([dcbc95e](https://github.com/MAHDTech/agent-skills/commit/dcbc95e6722288bdfda4f193e376d97d0b6369ea))
* **skills-cli:** ignore relative links pointing outside skills directory
([2e8b6f5](https://github.com/MAHDTech/agent-skills/commit/2e8b6f57479eeab007548a9ccc388507768fd351))
* **github-workflows:** define GIPHY_TOKEN at job level for conditional steps
([c0a2c34](https://github.com/MAHDTech/agent-skills/commit/c0a2c34c277159dc43aaad4a343c557c850cf9dc))
* **dashboard-cli:** replace Atomics.wait with Bun.sleepSync on main thread
([551ce6e](https://github.com/MAHDTech/agent-skills/commit/551ce6e54380efd571554502f0481a83861857d0))
