## [14.0.0](https://github.com/RustFields/RuFi-core/compare/13.1.0...14.0.0) (2023-10-16)


### ⚠ BREAKING CHANGES

* make vm clonable by using Rc with Box of Any
* made export only take a reference to Any

### Features

* add isolate function ([1d7111c](https://github.com/RustFields/RuFi-core/commit/1d7111c06c2c953a7073dbf4a12f601e4a4cfc65))
* add utility function to shorten sensor creation ([55183c4](https://github.com/RustFields/RuFi-core/commit/55183c4c7a2fc1a90fd3301b6688071ed8947a12))


### Bug Fixes

* fix isolate executing expr in the wrong time ([6f3f4b4](https://github.com/RustFields/RuFi-core/commit/6f3f4b4d679e9c00404907b80c03feef8c47e322))
* fix mux to compute both th and el before returning ([b0ebaeb](https://github.com/RustFields/RuFi-core/commit/b0ebaebd352488b0c427d36b78efee2ea1a37a7c))
* make some fields public ([b9447a0](https://github.com/RustFields/RuFi-core/commit/b9447a0d188dcaeec4a2ab50c5b11ec086dbc13b))
* prevent nbr from panicking ([d43db17](https://github.com/RustFields/RuFi-core/commit/d43db176d0e0e963e5bd4637950045e43a4e5edb))
* switch then and else in the mux call inside foldhood_plus ([6a5b541](https://github.com/RustFields/RuFi-core/commit/6a5b541806685ddb5e66998dc82fac1f05c1b3a2))


### Refactoring

* made export only take a reference to Any ([c9b06e5](https://github.com/RustFields/RuFi-core/commit/c9b06e5bacacff371b7c63dd062d2c049980b282))
* make vm clonable by using Rc with Box of Any ([b3b74f3](https://github.com/RustFields/RuFi-core/commit/b3b74f373bdb93a87d236ae1a3c09037ac7dcc79))
* use isolate function inside foldhood ([15147d9](https://github.com/RustFields/RuFi-core/commit/15147d981e7d6b07761295e42cf4408641969b20))

## [13.1.0](https://github.com/RustFields/RuFi-core/compare/13.0.1...13.1.0) (2023-10-12)


### Features

* add mux and foldhood_plus builtins ([6d35008](https://github.com/RustFields/RuFi-core/commit/6d350087564c4c4cf6e145aaae3c3e9205f18dfe))


### Bug Fixes

* fix mux signature ([74f8fa8](https://github.com/RustFields/RuFi-core/commit/74f8fa8187c5c3ecb39d376842bdd77c9600bbdd))
* remove unused imports ([5499128](https://github.com/RustFields/RuFi-core/commit/5499128f07628125570968a2d95a08879a13efc8))

## [13.0.1](https://github.com/RustFields/RuFi-core/compare/13.0.0...13.0.1) (2023-10-11)


### Dependency updates

* **deps:** update dependency semantic-release-preconfigured-conventional-commits to v1.1.68 ([a157fce](https://github.com/RustFields/RuFi-core/commit/a157fce2204dacdfb92fe7a5ce6dce7fa027a31a))
* **deps:** update node.js to 18.18 ([05c6f86](https://github.com/RustFields/RuFi-core/commit/05c6f861926f3c9da1de16524f2af228feabc605))


### Bug Fixes

* fix branch signature ([7059bcf](https://github.com/RustFields/RuFi-core/commit/7059bcfc0e20af4088dd912920e3ba7508c66ad8))


### Build and continuous integration

* **deps:** update actions/checkout action to v4 ([ddfdcd8](https://github.com/RustFields/RuFi-core/commit/ddfdcd83b3a8e90b91fce946b45a5bcc9475f626))


### Refactoring

* refactor branch signature ([360d29b](https://github.com/RustFields/RuFi-core/commit/360d29b1d9b7d3b41a31f53f2f4d48ad4a4c64f3))

## [13.0.0](https://github.com/RustFields/RuFi-core/compare/12.0.2...13.0.0) (2023-10-09)


### ⚠ BREAKING CHANGES

* moved functions to test/utils
* change the signature of foldhood to accept constructs as inits
* change the signature of rep to accept constructs as inits

### Features

* **test:** add utility to assert the equivalence of programs ([3cd824e](https://github.com/RustFields/RuFi-core/commit/3cd824ecf3de761e6b9198296f23d9b34c54c2d1))
* **test:** add utility to create a fully connected topology ([f63df0b](https://github.com/RustFields/RuFi-core/commit/f63df0b9a06082f8d0ba3c612a04d65861654607))
* **test:** add utility to create vms ([1396037](https://github.com/RustFields/RuFi-core/commit/139603707d865b71c74c98fc2f81384734e7a15b))


### Tests

* add first two tests by equivalence ([321e7f4](https://github.com/RustFields/RuFi-core/commit/321e7f45fb0f51cefc71e2ad8cd0bf623e9c59a0))
* add more tests by round ([c0ef14d](https://github.com/RustFields/RuFi-core/commit/c0ef14da57d45719efa045e61ce9dce9629887e8))


### General maintenance

* **build:** add rand dependency ([77ed738](https://github.com/RustFields/RuFi-core/commit/77ed738f3765a7d4941f9b446e6090d4ab301278))


### Style improvements

* change indentation and use trailing commas in `lang.rs` ([bb932bb](https://github.com/RustFields/RuFi-core/commit/bb932bb797b0ea1d22ee1199c227d121c5c8e174))


### Refactoring

* change the signature of foldhood to accept constructs as inits ([cac82cb](https://github.com/RustFields/RuFi-core/commit/cac82cba90cc2ca97ae93bbfcb996fec8157c91d))
* change the signature of rep to accept constructs as inits ([543e120](https://github.com/RustFields/RuFi-core/commit/543e1201529333ae216f3e72bcb8d2ebcc92894f))
* move lang tests on subdirectory and separate tests by round from by equivalence ([4231aff](https://github.com/RustFields/RuFi-core/commit/4231aff009135fd43fbee094845532c6d02a2111))
* moved functions to test/utils ([93e26d6](https://github.com/RustFields/RuFi-core/commit/93e26d6e92b6dcbd305f3b49e1b2c2f26b50a489))
* **test:** restructure tests ([2bee87b](https://github.com/RustFields/RuFi-core/commit/2bee87b15e338dc39351f75c25da5e95e311e555))

## [12.0.2](https://github.com/RustFields/RuFi-core/compare/12.0.1...12.0.2) (2023-07-31)


### Documentation

* add folded_eval documentation ([bff42e7](https://github.com/RustFields/RuFi-core/commit/bff42e74cca063cfcbd905c713e6dddb667773a7))
* add language documentation ([1efe1ee](https://github.com/RustFields/RuFi-core/commit/1efe1ee85d49609f1e2abb983e5e0f24a781b9a4))


### Refactoring

* refactor lang functions signatures ([d82b6a3](https://github.com/RustFields/RuFi-core/commit/d82b6a3983e3955425f6864ef9708653cae8da2c))

## [12.0.1](https://github.com/RustFields/RuFi-core/compare/12.0.0...12.0.1) (2023-07-27)


### Dependency updates

* **deps:** update dependency semantic-release-preconfigured-conventional-commits to v1.1.43 ([b811758](https://github.com/RustFields/RuFi-core/commit/b81175875dca7b159a03e45e8d72d2ca60794b17))
* **deps:** update node.js to 18.17 ([82e68ef](https://github.com/RustFields/RuFi-core/commit/82e68ef301d2e8b701a53f22cec7671a926fe6ce))


### Bug Fixes

* change locally to match the original scafi implementation ([38083a8](https://github.com/RustFields/RuFi-core/commit/38083a86f8599ce98f2392b728e8d687fcff15a6))
* change rep and foldhood internal behaviour ([fba8c89](https://github.com/RustFields/RuFi-core/commit/fba8c89d0b630875b8ded98ff0a6274909a856a5))
* now aligned neighbors are correctly returned ([e9f906f](https://github.com/RustFields/RuFi-core/commit/e9f906f3633c8a7d0850caed516ace657e986cbc))


### Tests

* add foldhood failure test ([57ec80d](https://github.com/RustFields/RuFi-core/commit/57ec80daae977a4ec3aef4e502c8c35a434b2b7a))
* add rep test ([5b524a7](https://github.com/RustFields/RuFi-core/commit/5b524a7fc74cc8d0f560cc4d63cd2df12542e5ba))
* alignment and foldhood are now tested ([63fafa2](https://github.com/RustFields/RuFi-core/commit/63fafa2f614e6d98ed115415b1bdf2269dd3999e))
* fix test exports ([5e39e77](https://github.com/RustFields/RuFi-core/commit/5e39e77b272983b55a19cfc1e77d0cf0e5eb185e))
* reduce the number of external functions used ([b72ae7b](https://github.com/RustFields/RuFi-core/commit/b72ae7bcf519c4eb59b2b20b94dcd2dc0e0d79d5))
* refactor export creation and fix rep tests ([0f00a93](https://github.com/RustFields/RuFi-core/commit/0f00a936e7f6683ed89efcdea8f66d988b47caf1))
* refactor test (code quality, comments etc..) ([caf978d](https://github.com/RustFields/RuFi-core/commit/caf978dd8070da45307591655d00ef6758847a6b))
* remove comments ([4f292e3](https://github.com/RustFields/RuFi-core/commit/4f292e3b6fc175803d56519168185e252c48cf59))


### General maintenance

* add example of folded eval issue ([bc4d569](https://github.com/RustFields/RuFi-core/commit/bc4d569cd3b6badb302a3ac19a3d543cce09abd5))


### Refactoring

* change folded_eval code to be more clear ([bdee508](https://github.com/RustFields/RuFi-core/commit/bdee508d9f89b7602f1ea28feac167ed8cd2837c))
* remove mid function ([53e9835](https://github.com/RustFields/RuFi-core/commit/53e9835fdc7aa021cb63461022f743a7c58b987b))

## [12.0.0](https://github.com/RustFields/RuFi-core/compare/11.1.0...12.0.0) (2023-07-17)


### ⚠ BREAKING CHANGES

* refactor nbr signature

### Refactoring

* refactor nbr signature ([b0ff292](https://github.com/RustFields/RuFi-core/commit/b0ff292992555d3e2ad4afe15ff4e40c732b9bb9))

## [11.1.0](https://github.com/RustFields/RuFi-core/compare/11.0.0...11.1.0) (2023-07-17)


### Features

* add macro for export creation ([c1a02d4](https://github.com/RustFields/RuFi-core/commit/c1a02d417b31a90c3c9fb6661c753bf507930ad7))
* add macro for path ([5f44ef8](https://github.com/RustFields/RuFi-core/commit/5f44ef83d4c50b06fca897de663a859e02fa3200))


### Refactoring

* **test:** refactor tests to use macros ([11188bc](https://github.com/RustFields/RuFi-core/commit/11188bc6f24a05f1ef18f6e94befbf5e34496db9))

## [11.0.0](https://github.com/RustFields/RuFi-core/compare/10.1.0...11.0.0) (2023-07-17)


### ⚠ BREAKING CHANGES

* changed expr to take and return a vm to allow combination with other operators

### Refactoring

* changed expr to take and return a vm to allow combination with other operators ([2258e31](https://github.com/RustFields/RuFi-core/commit/2258e313367bb26ade92d229f44c422a3c71da7f))

## [10.1.0](https://github.com/RustFields/RuFi-core/compare/10.0.0...10.1.0) (2023-07-17)


### Features

* implemented functions to manipulate multiple aggregate programs ([655698f](https://github.com/RustFields/RuFi-core/commit/655698ffc1f813104ba84dc462850192d2f6f4c6))

## [10.0.0](https://github.com/RustFields/RuFi-core/compare/9.1.0...10.0.0) (2023-07-14)


### ⚠ BREAKING CHANGES

* remove unused function

### Features

* add nest in out and write ([53dec31](https://github.com/RustFields/RuFi-core/commit/53dec31b676c70feb9c5ec6b77debc2e55148c2f))


### Bug Fixes

* fix foldhood disalignment ([a47a294](https://github.com/RustFields/RuFi-core/commit/a47a29493f15f8f56111b0dc328facf04de0ac2d))
* **test:** fix rep test ([56432ca](https://github.com/RustFields/RuFi-core/commit/56432ca4a951810258fbb98bcf771a5d5da11b36))


### Refactoring

* make nest_write return the input value ([49ef3a0](https://github.com/RustFields/RuFi-core/commit/49ef3a034d52e1975b8c0899bdf5d26c2e1be434))
* refactor language constructs to use nest in write and out ([3e2be40](https://github.com/RustFields/RuFi-core/commit/3e2be40d9837f766583b533613abeec6cfa1b1ea))
* remove unused function ([fea140b](https://github.com/RustFields/RuFi-core/commit/fea140b306684b7dd1e032972602b070c52b436a))

## [9.1.0](https://github.com/RustFields/RuFi-core/compare/9.0.1...9.1.0) (2023-07-13)


### Features

* implement mid() construct ([61a018e](https://github.com/RustFields/RuFi-core/commit/61a018ee41866444d4e0792d3bf8b9a1a904b3ef))


### Dependency updates

* **deps:** update dependency semantic-release-preconfigured-conventional-commits to v1.1.41 ([42336b8](https://github.com/RustFields/RuFi-core/commit/42336b831f1cc636020029d41a8a4d59e5789751))

## [9.0.1](https://github.com/RustFields/RuFi-core/compare/9.0.0...9.0.1) (2023-07-11)


### Bug Fixes

* fixed nest not writing the status correctly ([2bf473d](https://github.com/RustFields/RuFi-core/commit/2bf473d0c4c8a9b6f2dd5ee35aeb80c478835dcb))
* **test:** fixed nest test not expecting correct value ([f379c6c](https://github.com/RustFields/RuFi-core/commit/f379c6cec84d5b433446a4abf2a3a7ef19de2c3c))

## [9.0.0](https://github.com/RustFields/RuFi-core/compare/8.0.0...9.0.0) (2023-07-07)


### ⚠ BREAKING CHANGES

* moved locally out of round_vm

### Refactoring

* moved locally out of round_vm ([97889da](https://github.com/RustFields/RuFi-core/commit/97889da1880368839d1648362ffc3b103ddc707f))

## [8.0.0](https://github.com/RustFields/RuFi-core/compare/7.0.0...8.0.0) (2023-07-07)


### ⚠ BREAKING CHANGES

* use option in a more idiomatic way

### Bug Fixes

* use option in a more idiomatic way ([08dafc2](https://github.com/RustFields/RuFi-core/commit/08dafc2435c0df2f4592ceaee53971f91d5eb68d))

## [7.0.0](https://github.com/RustFields/RuFi-core/compare/6.1.0...7.0.0) (2023-07-07)


### ⚠ BREAKING CHANGES

* add round

### Features

* add core constructs to implement ([7adad4b](https://github.com/RustFields/RuFi-core/commit/7adad4b3b9a58e222946198bfaa9308b57849ed3))
* add round ([e8f1837](https://github.com/RustFields/RuFi-core/commit/e8f18376103b4d961eacf06851aa6212227e97a5))
* implement branch ([662e17d](https://github.com/RustFields/RuFi-core/commit/662e17d9fa4ea57bb182d55d5c4937caf7b61a63))
* implement foldhood construct ([dfa2e58](https://github.com/RustFields/RuFi-core/commit/dfa2e58dc3837cfe1de9e6808f5fbdd076d0be61))
* implement nbr and rep ([c26db41](https://github.com/RustFields/RuFi-core/commit/c26db41635824300e0a1428cb856b5948af2be3a))


### Bug Fixes

* new now add an empty export to the stack ([2b00de2](https://github.com/RustFields/RuFi-core/commit/2b00de2d080cbc00151603492e3d7db1e67d8067))
* now the expression is computed correctly locally ([e004ce1](https://github.com/RustFields/RuFi-core/commit/e004ce1e0d73c5f7ae2285e6e89ea7e28883dda7))
* remove useless imports ([d372375](https://github.com/RustFields/RuFi-core/commit/d372375c8e3addf0b7e878441fd1edd776a95af2))
* revert problematic change made previously ([db036c6](https://github.com/RustFields/RuFi-core/commit/db036c6f954f75dec596e86dd6422fb95e1e1265))
* **test:** fix round vm creation for testing ([0960b74](https://github.com/RustFields/RuFi-core/commit/0960b749f130a5ff759c11bc33fb219f52bfe010))


### Tests

* improve test for multiple rounds ([28da239](https://github.com/RustFields/RuFi-core/commit/28da239a7884bb337848a0344a2a96223a2f1deb))
* improve tests ([e50a916](https://github.com/RustFields/RuFi-core/commit/e50a9160ce6c593f1cfc5b12477756e943c6c0d3))


### Refactoring

* remove useless vm duplication ([ac27413](https://github.com/RustFields/RuFi-core/commit/ac27413f195434d09e878707cfd11daac66b1484))
* remove useless vm duplication ([c113ece](https://github.com/RustFields/RuFi-core/commit/c113ece8b45cb0d5a372e1d62cb9c5f0233b789c))
* round now returns also the result of the computation ([4723099](https://github.com/RustFields/RuFi-core/commit/4723099f138d5daf947bc588ab2f87b792d0b5ae))
* **test:** add empty export to stack on vm creation ([b250d8e](https://github.com/RustFields/RuFi-core/commit/b250d8e6ccf81092f48cc3ea4c2234b0c14ca947))
* **test:** round now returns also the result of the computation ([e88ee08](https://github.com/RustFields/RuFi-core/commit/e88ee08379e9b7b60282551fc7763df84ad26ced))

## [6.1.0](https://github.com/RustFields/RuFi-core/compare/6.0.1...6.1.0) (2023-07-06)


### Features

* add foldhood slot ([ec4d2b3](https://github.com/RustFields/RuFi-core/commit/ec4d2b381642e033a4762de11bd555768fc4c581))


### Dependency updates

* **deps:** update dependency semantic-release-preconfigured-conventional-commits to v1.1.39 ([bf6379e](https://github.com/RustFields/RuFi-core/commit/bf6379e67a4bb1baae4a18ca46c98b510850240b))

## [6.0.1](https://github.com/RustFields/RuFi-core/compare/6.0.0...6.0.1) (2023-07-03)


### Documentation

* fix doc for SensorId ([f0da921](https://github.com/RustFields/RuFi-core/commit/f0da921afd02cf3f256bca01575e2b81e7aa9810))
* improve context docs ([749b78e](https://github.com/RustFields/RuFi-core/commit/749b78e7e330ad2bf0abe76ce427dc744af56ca2))
* improve context docs ([fc98166](https://github.com/RustFields/RuFi-core/commit/fc98166342821d40b7f32cdfbe46eca2be08e0d0))
* improve context docs ([be46628](https://github.com/RustFields/RuFi-core/commit/be466280c1132f08b9b34b4c5eeddee781864f72))
* improve docs ([880313d](https://github.com/RustFields/RuFi-core/commit/880313df7af8f23d3ef0b3acd8e562761a7870df))
* improve export docs ([352e086](https://github.com/RustFields/RuFi-core/commit/352e08696915856522b082a01acb536a8b5860f4))
* improve path docs ([bf08065](https://github.com/RustFields/RuFi-core/commit/bf0806544f93bf548d7903ca9edc294df3cf9f35))
* improve round_vm docs ([61b6ded](https://github.com/RustFields/RuFi-core/commit/61b6ded667b865c7f0ab6f9875b96c06248ef9ba))
* improve sensor_id docs ([8ebae99](https://github.com/RustFields/RuFi-core/commit/8ebae997062351a0556e43854cf0bb3ce53acc68))
* improve slot docs ([19e9721](https://github.com/RustFields/RuFi-core/commit/19e9721a747fcddc64a1fdd3256619864e70d7b7))
* improve vm_status docs ([41a864d](https://github.com/RustFields/RuFi-core/commit/41a864d4d6962459a1dd08b969a58a25cd6f608f))

## [6.0.0](https://github.com/RustFields/RuFi-core/compare/5.1.0...6.0.0) (2023-07-03)


### ⚠ BREAKING CHANGES

* remove deprecated constructors in favor of From

### Features

* implement From trait for Export, Path and VMStatus ([7c78439](https://github.com/RustFields/RuFi-core/commit/7c78439a1712736bf9ffad13bd98c28ada2d8f42))


### Dependency updates

* **deps:** update dependency semantic-release-preconfigured-conventional-commits to v1.1.38 ([b7b1146](https://github.com/RustFields/RuFi-core/commit/b7b11464608ce2ea2abdc63ba59e90c67e86224d))


### Tests

* change tests to use new construcrors and From ([f86064b](https://github.com/RustFields/RuFi-core/commit/f86064b96b290d06d61539981bbd09170f7a5bc1))


### Refactoring

* change default parameter in new ([e5f8f15](https://github.com/RustFields/RuFi-core/commit/e5f8f15f92147c883219c24eeacd4c8be10e5ed2))
* remove deprecated constructors in favor of From ([aacda07](https://github.com/RustFields/RuFi-core/commit/aacda0788d05af8b55572b0b447a961784cb80ed))
* switch data structures names to self in creation ([f706179](https://github.com/RustFields/RuFi-core/commit/f706179ee44788352765a0bace8ed10385319116))

## [5.1.0](https://github.com/RustFields/RuFi-core/compare/5.0.0...5.1.0) (2023-06-22)


### Features

* add empty constructor in Export ([b7ac465](https://github.com/RustFields/RuFi-core/commit/b7ac465039a1a950fad38c8555dbc46edda7e28e))
* add empty constructor in RoundVM ([f37dd33](https://github.com/RustFields/RuFi-core/commit/f37dd3307015b9a81e3bf2ec1ddc000b2b61f37d))
* add empty constructor in VMStatus ([379e8eb](https://github.com/RustFields/RuFi-core/commit/379e8ebfbc02e70c40c661acb584ff57e44c0b50))


### Dependency updates

* **deps:** update dependency semantic-release-preconfigured-conventional-commits to v1.1.30 ([bbe2332](https://github.com/RustFields/RuFi-core/commit/bbe23329ade7670de35e13ed150168e5124cb127))
* **deps:** update dependency semantic-release-preconfigured-conventional-commits to v1.1.32 ([fcd6dbe](https://github.com/RustFields/RuFi-core/commit/fcd6dbede5f0c35d5f3d4826f0ac2c32b72bd304))
* **deps:** update dependency semantic-release-preconfigured-conventional-commits to v1.1.33 ([1edcf0f](https://github.com/RustFields/RuFi-core/commit/1edcf0fe61f3fe7cb85f4878d30e9f59b709143c))
* **deps:** update dependency semantic-release-preconfigured-conventional-commits to v1.1.35 ([50f2475](https://github.com/RustFields/RuFi-core/commit/50f2475ace5cf0f62339de49bd182b295ee94e81))
* **deps:** update dependency semantic-release-preconfigured-conventional-commits to v1.1.37 ([d92b98b](https://github.com/RustFields/RuFi-core/commit/d92b98bfb2009108bef8b14894315e9d0237e202))


### Tests

* fix test in vm_status ([afa4540](https://github.com/RustFields/RuFi-core/commit/afa45406f2f60c7f7b2ceca46026317b2143f2ee))


### Build and continuous integration

* **Mergify:** configuration update ([b5ce6b0](https://github.com/RustFields/RuFi-core/commit/b5ce6b057df6c571a18d277db5a8c8be273a3a22))


### Refactoring

* replace Path::new(vec![]) with Path::new_empty() ([7e81d8d](https://github.com/RustFields/RuFi-core/commit/7e81d8d4f9dd95f0240b2878795cb9e3fa8eb5b3))

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
