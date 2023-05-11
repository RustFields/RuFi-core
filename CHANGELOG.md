## [1.1.0](https://github.com/RustFields/RuFi-core/compare/1.0.0...1.1.0) (2023-05-11)


### Features

* add Export ([2961083](https://github.com/RustFields/RuFi-core/commit/29610832464573345cd9e6346ea518452d00f8e0))
* add Export working API with tests ([2151f68](https://github.com/RustFields/RuFi-core/commit/2151f6896d481d3d64a01a3c0b5d0570398cfaff))
* add export_factory module ([55071b6](https://github.com/RustFields/RuFi-core/commit/55071b65b304491f3181261e3b8c3e1074fb8421))
* add method to create empty export and new export from path, update test ([2f50853](https://github.com/RustFields/RuFi-core/commit/2f508536e652b7c7b8f82dd449f758cf30af5078))
* core module is public ([1dfefd2](https://github.com/RustFields/RuFi-core/commit/1dfefd219ecafb29bf1cdd4cd0d6385be5d87522))
* slots in Path is visible only in this crate ([a090212](https://github.com/RustFields/RuFi-core/commit/a090212088ebe081e375d3ce4436654ecac214c6))


### Bug Fixes

* add Eq and Hash derive to Path ([4e35674](https://github.com/RustFields/RuFi-core/commit/4e356744961800ad2de260f7bb663c27e154fc59))
* add Eq and Hash derive to Slot ([9822ef2](https://github.com/RustFields/RuFi-core/commit/9822ef2e0075937ce6bb02317d86cad437e5c3a9))


### Build and continuous integration

* fix Semantic Release commit for updating version in Cargo.toml ([e35bb7c](https://github.com/RustFields/RuFi-core/commit/e35bb7c7fcb1988c6a04a0162af4dc05b355d8d8))


### Refactoring

* add generics ([e64d462](https://github.com/RustFields/RuFi-core/commit/e64d4620d510349f26a087766f483767282a22b2))
* put method is immutable ([8e11a57](https://github.com/RustFields/RuFi-core/commit/8e11a57ed7f96e7cc022220b160f5efd9d8c331c))


### General maintenance

* add empty space ([1e17af1](https://github.com/RustFields/RuFi-core/commit/1e17af161145caa33343b202e4325f66e2d8c625))


### Tests

* remove inner module for tests ([6a70078](https://github.com/RustFields/RuFi-core/commit/6a70078b098df6be357dfca625ae0bd4cb1b926a))


### Style improvements

* change style for docs ([6ce00f2](https://github.com/RustFields/RuFi-core/commit/6ce00f24c382af26743cbc50e99e8640041f1681))


### Documentation

* add docs to Export ([6d26c80](https://github.com/RustFields/RuFi-core/commit/6d26c80aa40c513881ae38ea2bde70e9783a87bb))
* add docs to export_factory ([c09ad8a](https://github.com/RustFields/RuFi-core/commit/c09ad8abcb503c3318a77a44502a1a35441b4b78))
* add docs to Slot ([1e57456](https://github.com/RustFields/RuFi-core/commit/1e57456bc5d7bd9c39deb4c7825cb171dd87c99f))
* improve documentation ([d25640e](https://github.com/RustFields/RuFi-core/commit/d25640edf2a0c02cceb6c9a6deff9f4105e54d95))

## 1.0.0 (2023-05-08)


### Features

* add Path ([81fe981](https://github.com/RustFields/RuFi-core/commit/81fe9814b4b985c08b872988d2cb7d1dc892a16f))
* add project structure ([40e47e2](https://github.com/RustFields/RuFi-core/commit/40e47e258eccaeb7fc8d23e5ac5551c1a2546af8))
* add Slot API ([9585197](https://github.com/RustFields/RuFi-core/commit/95851971afd40cc32162e311c8d579df8d0bc018))
* added Exchange primitive in Slot ([eefa685](https://github.com/RustFields/RuFi-core/commit/eefa68535dcf60329e43a726afd6b3142471e89a))


### Bug Fixes

* add Clone derive to Slot ([0c977fb](https://github.com/RustFields/RuFi-core/commit/0c977fb9943b728ca2db5d18130097da7b44d727))


### Refactoring

* add modules in files ([598f1e1](https://github.com/RustFields/RuFi-core/commit/598f1e182fcc0fc5fc62fb3edcb1eec85381c315))
* better implementation in head function ([c824c2b](https://github.com/RustFields/RuFi-core/commit/c824c2b44764938413f62b4f8f910590ec578601))


### Tests

* add test for matches and is_root functions ([a013e41](https://github.com/RustFields/RuFi-core/commit/a013e414a178d3d2871fe7574ec3dea725083d12))


### Documentation

* add docs for to_str and matches functions ([2cc4af9](https://github.com/RustFields/RuFi-core/commit/2cc4af9a5f00bef923085ca89609700c4dd39b02))


### General maintenance

* add authors in Cargo.toml ([cd3e482](https://github.com/RustFields/RuFi-core/commit/cd3e482a70bf83d1485f1985e0b8d4f47dfa45f3))
* add Node files to .gitignore ([050610a](https://github.com/RustFields/RuFi-core/commit/050610a859087ba2f19703ae60a9081ab0c643f3))
* Adde .gitignore ([899b506](https://github.com/RustFields/RuFi-core/commit/899b506b4e6a0bd5c46aaf662b53485ee45690da))
* Added .gitattributes ([2cb1a4e](https://github.com/RustFields/RuFi-core/commit/2cb1a4e1cd7c41aee28ce076be8645b2d9a0b240))
* Added Cargo project ([dee3446](https://github.com/RustFields/RuFi-core/commit/dee3446d4ef9ffc6313629a6ba7a2bbd8e348c60))
* Added git-hooks submodule ([01bfd89](https://github.com/RustFields/RuFi-core/commit/01bfd89605dd0c372357ad801ca51e7bd20c1f1a))
* Added Intellij Idea config to .gitignore ([9acab91](https://github.com/RustFields/RuFi-core/commit/9acab91fff55c8c9260ec242dcc9df0e232ca60f))
* Fixed LICENSE copyright ([1ef7d88](https://github.com/RustFields/RuFi-core/commit/1ef7d8868b3a7390cb3aa5eed1571a2a18b9793d))
* remove square brackets in LICENSE ([8b73970](https://github.com/RustFields/RuFi-core/commit/8b73970976a5134727405faee79370b8ae9f1ffe))


### Build and continuous integration

* Added better workflow ([39e26ed](https://github.com/RustFields/RuFi-core/commit/39e26ed8ff68cd4aa5c3c8eee7a9dc66a4301ee4))
* Added dry-run for publishing package in crates.io ([e61b3c3](https://github.com/RustFields/RuFi-core/commit/e61b3c3c4c82140099fb57fb0ac4cb37dcbefe44))
* Added job for required check ([2205c57](https://github.com/RustFields/RuFi-core/commit/2205c5794deae3cbe251dc33593ab845c996d043))
* Added release job ([30216a4](https://github.com/RustFields/RuFi-core/commit/30216a46e5a1a3d2b8f2e30493ffcf05f1fbda02))
* Added workflow for testing ([ca5d3cf](https://github.com/RustFields/RuFi-core/commit/ca5d3cf78eb8f1e4a6eebb695c806684b85650cf))
* configure Semantic Release ([539c172](https://github.com/RustFields/RuFi-core/commit/539c172e7a1be73bd723e3d8eeeb5687da47e2df))
* converted project name in snake case ([7289855](https://github.com/RustFields/RuFi-core/commit/72898550ffae2e1f086275457399ea2440e83408))
* fixed dry run for publishing crate ([16c01f9](https://github.com/RustFields/RuFi-core/commit/16c01f9f8ba6bb13285bbc02a5f866240664f7e1))
* Fixed workflow file ([1a22b51](https://github.com/RustFields/RuFi-core/commit/1a22b5122b2a781f6197b10e25776eaff57cf570))
* remove unused job in github workflow ([7a4957f](https://github.com/RustFields/RuFi-core/commit/7a4957f8e220703aa3bc3cee5b9688cb117a6a44))
* Renamed workflow ([f0405b9](https://github.com/RustFields/RuFi-core/commit/f0405b9d97795118e80b0f62d66786f0acfcc497))


### Dependency updates

* **deps:** add renovate.json ([d1d44d3](https://github.com/RustFields/RuFi-core/commit/d1d44d317cd21430a86d1bdd66b7a9e764bc9664))
* **deps:** update dependency semantic-release-preconfigured-conventional-commits to v1.1.26 ([7fbce5b](https://github.com/RustFields/RuFi-core/commit/7fbce5bfe514696f097f686e6555046981b3e64d))
* **deps:** update git-hooks digest to 026266e ([6ef24bc](https://github.com/RustFields/RuFi-core/commit/6ef24bc53de6a56a7150cbdbe587c93e70ab9de2))
* **deps:** update git-hooks digest to 70761e0 ([e3e804e](https://github.com/RustFields/RuFi-core/commit/e3e804e7ba0eebc236758fc0a2491a30c9a104e8))
* **deps:** update git-hooks digest to 8ed8da6 ([85f9749](https://github.com/RustFields/RuFi-core/commit/85f9749cf9644b1e7a1aed18cf8e5fb65adef401))
* **deps:** update git-hooks digest to abb71e7 ([2211983](https://github.com/RustFields/RuFi-core/commit/2211983fecce403a6a36363ea244ab29bfd53e0e))
