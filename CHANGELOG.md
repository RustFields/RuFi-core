## [5.0.0](https://github.com/RustFields/RuFi-core/compare/4.0.0...5.0.0) (2023-05-31)


### ⚠ BREAKING CHANGES

* put function in export now takes a closure as value

### Features

* add correct implementation to nest function ([22c9be2](https://github.com/RustFields/RuFi-core/commit/22c9be28c3c8f2c2d8e58868333eceb50a70953a))
* add new functions to round_vm ([b12c5b6](https://github.com/RustFields/RuFi-core/commit/b12c5b6aaab75d6cf85eb83a1d1283ad16c0018a))
* add new_empty function in Path ([52af8b7](https://github.com/RustFields/RuFi-core/commit/52af8b70b86f34140e6f66eb1065cbef2d5c8d6e))
* add PartialEq and Clone to VMStatus ([95d1f89](https://github.com/RustFields/RuFi-core/commit/95d1f89a2f8b6af146e9a8a1e2fca12bcbfba20c))
* add some missing unimplemented functions ([c85c6bb](https://github.com/RustFields/RuFi-core/commit/c85c6bb5a67f423576907d24de5e2a506bd15e36))
* add vm_status module ([d9c59d9](https://github.com/RustFields/RuFi-core/commit/d9c59d9bc28ddaa2d0565ffa2625573c0956867e))
* change return type of neighbor_val, local_sense and nbr_sense ([afb15dc](https://github.com/RustFields/RuFi-core/commit/afb15dcb454848a1daa7e2d70a8ce6987c27836d))
* change self_id return type ([e6b6338](https://github.com/RustFields/RuFi-core/commit/e6b633872f9c71797e7ce3bb0dc4347e2e53c7ff))
* implement aligned_neighbours function ([1539f58](https://github.com/RustFields/RuFi-core/commit/1539f58ffcbf55238fe7af56d5d16e56e56142a7))
* implement folded_eval ([7096188](https://github.com/RustFields/RuFi-core/commit/70961886a889e5840b03c5fe4cdfd58fd3d162f9))
* implement isolate function ([7dfe095](https://github.com/RustFields/RuFi-core/commit/7dfe0955e1437458c11bce59b332bf2e14a1e38b))
* implement locally function ([0777ea6](https://github.com/RustFields/RuFi-core/commit/0777ea6a12d1cbbc705d2b5e6bfd2afcbbf96a01))
* implement new/discard/merge export ([65eb626](https://github.com/RustFields/RuFi-core/commit/65eb62636605738dd1e912fc463bdd82ad0e36e7))
* implement round_vm module ([1832a9a](https://github.com/RustFields/RuFi-core/commit/1832a9a0cd2b8249dbd360599975cdcf12cd2159))
* partially implement nest function ([70712fa](https://github.com/RustFields/RuFi-core/commit/70712faede2ae5767e180f0dccf543a87e19ccbe))


### Bug Fixes

* add factory to new() ([0e165ec](https://github.com/RustFields/RuFi-core/commit/0e165ec7253f298b8a7a783db82527e584c0983d))
* error on types in folding functions ([3c2a888](https://github.com/RustFields/RuFi-core/commit/3c2a888678cd29287e6f369ae1d3a983228a271c))
* fix aligned_neighbours function ([4d907bf](https://github.com/RustFields/RuFi-core/commit/4d907bf866981a652aacf3084a0a96b8cd46bc79))
* fix compilation error ([5cc1bcb](https://github.com/RustFields/RuFi-core/commit/5cc1bcb67d1be0d015143505292bbd881089a839))
* fix isolate function ([57760ba](https://github.com/RustFields/RuFi-core/commit/57760ba49b17f36dee8c74f7fb4d313997c31366))
* fix nest function ([70afefe](https://github.com/RustFields/RuFi-core/commit/70afefe8b2886ad48fe49629e97b4b304c83f2f1))
* fix only_when_folding_on_self and unless_folding_on_others functions ([9a9309b](https://github.com/RustFields/RuFi-core/commit/9a9309bedffbe1df76289e698b5b94efb3abe3bf))


### General maintenance

* add unimplemented macro ([b045952](https://github.com/RustFields/RuFi-core/commit/b045952d863c229d051209eb76ee705c8ccfe758))
* remove unused import ([1ca0c99](https://github.com/RustFields/RuFi-core/commit/1ca0c99257200f9b0d7f21f1722f45fb17911cf5))


### Style improvements

* apply rustfmt rules ([376ea87](https://github.com/RustFields/RuFi-core/commit/376ea87878a37ee2df7e31591cee3ccf18d4403a))


### Refactoring

* adapt round_vm to use export ([a65d1dc](https://github.com/RustFields/RuFi-core/commit/a65d1dcfb9044844e1f11c4ea58442a0f8299b61))
* add correct implementation in isolate function ([59f1fbf](https://github.com/RustFields/RuFi-core/commit/59f1fbfad2fd74b9005341470824d4f64489a0d5))
* apply rustfmt rules ([a82cc24](https://github.com/RustFields/RuFi-core/commit/a82cc24b03558a6ddf44e87c8769307542d8f49a))
* apply rustfmt rules in round_vm ([0f5145a](https://github.com/RustFields/RuFi-core/commit/0f5145aada8ecc5e972e1679c44eeb8d2910cf81))
* apply rustfmt rules in vm_status ([56153e1](https://github.com/RustFields/RuFi-core/commit/56153e1dc5d1c0cd51d2bac73204681a7b7c4c40))
* change function name in round_vm ([1702207](https://github.com/RustFields/RuFi-core/commit/17022071413d93c3a71aa573b6261e2586a4d261))
* change parameter name in locally function ([7cf56dc](https://github.com/RustFields/RuFi-core/commit/7cf56dc9e839b26997adc4b8ce60d1c146caba1b))
* change RoundVM to Self in new ([b7e068f](https://github.com/RustFields/RuFi-core/commit/b7e068fbcf07cd5d72f092a420fa4e93d40ac004))
* make mutable val immutable ([e1d59f7](https://github.com/RustFields/RuFi-core/commit/e1d59f7c4a03c71a8a99cf71ac34c8190a61d672))
* now folded_eval takes an higher order as expr ([ec70771](https://github.com/RustFields/RuFi-core/commit/ec707711dcb7e9be2490c9da9e89a5ab25f254eb))
* put function in export now takes a closure as value ([f3a0872](https://github.com/RustFields/RuFi-core/commit/f3a087261b6cc5654700185e126efc2e59ae14dc))
* refactor fnunctions pop and new ([86e3a1d](https://github.com/RustFields/RuFi-core/commit/86e3a1d953cbdddf86e50128bce7dbf621d79ea8))
* register_root no longer take a function as parameter ([c97c4de](https://github.com/RustFields/RuFi-core/commit/c97c4dec367173a083392edcdb45bab758d0af83))
* register_root now takes an higher order function ([51637b9](https://github.com/RustFields/RuFi-core/commit/51637b91e2865812ef151a0705fe35d5b4bc294f))
* remove closures inside nest function ([7277cec](https://github.com/RustFields/RuFi-core/commit/7277cece10987a2bdab1fd5d835603a3ab336922))
* remove export_factory dependency ([920d790](https://github.com/RustFields/RuFi-core/commit/920d790c4d4c7497f9be59d561684c1a21c7c6af))
* remove unused parameter in closure ([2e8af99](https://github.com/RustFields/RuFi-core/commit/2e8af99fd591dbc26302d894eb113ad4dd26bd0e))
* remove useless functions ([00e6d3e](https://github.com/RustFields/RuFi-core/commit/00e6d3eb230d94ae8b5f67e83e3b0cbe31248c67))
* remove useless variable definition ([297897f](https://github.com/RustFields/RuFi-core/commit/297897fb0748c43309100b49dff0046065e2dc56))
* rewrite nest function in more readable way ([8875c7a](https://github.com/RustFields/RuFi-core/commit/8875c7acb2059cdb643a531f6db2e12ee57a5420))
* rewritten functions more idiomatically ([f88d596](https://github.com/RustFields/RuFi-core/commit/f88d596a23e10269ec21b3ebb4c663f6aff8eb38))
* switch if let with math in nest function ([689d038](https://github.com/RustFields/RuFi-core/commit/689d0388b50b6e07f8593929f6f06c243bfd558b))


### Documentation

* add documentation for locally function ([e443dbc](https://github.com/RustFields/RuFi-core/commit/e443dbc691be99fabffbae084973aec9ce89ee2b))
* add documentation for RoundVM functions ([6f31b2e](https://github.com/RustFields/RuFi-core/commit/6f31b2e8841a74688103f2087cc1f108133bbe53))
* add documentation in vm_status ([8ef5b28](https://github.com/RustFields/RuFi-core/commit/8ef5b281ed7fb392140203ace9debc58e7af7632))
* improve documentation ([0e0323a](https://github.com/RustFields/RuFi-core/commit/0e0323a1b474f2559ad575b4488005be9ce4f468))
* improve documentation ([495aadb](https://github.com/RustFields/RuFi-core/commit/495aadb4da2ce24c7f643894de45937253c7304d))
* improve documentation for nest function ([697ab22](https://github.com/RustFields/RuFi-core/commit/697ab222bcf746ef651059e302fdebfac4fc9e5c))


### Tests

* add test for local_sense function ([5ca7eca](https://github.com/RustFields/RuFi-core/commit/5ca7ecaf0343d8d7734681e292e2e52b57f7332e))
* add test for neighbor_val ([1d9c78f](https://github.com/RustFields/RuFi-core/commit/1d9c78fac6b30936c911edb20003da984ee84b54))
* add tests for following functions ([6ef5b1f](https://github.com/RustFields/RuFi-core/commit/6ef5b1fe7265a24c0573f5711251cfe91a232687))
* add tests for following functions ([e5c9d9c](https://github.com/RustFields/RuFi-core/commit/e5c9d9c892d8f80dcdf4cef90e028f8a04b80fe8))
* add tests for following functions ([043bdad](https://github.com/RustFields/RuFi-core/commit/043bdad7bf34e0afc7889156a6ff072a5faf4a3a))
* add tests for vm_status ([220d96b](https://github.com/RustFields/RuFi-core/commit/220d96b16422eebe1046c90f01515b410e0cb725))
* change sensor name in test ([6e035d6](https://github.com/RustFields/RuFi-core/commit/6e035d6e1a5006e5a27af4e4cf09c093c6b81e4f))
* fix test for RoundVM ([a5d1929](https://github.com/RustFields/RuFi-core/commit/a5d1929305198a37d5ca36ab5ac4865d34e69929))
* fix test_as_stack in round_vm ([e23465a](https://github.com/RustFields/RuFi-core/commit/e23465a7f15faff8cae3f45b752a5c8842d5e448))
* fix tests in vm_status, export_factory is no longer used ([82e8556](https://github.com/RustFields/RuFi-core/commit/82e8556d971f6f9661f6b82a43907a23d9adbae9))
* import tests from export_factory module to export module ([8ebf449](https://github.com/RustFields/RuFi-core/commit/8ebf449b958f86236e8b639e8628e96ddd5df8da))
* remove mutability were not needed ([f380910](https://github.com/RustFields/RuFi-core/commit/f380910a0ba0eca4a0da41878bd5c2b72df26dea))

## [4.0.0](https://github.com/RustFields/RuFi-core/compare/3.0.0...4.0.0) (2023-05-29)


### ⚠ BREAKING CHANGES

* remove export_factory module

### Refactoring

* remove export_factory module ([98fb6b9](https://github.com/RustFields/RuFi-core/commit/98fb6b9748a891bb4ddb80bfd41afaa482583b8c))

## [3.0.0](https://github.com/RustFields/RuFi-core/compare/2.0.0...3.0.0) (2023-05-18)


### ⚠ BREAKING CHANGES

* refactor context API

### Refactoring

* apply rustfmt rules in path ([7fd592b](https://github.com/RustFields/RuFi-core/commit/7fd592b875c08d37311b4ac6b841b3c1c34dc239))
* refactor context API ([1197dae](https://github.com/RustFields/RuFi-core/commit/1197dae8302b2daaa341e8d2bf189a7c8652f53b))

## [2.0.0](https://github.com/RustFields/RuFi-core/compare/1.2.0...2.0.0) (2023-05-17)


### ⚠ BREAKING CHANGES

* fix path behavior

### Bug Fixes

* fix path behavior ([2e24e85](https://github.com/RustFields/RuFi-core/commit/2e24e85820e1f1ace4147cb0e6cba63b35e6098e))


### Refactoring

* apply rustfmt rules ([630846e](https://github.com/RustFields/RuFi-core/commit/630846eb15ab6e2c4fee43ae75a6090a7497a69c))

## [1.2.0](https://github.com/RustFields/RuFi-core/compare/1.1.0...1.2.0) (2023-05-16)


### Features

* add context module ([67f11c2](https://github.com/RustFields/RuFi-core/commit/67f11c26edd03cbb43b241511e7e44b92b5cec28))
* add functions to context ([e2cdc7b](https://github.com/RustFields/RuFi-core/commit/e2cdc7b0ce97bca4c1d950943e93711cddb67c03))
* add sensor_id module ([dd89933](https://github.com/RustFields/RuFi-core/commit/dd89933d7b5c8ff8280e74ec581e26ebaf6ae323))
* implement exports_map function and test ([316cb39](https://github.com/RustFields/RuFi-core/commit/316cb392b2e1675067ea86a0b31722456408e00d))
* implement nbr_sense function and test ([ec947c9](https://github.com/RustFields/RuFi-core/commit/ec947c9fa22a428428cc20a65bac5739d57dce84))
* implement read_slot function and test ([1423b77](https://github.com/RustFields/RuFi-core/commit/1423b7731bd72982751f900fd9cb98d2557187b1))
* implement sense function and test ([f3b5805](https://github.com/RustFields/RuFi-core/commit/f3b5805921ff7f3f10311da4247f2ad0f1f4f684))


### Bug Fixes

* add derive to sensor_id ([6c1e5e4](https://github.com/RustFields/RuFi-core/commit/6c1e5e4a460aac18ce7f59e069b3f5bfbbd96583))


### Documentation

* update Context docs ([adb1363](https://github.com/RustFields/RuFi-core/commit/adb1363e7e4b965501def700f0a95d4fd3289077))


### Refactoring

* change methods and params names ([bf2d31a](https://github.com/RustFields/RuFi-core/commit/bf2d31a704ea1e19febc69d3f14fa0291e8fcc59))
* refactor names in sensor_id ([32a258c](https://github.com/RustFields/RuFi-core/commit/32a258c055539380ba524c65207f59fa797d5d49))


### Tests

* add import to tests ([3f2e5a1](https://github.com/RustFields/RuFi-core/commit/3f2e5a13239941c84e3c7d0e34e4517cc7a64ff3))
* improve tests for context ([9928267](https://github.com/RustFields/RuFi-core/commit/9928267ea8591097e6d4244c9a0db3e1d81fcd82))

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
