# [0.26.1](https://github.com/cladam/tbdflow/releases/tag/v0.26.1) (2026-04-11)

### ✨ Features
- add actual append to .gitignore for the intent log [`679ed88`](https://github.com/cladam/tbdflow/commit/679ed88afe147ea2987ef074f5ac9ca5e8de4772)

### ⚙️ Maintenance
- **(release):** bump version and publish to GH Releases and crates.io [`b3fa7b6`](https://github.com/cladam/tbdflow/commit/b3fa7b639fec81eae3cfedd34e065f502ddd4bb9)
- add tbdflow-agent to this repo and cleaning up [`d0af7ba`](https://github.com/cladam/tbdflow/commit/d0af7ba9c15fbad99552a41cd514cf8d1ba7af53)
- update SKILL.md in this repo [`0ec5f32`](https://github.com/cladam/tbdflow/commit/0ec5f325544e5b50f3f9e53ee0152c92e1dd8dd8)
- wording [`a1d3648`](https://github.com/cladam/tbdflow/commit/a1d36489b7558f1f73d0d03500f2ad1497f6476e)
- update tbdflow SKILL and AGENTS files, for the Genie [`02132f1`](https://github.com/cladam/tbdflow/commit/02132f1f3833386a6b795e8ea95721920faea90b)
- restructure README so it flows better and not so spec-ish [`1c1fc14`](https://github.com/cladam/tbdflow/commit/1c1fc146e4121cde68e697d8d0e38654e7369480)


# [0.26.0](https://github.com/cladam/tbdflow/releases/tag/v0.26.0) (2026-04-11)

### ✨ Features
- **(intent):** add intent log for capturing pre-work context [`db21c67`](https://github.com/cladam/tbdflow/commit/db21c6799daf1daa9fbedf3dd08e043098447afb)

### 🐛 Bug Fixes
- remove unessecary comments, redundant in most cases [`d84cefe`](https://github.com/cladam/tbdflow/commit/d84cefe17b5e43e8d1f8b9da5ca1af3e16b1c55c)
- remove all emojis except in the changelog feature [`2f9d96a`](https://github.com/cladam/tbdflow/commit/2f9d96a08b6a16781a5dc3440e90d6e8d50579b4)
- changed yaml crate and did a review [`9a8d019`](https://github.com/cladam/tbdflow/commit/9a8d0196e786679cc598edc091997c13b6414d89)

### 🔨 Code Refactoring
- add linting tests to commit.rs [`ed2896e`](https://github.com/cladam/tbdflow/commit/ed2896e66d5f1acc671360f5c5c34d5900af5fb0)

### ⚙️ Maintenance
- **(release):** bump version and publish to GH Releases and crates.io [`8055e67`](https://github.com/cladam/tbdflow/commit/8055e677284d41ad6ee3c52efe6d9aee594f006d)


# [0.25.3](https://github.com/cladam/tbdflow/releases/tag/v0.25.3) (2026-04-11)

### 🐛 Bug Fixes
- bug in swapped parameters [`721b4d9`](https://github.com/cladam/tbdflow/commit/721b4d9c01013e22634b75f3aed38a3aa153c9a7)

### 🔨 Code Refactoring
- implement clippy warnings [`6f138fe`](https://github.com/cladam/tbdflow/commit/6f138fed1e874188fdcb351b371a790a63db0a7c)
- remove duplicate code and added two helpers [`e855337`](https://github.com/cladam/tbdflow/commit/e85533763fa57aecea87ec167817ca25b6a797a7)
- remove dead code and renamed misc.rs to commands.rs for clarity [`36329b0`](https://github.com/cladam/tbdflow/commit/36329b0e2652599e13195743b0c0f12f4206aa42)
- remove a redundant statement [`12abe26`](https://github.com/cladam/tbdflow/commit/12abe26a719e9f456867472e35f01a01225932f7)
- remove unwrap in a regex matching snippet [`6c96070`](https://github.com/cladam/tbdflow/commit/6c960708f567be310960eea5a91947f3559498a7)
- add serde_json as dep and remove handrolled parser [`7f60e2b`](https://github.com/cladam/tbdflow/commit/7f60e2b6dea153ee82b92411c4a808365fec9b4a)
- moved parameters into a struct for cleanliness [`268d644`](https://github.com/cladam/tbdflow/commit/268d6447a98793ea53f0070d5c65b6e02f31a347)

### ⚙️ Maintenance
- **(release):** bump version and publish to GH Releases and crates.io [`4433ebf`](https://github.com/cladam/tbdflow/commit/4433ebff636c76669572b1f7e02643e859518a3b)


# [0.25.2](https://github.com/cladam/tbdflow/releases/tag/v0.25.2) (2026-04-11)

### ⚙️ Maintenance
- **(release):** bump version and publish to GH Releases and crates.io [`e1c8110`](https://github.com/cladam/tbdflow/commit/e1c811023e5db3fc5901a9d80702216dde4c78b0)
- fix small formatting error [`4344321`](https://github.com/cladam/tbdflow/commit/4344321868d48894651cab337e1e81d249bd9c87)
- update formatting in README [`49e1153`](https://github.com/cladam/tbdflow/commit/49e1153375c307f8fc8e65665bcd9baab0714825)
- add pre-commit hook to enforce cargo fmt [`057195d`](https://github.com/cladam/tbdflow/commit/057195d96022ca103ccfc6b12fa20f0d6b67dbeb)


# [0.25.1](https://github.com/cladam/tbdflow/releases/tag/v0.25.1) (2026-04-11)

### ✨ Features
- **(info):** add --edit flag and review, radar, ci-check sections to info command [`2830c21`](https://github.com/cladam/tbdflow/commit/2830c219ea3d39cbfa11a9dc5b8a1bbe240fd9e9)

### ⚙️ Maintenance
- **(release):** bump version and publish to GH Releases and crates.io [`3805b61`](https://github.com/cladam/tbdflow/commit/3805b616caa71edd81c625db247e338771e32812)
- **(release):** bump version and publish to GH Releases and crates.io [`6597d56`](https://github.com/cladam/tbdflow/commit/6597d5640fe4fdf2f9438509f54a30fffcf1b5b5)
- fix wording [`25a0298`](https://github.com/cladam/tbdflow/commit/25a02986ab3fb9e3279a269b989cacdd4f5349cf)
- update README with philosophy [`87ed94d`](https://github.com/cladam/tbdflow/commit/87ed94d60230b76de7ef07ce8f9f56d1024f1d6a)


# [0.24.1](https://github.com/cladam/tbdflow/releases/tag/v0.24.1) (2026-04-11)

### ⚙️ Maintenance
- **(release):** bump version and publish to GH Releases and crates.io [`7eccc3c`](https://github.com/cladam/tbdflow/commit/7eccc3c516a8543d459d8314ef7a2554cd5f371e)
- update README with review positional SHA and pre-flight CI check [`381197e`](https://github.com/cladam/tbdflow/commit/381197e0a7838814537a64ce5a59fc55d74e675f)


# [0.24.0](https://github.com/cladam/tbdflow/releases/tag/v0.24.0) (2026-04-11)

### ✨ Features
- **(sync):** add pre-flight CI status check to sync command [`6524fe8`](https://github.com/cladam/tbdflow/commit/6524fe83582892050bd07983cfaf751bf08d07c3)

### 🐛 Bug Fixes
- fix wording [`3bbfc35`](https://github.com/cladam/tbdflow/commit/3bbfc35c90a0e62611f17668baa43e63351d6919)
- **(radar):** fix choreo test parsing and tracked file detection [`94a593f`](https://github.com/cladam/tbdflow/commit/94a593f5d73e15339d6d44e449870e98b1f80f03)

### ⚙️ Maintenance
- **(release):** bump version and publish to GH Releases and crates.io [`1adcd91`](https://github.com/cladam/tbdflow/commit/1adcd91fbdc5ccaf36d12c4d8da47aced09b05f4)
- update README with pre-flight CI check [`90581f7`](https://github.com/cladam/tbdflow/commit/90581f7f74d273d40ad090c66f677e4e89fbed5e)
- update README with a problem statement [`87d5497`](https://github.com/cladam/tbdflow/commit/87d549795d4aea398c4cbd4f726eedad683397df)
- add BDD radar test using choreo [`fb5305d`](https://github.com/cladam/tbdflow/commit/fb5305d37e33602d17fc1992393e174304f227c8)
- add installation pre-flight check to agent and skill [`5b48952`](https://github.com/cladam/tbdflow/commit/5b48952f7f441ba5e82e8b93ab6e695fe3dbe45e)


# [0.23.0](https://github.com/cladam/tbdflow/releases/tag/v0.23.0) (2026-04-11)

### ✨ Features
- **(radar):** add overlap detection radar for social coding [`97111bb`](https://github.com/cladam/tbdflow/commit/97111bb41fdfc4b338a78b61de514f9d2a764189)

### 🐛 Bug Fixes
- **(radar):** use tbdflow commands in radar choreo test [`01d4304`](https://github.com/cladam/tbdflow/commit/01d430467345d186502f58eee6d9fe278be80ce5)

### 🔨 Code Refactoring
- **(tests):** use head-sha instead of git rev-parse or grepping output [`3dce239`](https://github.com/cladam/tbdflow/commit/3dce2398578b14a93f6186a383a6c71aa0904652)

### ⚙️ Maintenance
- **(release):** bump version and publish to GH Releases and crates.io [`22c4cf9`](https://github.com/cladam/tbdflow/commit/22c4cf91800af51bb22e3326c862ccbd25636411)
- **(radar):** add BDD choreo test for radar overlap detection [`431c919`](https://github.com/cladam/tbdflow/commit/431c919b4ed2a0a69d2bcc40b8ba6a1449532201)
- add radar command to README [`101bec4`](https://github.com/cladam/tbdflow/commit/101bec4dc417b2036455464bcc0d3f82c5e5b42b)
- add undo and radar to skill and agent docs [`2ffbb4d`](https://github.com/cladam/tbdflow/commit/2ffbb4db17c7d3b5b2c9bb2e4894cca2949c731a)


# [0.22.2](https://github.com/cladam/tbdflow/releases/tag/v0.22.2) (2026-04-11)

### ✨ Features
- add small command for getting head sha [`c9d653f`](https://github.com/cladam/tbdflow/commit/c9d653f05be0b7d0541aa5ce2ce7727bf5b64cca)

### 🔨 Code Refactoring
- **(tests):** removed git log command from chor file [`7532fbf`](https://github.com/cladam/tbdflow/commit/7532fbf15ef4b942b6213455d65b91e6a7086213)

### ⚙️ Maintenance
- **(release):** bump version and publish to GH Releases and crates.io [`1c8d086`](https://github.com/cladam/tbdflow/commit/1c8d086bb684fa150e6f6ebc0a549d505f32ded4)
- update README again after undo [`2bc310a`](https://github.com/cladam/tbdflow/commit/2bc310a35c13cbd341356615cf286067bca1be3a)


# [0.22.1](https://github.com/cladam/tbdflow/releases/tag/v0.22.1) (2026-04-11)

### ⚠️ BREAKING CHANGES
- **(api):** remove legacy endpoint [`6364a52`](https://github.com/cladam/tbdflow/commit/6364a5216898f401033eb4ddfad98145565676b1)
- **(api):** remove legacy endpoint [`84b3ed0`](https://github.com/cladam/tbdflow/commit/84b3ed05e86c168424cb410ecb3b1e91cf72e189)

### ✨ Features
- **(undo):** add undo command [`18893a6`](https://github.com/cladam/tbdflow/commit/18893a6ca6c09ec7851e4c96cdee9f9de1a3e45b)
- **(feedback):** add persona feedback report generator [`5b37d78`](https://github.com/cladam/tbdflow/commit/5b37d788ebdd0148f1c2dd5b46e70cda35235b41)
- validate feat type [`d11ff97`](https://github.com/cladam/tbdflow/commit/d11ff97ec3c79893ef658cf02a3ade90903e843c)
- validate feat type [`c01c664`](https://github.com/cladam/tbdflow/commit/c01c66429038eae1a9a6a747aa32192d0e6c51a0)
- validate feat type [`3d7ac28`](https://github.com/cladam/tbdflow/commit/3d7ac288b2c265c38744cb2a632e9ce558d9a90b)
- validate feat type [`308ee80`](https://github.com/cladam/tbdflow/commit/308ee808bd04d15d3364422cfb632f4ee710bca5)
- validate feat type [`a85a485`](https://github.com/cladam/tbdflow/commit/a85a485b7ad0bcf81625e4457e41ce62736fc38c)
- validate feat type [`a32b340`](https://github.com/cladam/tbdflow/commit/a32b34034d131ed31341c1ca2a9a19793f1a5df4)
- validate feat type [`899b93f`](https://github.com/cladam/tbdflow/commit/899b93f9495eb6a656ba01ba08120acefd4fc0a6)
- add feature a [`f2b7280`](https://github.com/cladam/tbdflow/commit/f2b7280708b725cb5e7660b9329259b144aa1476)
- validate feat type [`ea7a526`](https://github.com/cladam/tbdflow/commit/ea7a5267cf621382a5e7e5f4e925c2dbb05d0c93)
- **(api):** remove legacy endpoint [`6364a52`](https://github.com/cladam/tbdflow/commit/6364a5216898f401033eb4ddfad98145565676b1)
- add user greeting [`7921b85`](https://github.com/cladam/tbdflow/commit/7921b855dfdfd76e7ebdb1ef9a62fe2c7de128b4)
- validate feat type [`2dc2246`](https://github.com/cladam/tbdflow/commit/2dc22463e7782b78f2bf233d5e763f70fec2ab0e)
- add feature a [`edfa43b`](https://github.com/cladam/tbdflow/commit/edfa43be9df606561f271a0e748302e8d6308225)
- validate feat type [`5072e33`](https://github.com/cladam/tbdflow/commit/5072e33d07a7c1f3de98190b0a2dff59415e4be9)
- add user greeting [`a677dc6`](https://github.com/cladam/tbdflow/commit/a677dc6e135f61caba0790048b4e1afe10bd44ca)
- **(api):** remove legacy endpoint [`84b3ed0`](https://github.com/cladam/tbdflow/commit/84b3ed05e86c168424cb410ecb3b1e91cf72e189)
- add user greeting [`9c7adb1`](https://github.com/cladam/tbdflow/commit/9c7adb16b0442046309164558e29d25d73a296cf)

### 🐛 Bug Fixes
- add undo chor file to the CI suite [`bca002a`](https://github.com/cladam/tbdflow/commit/bca002a17c2921b94088d3916fe27ed4ef57eb79)
- is_ancestor_of now uses refs/heads/main instead of bare main [`93dc655`](https://github.com/cladam/tbdflow/commit/93dc6553a2408f2c36d9a5c78c3cc235ee303f2a)
- fix so undo only do pull --ff-only to preserve existing commit sha [`8426c5c`](https://github.com/cladam/tbdflow/commit/8426c5c22b02936272a106b9f817621def7e0363)
- fix so undo uses full hash in the bckground [`c264cb9`](https://github.com/cladam/tbdflow/commit/c264cb9b5b9d920b9d74c8bbe4b4a2e64fbc3ab7)
- validate fix type [`3326749`](https://github.com/cladam/tbdflow/commit/3326749b561bcfc39ffb99a5cdc1d5e22af36984)
- validate fix type [`265e986`](https://github.com/cladam/tbdflow/commit/265e98663b8f9d077d5113823593e9a4d44e8fe1)
- validate fix type [`d1b02e8`](https://github.com/cladam/tbdflow/commit/d1b02e8f869f2dde038231fb1ce0106282eecb17)
- validate fix type [`2f27b8b`](https://github.com/cladam/tbdflow/commit/2f27b8b9cc2671650752b9c9e7dfec62eeca0205)
- **(core):** resolve crash on startup [`992c677`](https://github.com/cladam/tbdflow/commit/992c6771d2114973b22eea0ef553c570c570d392)
- validate fix type [`f806d9e`](https://github.com/cladam/tbdflow/commit/f806d9e992b14472d2f110462995c765bb6a2a97)
- **(auth):** resolve token expiry [`d899ee0`](https://github.com/cladam/tbdflow/commit/d899ee00e5d6cf4302dd2f7e7e00646039c182b2)
- **(core):** resolve crash on startup [`7face46`](https://github.com/cladam/tbdflow/commit/7face46f94a2dea6a4719b162a5f2bbac6aab7e5)
- validate fix type [`d1dbf58`](https://github.com/cladam/tbdflow/commit/d1dbf5851894f48ed9cfb6466f1f33ed513f0924)
- **(auth):** resolve token expiry [`40d46a8`](https://github.com/cladam/tbdflow/commit/40d46a8693f78f62141d8ce75c0417771f3d0cdb)

### 🚀 Performance Improvements
- validate perf type [`477b901`](https://github.com/cladam/tbdflow/commit/477b9013f37438ecf1df4d65f239748128070724)
- validate perf type [`4fe6ae9`](https://github.com/cladam/tbdflow/commit/4fe6ae9cb68db34393d6ce7f204a2ebaba266067)
- validate perf type [`38be9f2`](https://github.com/cladam/tbdflow/commit/38be9f27a469cf75e387d57b057d425723fe9010)
- validate perf type [`5bf0673`](https://github.com/cladam/tbdflow/commit/5bf0673f1765165215fc382d504503383db1221d)
- validate perf type [`423db4a`](https://github.com/cladam/tbdflow/commit/423db4a2fc627dc89a33b8b9d4b36dbca1a06179)
- validate perf type [`42c8a6a`](https://github.com/cladam/tbdflow/commit/42c8a6a5a0addbde7bf71b392da9ada9f44b36ca)

### 🔨 Code Refactoring
- validate refactor type [`806cdaf`](https://github.com/cladam/tbdflow/commit/806cdaf3981e4f5d877d3f72c603b2d74d7b1997)
- validate refactor type [`37dfb75`](https://github.com/cladam/tbdflow/commit/37dfb75db1d99601382517ae99c4eec0917f045a)
- validate refactor type [`fa3e93b`](https://github.com/cladam/tbdflow/commit/fa3e93b02b355a4877fb735aaead3f0309e40124)
- validate refactor type [`328944e`](https://github.com/cladam/tbdflow/commit/328944ec0ceb56b603e7fea26486bf3bd34b6348)
- validate refactor type [`c69dacb`](https://github.com/cladam/tbdflow/commit/c69dacbfc35b0984116db44011c1e50d2a006554)
- validate refactor type [`f9a6561`](https://github.com/cladam/tbdflow/commit/f9a6561b113c1424481cf4d8fc782e73d031c526)

### ⚙️ Maintenance
- **(release):** bump version and publish to GH Releases and crates.io [`b94d167`](https://github.com/cladam/tbdflow/commit/b94d167ed983651a2cc000ec4906b1b698710ff7)
- **(release):** bump version and publish to GH Releases and crates.io [`1bf0384`](https://github.com/cladam/tbdflow/commit/1bf03845a6a3cdec3ddef2e7ed4bc21e4fb8543d)
- update README [`b11a19a`](https://github.com/cladam/tbdflow/commit/b11a19a6dd86308f02d1a5f92872a6838ab778b8)
- update README with latest undo command [`c50403d`](https://github.com/cladam/tbdflow/commit/c50403df3506d2c2c397942c3a76d227972d7956)
- **(explorations):** add persona-driven exploratory UX agents [`c12372b`](https://github.com/cladam/tbdflow/commit/c12372b94dce7448e534e727815697157c44cc06)
- **(personas):** add persona-driven UX feedback test suite Implements 'User Feedback as Code' using choreo persona simulations against the tbdflow CLI. Four personas, 37 tests total: - Nervous Newbie (13): help, dry-run, clear errors - TBD Purist (7): rapid commits, no prompts - Git-Flow Refugee (10): branch-commit-complete cycle - Monorepo Architect (7): scoped commits, changelog UX findings surfaced during development: - feature is valid branch type but not commit type - Default feature prefix is underscore not slash - tbdflow commit without flags fails gracefully w/o TTY Also adds reports/ to .gitignore. [`220505c`](https://github.com/cladam/tbdflow/commit/220505c3c136af34370ed9122a623a55c04fd0aa)
- install tbdflow and configure git for choreo BDD tests [`70ee5b6`](https://github.com/cladam/tbdflow/commit/70ee5b67fb86f6ec17d25e53df69e00af69b7152)
- **(choreo):** refactor BDD suite with four-layer model and task composition [`29c9925`](https://github.com/cladam/tbdflow/commit/29c992535403b8e51f6995d26803e41064dcb94e)
- validate style type [`dcabead`](https://github.com/cladam/tbdflow/commit/dcabead962d31ef590a372f23832a781674b6fca)
- validate ci type [`16c7e3e`](https://github.com/cladam/tbdflow/commit/16c7e3e8acd38cf24557f49eaff1ca1e1b59565c)
- validate build type [`0e9765d`](https://github.com/cladam/tbdflow/commit/0e9765d1b6aadf3a6b5f6b122674a480bd953821)
- validate test type [`23072a3`](https://github.com/cladam/tbdflow/commit/23072a3edddaf03c44e91ae0eb0f446c9649b7f5)
- validate docs type [`2c98b4e`](https://github.com/cladam/tbdflow/commit/2c98b4e9bc931031033912a39fe035bfcd72e952)
- validate chore type [`1797f96`](https://github.com/cladam/tbdflow/commit/1797f963b1e183a16c68bc324d0d7d9411ae3539)
- validate style type [`5308785`](https://github.com/cladam/tbdflow/commit/53087850801a041ea19524736af3a2802945cf1a)
- validate ci type [`388c107`](https://github.com/cladam/tbdflow/commit/388c10782cef1a8820344f43e656d9bdb82c6e1e)
- validate build type [`5f9359c`](https://github.com/cladam/tbdflow/commit/5f9359c89b814ece425028e75829584768e39070)
- validate test type [`e14a7d7`](https://github.com/cladam/tbdflow/commit/e14a7d7dfe6297e5748f5ddad1273b2516d0accd)
- validate docs type [`17af810`](https://github.com/cladam/tbdflow/commit/17af8104af6568906e613717066bc594e463dbca)
- validate chore type [`b8dd5b6`](https://github.com/cladam/tbdflow/commit/b8dd5b68fe3f608a4cb99945ca792bfbbc93de27)
- validate style type [`38a49bf`](https://github.com/cladam/tbdflow/commit/38a49bf7918b9874fca3f44fb066a79d17901417)
- validate ci type [`c8a12d7`](https://github.com/cladam/tbdflow/commit/c8a12d7daeb2fe5d5528cbb80388814f7aa80de3)
- validate build type [`f7a7243`](https://github.com/cladam/tbdflow/commit/f7a7243d8f7628868c71a671e5a3804591018522)
- validate test type [`5732b63`](https://github.com/cladam/tbdflow/commit/5732b63c649b0572ddf68442bcdd0e719944ea67)
- validate docs type [`8205389`](https://github.com/cladam/tbdflow/commit/8205389c701fb991f76ff8b66f8daa1a610fecc9)
- validate chore type [`621175d`](https://github.com/cladam/tbdflow/commit/621175d14108e8d1152a4e6386646705ec8e8126)
- validate style type [`a179e3c`](https://github.com/cladam/tbdflow/commit/a179e3ca511c5cd4b62dc9a6c4f40e1ba16823d0)
- validate ci type [`1265f46`](https://github.com/cladam/tbdflow/commit/1265f46d16f072775b728138b0907f2b8f23e051)
- validate build type [`fb2f4c5`](https://github.com/cladam/tbdflow/commit/fb2f4c5f18a087e5e7085357c565a0bb0a853124)
- validate test type [`03a8c01`](https://github.com/cladam/tbdflow/commit/03a8c01457504cccec9a5b311b1a376ea5c39a97)
- validate docs type [`960fb03`](https://github.com/cladam/tbdflow/commit/960fb03c33effa35d43b5a1cb8a81f8a05a01fbd)
- validate chore type [`202b265`](https://github.com/cladam/tbdflow/commit/202b265e21b4ff1d6c2dad06b9d580e7ba163851)
- update readme [`e315ccc`](https://github.com/cladam/tbdflow/commit/e315ccc87f7bc1f0901be546e983d378d29b099d)
- validate style type [`60e4f56`](https://github.com/cladam/tbdflow/commit/60e4f56cde5ee835786c5bc6c3e49c10f972d04a)
- validate ci type [`6aa7653`](https://github.com/cladam/tbdflow/commit/6aa7653758142f6ce60c47a0dc0787aa573916fb)
- validate build type [`306cc4a`](https://github.com/cladam/tbdflow/commit/306cc4aea1ed55eec6c2e7a2b6e6f70a209abfb0)
- validate test type [`d626c7e`](https://github.com/cladam/tbdflow/commit/d626c7e2d8580ab3c2ecdb182b83e8874ac008f7)
- validate docs type [`468b92a`](https://github.com/cladam/tbdflow/commit/468b92a2c4b4762cd30e83c8fe0943a2dc3828ca)
- validate chore type [`552d8dc`](https://github.com/cladam/tbdflow/commit/552d8dc0e40742c3a817c4f72ebe2ef29e5c48a2)
- update dependencies [`53f9e00`](https://github.com/cladam/tbdflow/commit/53f9e00ab3ae7fb9d09ca3d862d17c82bc6d6891)
- update readme [`3082fb9`](https://github.com/cladam/tbdflow/commit/3082fb9717eb45be7f7748354c107ebc2b23dad7)
- validate style type [`56ce533`](https://github.com/cladam/tbdflow/commit/56ce53312d1754c46dbd68e155ebfe70f5940130)
- validate ci type [`b4f8e80`](https://github.com/cladam/tbdflow/commit/b4f8e80fa35cc8f627641f3293e83f8cca89e214)
- validate build type [`d39bd9b`](https://github.com/cladam/tbdflow/commit/d39bd9bdb32c62f8f5fb2f674df7a12fe8595f3d)
- validate test type [`c70ef8c`](https://github.com/cladam/tbdflow/commit/c70ef8c9bcfbf1240f8b6e4be98328c80c576a8e)
- validate docs type [`e86dd5b`](https://github.com/cladam/tbdflow/commit/e86dd5ba067fb79733355e59a029fec0cc87991b)
- validate chore type [`a599f87`](https://github.com/cladam/tbdflow/commit/a599f87ee3274e30ea34998a91c17772374a63df)
- update dependencies [`c5d4ee8`](https://github.com/cladam/tbdflow/commit/c5d4ee83c4be9b2f7c7f2740b04559f164f42509)

### Miscellaneous
- validate revert type [`fd4760a`](https://github.com/cladam/tbdflow/commit/fd4760aa31989921f758bffacb5f75acbfa6070f)
- validate revert type [`6970a92`](https://github.com/cladam/tbdflow/commit/6970a9242df94956ebc091599dabe40c7ac509e4)
- validate revert type [`617dedf`](https://github.com/cladam/tbdflow/commit/617dedf9add3f7dc32d0036d3a8b7a606b8fcaf1)
- validate revert type [`96276ae`](https://github.com/cladam/tbdflow/commit/96276ae7da982d743bb7f897e8e6d5410691adb1)
- validate revert type [`fda7bf1`](https://github.com/cladam/tbdflow/commit/fda7bf1300352c58f5532b15e295486ec41dea7d)
- validate revert type [`8fef72b`](https://github.com/cladam/tbdflow/commit/8fef72b6e82ee2c4675405e52671d177a92fe9be)


# [0.21.0](https://github.com/cladam/tbdflow/releases/tag/v0.21.0) (2026-04-11)

### ✨ Features
- **(review):** add nuanced review labels and concern workflow [`ca14d97`](https://github.com/cladam/tbdflow/commit/ca14d970eb783525c987fd9e8d660754a9176e22)

### 🐛 Bug Fixes
- **(review):** remove CC @author from concern comments [`2e5fd46`](https://github.com/cladam/tbdflow/commit/2e5fd468095f1dc47c8f5c6500f8d235582e8be9)
- **(review):** use email local-part for GitHub username in concern mentions [`863a4ae`](https://github.com/cladam/tbdflow/commit/863a4aed243a21f42675c9e1de03f469fc8df169)

### ⚙️ Maintenance
- **(release):** bump version and publish to GH Releases and crates.io [`b5ba1f5`](https://github.com/cladam/tbdflow/commit/b5ba1f5f9be31e6cbdfb39eedfed9ab64ac5d3ae)


# [0.20.0](https://github.com/cladam/tbdflow/releases/tag/v0.20.0) (2026-04-11)

### ✨ Features
- **(review):** auto-trigger reviews when commit matches rule patterns [`fa887a6`](https://github.com/cladam/tbdflow/commit/fa887a6405a83c06e4324516b6564569a4c65fec)
- **(review):** add targeted review rules with file pattern matching [`308f7a1`](https://github.com/cladam/tbdflow/commit/308f7a11c115ed7555edda4c2dd1b57426b592e1)
- **(review):** add github-workflow strategy with server-side NBR support [`a473f48`](https://github.com/cladam/tbdflow/commit/a473f486ea8b7327507913d816cffb551f22d770)

### 🐛 Bug Fixes
- **(docs):** fix formatting [`5fd40ff`](https://github.com/cladam/tbdflow/commit/5fd40ff3af835b538ec866881bd1d3c514f20eef)
- remove old doc [`82d2c33`](https://github.com/cladam/tbdflow/commit/82d2c339aedf14e9ae45bd445c91670059eeab3b)
- remove old examples [`d4456d4`](https://github.com/cladam/tbdflow/commit/d4456d4d935728aea386b08508acda8588400c00)

### ⚙️ Maintenance
- **(release):** bump version and publish to GH Releases and crates.io [`bae7156`](https://github.com/cladam/tbdflow/commit/bae7156d50546f8c5a520d74f4c5632a00ac89f5)
- **(review):** add targeted review rules documentation to README [`25a0842`](https://github.com/cladam/tbdflow/commit/25a0842da66dd519b4eda99658bfdeed9873f882)
- add a section about CI in no_PRs document [`3744c82`](https://github.com/cladam/tbdflow/commit/3744c82aeb99fdf309d5258f5649ce289663daba)
- add tbdflow stance on PRs [`7282390`](https://github.com/cladam/tbdflow/commit/728239036d51737ba1cd2e21950bf9f50f67f633)
- **(changelog):** update CHANGELOG.md with latest releases [`e15edd8`](https://github.com/cladam/tbdflow/commit/e15edd81162207d15d8ddb961179c8ce0094efe0)


# [0.19.0](https://github.com/cladam/tbdflow/releases/tag/v0.19.0) (2026-02-17)

### ✨ Features
- **(review):** add non-blocking review command for TBD workflows [`caf4d78`](https://github.com/cladam/tbdflow/commit/caf4d78df59071dbb6bc8b492873efee6b7aa828)
- renamed files [`e7a0a87`](https://github.com/cladam/tbdflow/commit/e7a0a878d8bd9cbf938c1e24187e8cd9766cd302)

### 🐛 Bug Fixes
- formatting in AGENTS.md for Sync command [`fd1ae47`](https://github.com/cladam/tbdflow/commit/fd1ae47ab2e9ecf51cbb71d7947478d1367bf37c)

### ⚙️ Maintenance
- **(release):** bump version and publish to GH Releases and crates.io [`6fc8f1a`](https://github.com/cladam/tbdflow/commit/6fc8f1ac33e82c435b1680e82f6547e072273163)
- add review command section to README [`f69c334`](https://github.com/cladam/tbdflow/commit/f69c334dbb699f5e3540a798773157dc0ede2908)
- add tbdflow skill and code review devlog - Add tbdflow skill for Git workflow guidance - Add devlog entry for review.rs code quality analysis [`39003d9`](https://github.com/cladam/tbdflow/commit/39003d99ac3ff742e78c5b63b65bec80125ca302)
- add code-analyzer and rust-developer skills - Add code-analyzer skill for Rust code quality reviews - Add rust-developer skill with idiomatic Rust guidance - Both skills tailored for tbdflow CLI codebase [`93e142a`](https://github.com/cladam/tbdflow/commit/93e142a884f4a49e596f22ef14e8dc41223c4e10)
- rename agent files and add sync-first workflow [`7c7766f`](https://github.com/cladam/tbdflow/commit/7c7766f7f61efd11d0dd808dd4e26c37ff5af721)
- rename agent files and fix formatting [`0e819e2`](https://github.com/cladam/tbdflow/commit/0e819e216c77a9203387fd2760a5cafccc7f06d7)
- add 80 char line length constraint for body flag [`a439f8b`](https://github.com/cladam/tbdflow/commit/a439f8b8655cc7da2d67fa3627ed431deee56e2a)
- add body flag guidance to avoid newline issues [`bd81cd4`](https://github.com/cladam/tbdflow/commit/bd81cd4bbf45c1e27996cea892f589abfb7a5935)
- add agent and skill documentation [`a16148c`](https://github.com/cladam/tbdflow/commit/a16148c2d62f14c79d747f6f12582e12dfc2819f)
- align SKILL.md and AGENTS.md files [`ad60d0b`](https://github.com/cladam/tbdflow/commit/ad60d0bc7ebeaa132a50dc331e078a9524cf9698)
- add a AGENTS.md file for agentic usage of tbdflow [`eb74a16`](https://github.com/cladam/tbdflow/commit/eb74a16090aae69c9f5abfb16767543db4eaa4b4)
- update tbdflow skill with clearer instructions [`6292a4d`](https://github.com/cladam/tbdflow/commit/6292a4d7b2c3283ebf0ddde1ab42b177ab81f735)
- update tbdflow skill with clearer instructions [`7dcb48e`](https://github.com/cladam/tbdflow/commit/7dcb48e7b830b7b1471c64e514f12ef78b65a050)
- add a SKILL.md file for agentic usage of tbdflow [`67c7cd8`](https://github.com/cladam/tbdflow/commit/67c7cd81b789b4553ee6616bb357bfb7d776dde1)
- update README emphasising main [`5a438c9`](https://github.com/cladam/tbdflow/commit/5a438c9bbcc8b817a67b8ec2386cd2f1152bb22e)
- update dependencies to latest [`ef57ce4`](https://github.com/cladam/tbdflow/commit/ef57ce49e7e426045f0c8771532c1d90b5e545ac)


# [0.18.2](https://github.com/cladam/tbdflow/releases/tag/v0.18.2) (2026-02-17)

### ✨ Features
- add new info command that shows highlevel overview [`faa6c2d`](https://github.com/cladam/tbdflow/commit/faa6c2dfdfee9b251d18755c289b4fca198a4b68)
- update CHANGELOG.md and the command itself - changelog [`ea5db5a`](https://github.com/cladam/tbdflow/commit/ea5db5afa0dbbde6437977664e3e891bfdf3625d)

### ⚙️ Maintenance
- **(release):** bump version and publish to GH Releases and crates.io [`fa6cb87`](https://github.com/cladam/tbdflow/commit/fa6cb8704735fb8c8a74b69b909d8825f34202bc)
- update README with new info command [`4ea5620`](https://github.com/cladam/tbdflow/commit/4ea5620b851a89351aa692487785d0c4d7c221fe)
- **(release):** bump version and publish to GH Releases and crates.io [`5048ac3`](https://github.com/cladam/tbdflow/commit/5048ac305839a7a43a252237ce3ec82a32b533a2)


# [0.18.0](https://github.com/cladam/tbdflow/releases/tag/v0.18.0) (2025-09-16)

### ✨ Features
- **(changelog):** add a wizard for the changelog command [`155d130`](https://github.com/cladam/tbdflow/commit/155d130c441556bd6dea0fa3dbe30e332bb6d67c)
- **(complete):** add wizard for the complete command [`a9fbb5f`](https://github.com/cladam/tbdflow/commit/a9fbb5fd874d69e993d37314953b1df0cdd8145d)
- **(branch):** add wizard support for the branch command [`8b9e558`](https://github.com/cladam/tbdflow/commit/8b9e558edcaf20797027cd5878510ecb4f5d2e82)
- **(commit):** add a wizard mode for commit [`6c81539`](https://github.com/cladam/tbdflow/commit/6c81539468697d4bbb5bcb91372c274aabc5bea2)

### ⚙️ Maintenance
- **(release):** bump version and publish to GH Releases and crates.io [`afe683d`](https://github.com/cladam/tbdflow/commit/afe683d05789d2a5ed597315c9761d3c7cfdc54a)
- clean up dead code and fix imports [`b9313e6`](https://github.com/cladam/tbdflow/commit/b9313e69000fe42f8d0ed706eb17ce0d9fddb795)
- run cargo fmt on codebase [`de4d7dd`](https://github.com/cladam/tbdflow/commit/de4d7dd01c2515ec251408ea51585dea49b1e10a)


# [0.17.3](https://github.com/cladam/tbdflow/releases/tag/v0.17.3) (2025-09-16)

### ✨ Features
- new icon and new gif [`45b4f71`](https://github.com/cladam/tbdflow/commit/45b4f71bc54a0e19f082cbca2f050ee604a0b63e)
- **(plugin):** update commit to a side panel [`71162eb`](https://github.com/cladam/tbdflow/commit/71162eb71ea5567e91fe28c76e2e669283bb4884)

### 🐛 Bug Fixes
- update build and settings [`00c803c`](https://github.com/cladam/tbdflow/commit/00c803c8d6df4e0b30bff6a9c01b853051f2c2e0)
- **(plugin):** update plugin.xml, add icon  and cleanup [`9ee1932`](https://github.com/cladam/tbdflow/commit/9ee1932f157f65d4ca7a391f4b0c78378d9c2d6e)
- **(plugin):** update README and fix deprecated code [`6edbad9`](https://github.com/cladam/tbdflow/commit/6edbad97edf5de73ae7ca74993d91d32035a5318)

### ⚙️ Maintenance
- **(release):** bump version and publish to GH Releases and crates.io [`c93d381`](https://github.com/cladam/tbdflow/commit/c93d38116d6cb375ea422f276139dc0084e27c03)
- document IDE support [`4b98349`](https://github.com/cladam/tbdflow/commit/4b983497b27e4f42d361ac497e8df38c000c3e91)
- document IDE support [`c346d52`](https://github.com/cladam/tbdflow/commit/c346d524e01581cfaa88efd82008da2098f6bd88)
- exclude plugins folder for rust CI [`bbe8971`](https://github.com/cladam/tbdflow/commit/bbe8971c96e79da2b3f4da426194588b92715956)


# [0.17.2](https://github.com/cladam/tbdflow/releases/tag/v0.17.2) (2025-09-16)

### ✨ Features
- **(plugin):** update logic and add README [`f0ac93b`](https://github.com/cladam/tbdflow/commit/f0ac93b245bf750f4d78f107014df952f4621137)
- add config for the plugin [`908e823`](https://github.com/cladam/tbdflow/commit/908e823581beddde972641627e73295bf1ee7375)
- **(plugin):** change response dialogue and add changelog [`44a90d8`](https://github.com/cladam/tbdflow/commit/44a90d86287ca9fb2290644a10cbc4d75cc2a9ca)
- **(plugin):** add working commit command [`1f7e60a`](https://github.com/cladam/tbdflow/commit/1f7e60ab7aede96cfea0962e31552025372c155b)
- **(plugin):** add utility commands [`3cf7029`](https://github.com/cladam/tbdflow/commit/3cf7029a146369f9752a7eaccaf7c9e380ec8f36)
- **(plugin):** add working sync command [`d1c2cd0`](https://github.com/cladam/tbdflow/commit/d1c2cd0307250e381326fe3c5f2dd0847e093174)

### 🐛 Bug Fixes
- update logic for status and sync command [`313b352`](https://github.com/cladam/tbdflow/commit/313b352576c20666a455e5b8be92c30d580e4122)
- add comment [`fba5792`](https://github.com/cladam/tbdflow/commit/fba5792b3a5c7e78d9dd301a8ed26bdb4240fceb)
- update CHANGELOG with fix release [`35b84b8`](https://github.com/cladam/tbdflow/commit/35b84b80ee98bf74497b94d8f53556f40a3f6411)

### ⚙️ Maintenance
- **(release):** bump version and publish to GH Releases and crates.io [`42fba2b`](https://github.com/cladam/tbdflow/commit/42fba2b1ef74ca5866ef970a2011bd1f48a5f442)


# [0.17.1](https://github.com/cladam/tbdflow/releases/tag/v0.17.1) (2025-08-29)

### 🐛 Bug Fixes
- update CHANGELOG and failing test [`2e4b5b1`](https://github.com/cladam/tbdflow/commit/2e4b5b1ec9f10308f5f85a17c1a7f30062cd7e3e)
- update README with monorepo --include-projects flag [`91e46c8`](https://github.com/cladam/tbdflow/commit/91e46c857b8dbf7d74cac39e3dd636599b70e648)

### ⚙️ Maintenance
- **(release):** bump version and publish to GH Releases and crates.io [`8754325`](https://github.com/cladam/tbdflow/commit/8754325cba7f578397e36cf100ccb9d1f530d90b)


# [0.17.0](https://github.com/cladam/tbdflow/releases/tag/v0.17.0) (2025-08-29)

### ✨ Features
- **(monorepo):** add an escape hatch --include-projects [`397cbec`](https://github.com/cladam/tbdflow/commit/397cbeca43218e6020d9f33807b418ffafd2d991)
- **(plugin):** add initial project for the IntelliJ plugin [`63df804`](https://github.com/cladam/tbdflow/commit/63df804a3df804a0f16b0517817fcba84234588b)

### ⚙️ Maintenance
- **(release):** bump version and publish to GH Releases and crates.io [`dd11a1a`](https://github.com/cladam/tbdflow/commit/dd11a1a3106426b8b9dc6b5b23b720a58a5cd8b6)


# [0.16.1](https://github.com/cladam/tbdflow/releases/tag/v0.16.1) (2025-08-29)

### 🐛 Bug Fixes
- improve status and sync, and add CHANGELOG [`0aceb0f`](https://github.com/cladam/tbdflow/commit/0aceb0f37041ccbd95d031fb0d6961f75017fd17)
- improve status and sync, and add CHANGELOG [`f47f5f2`](https://github.com/cladam/tbdflow/commit/f47f5f20ff2875fa1c91c420534c07e5e695e634)

### ⚙️ Maintenance
- **(release):** bump version and publish to GH Releases and crates.io [`4cb0598`](https://github.com/cladam/tbdflow/commit/4cb0598cd1c3753fa2e408385cd0bdc9a14fba22)


# [0.16.0](https://github.com/cladam/tbdflow/releases/tag/v0.16.0) (2025-08-29)

### ✨ Features
- **(monorepo):** make init, commit, sync and status monorepo aware [`e3f60f9`](https://github.com/cladam/tbdflow/commit/e3f60f984cb303c986b0f30592263af54cfc86f1)

### 🐛 Bug Fixes
- update README [`0cc09a2`](https://github.com/cladam/tbdflow/commit/0cc09a27c784ef29f4a6bfeda524f56a5d3942fc)
- update README with monorepo instructions [`d94cecd`](https://github.com/cladam/tbdflow/commit/d94cecd18f415dfd36548fcf6afac90dfe915dd0)
- update README with monorepo instructions [`ded6dec`](https://github.com/cladam/tbdflow/commit/ded6dec023651712534d3a609dea900f5c10ed36)
- update README with monorepo support [`fcccb5f`](https://github.com/cladam/tbdflow/commit/fcccb5f39d0f44fd0a4d9bf82dfda5574cea4896)

### 🔨 Code Refactoring
- remove deprecated code [`f943a95`](https://github.com/cladam/tbdflow/commit/f943a95d59ee5bcc10297a10484277d10707b8e7)
- **(tests):** use new branch command in tests [`2f62b7e`](https://github.com/cladam/tbdflow/commit/2f62b7e8944ac67e197dc57ac28fa8fc4793ef9d)
- **(monorepo):** add a small helper method [`0491fa4`](https://github.com/cladam/tbdflow/commit/0491fa43bac7f638d9cdfb98b9e8314a03a36f9f)

### ⚙️ Maintenance
- **(release):** bump version and publish to GH Releases and crates.io [`022db8c`](https://github.com/cladam/tbdflow/commit/022db8c63bd90840e61dc2ed72964879ec0a1793)
- remove obsolete init branch setup [`75f334f`](https://github.com/cladam/tbdflow/commit/75f334fe4b467ab626d63d6fd05d6983978140dd)


# [0.15.2](https://github.com/cladam/tbdflow/releases/tag/v0.15.2) (2025-08-28)

### 🐛 Bug Fixes
- update header in README, we got a logo! [`fb0dd60`](https://github.com/cladam/tbdflow/commit/fb0dd6085c60aeebb799d9cf12f6e136536dda85)
- **(tests):** use main branch in tests [`dbf3ca8`](https://github.com/cladam/tbdflow/commit/dbf3ca8beafa44338b68dfe8554a2f3bcd370082)
- revert last merge [`c082174`](https://github.com/cladam/tbdflow/commit/c0821748ae073449f1a632445f5ae4025912330e)
- **(tests):** make create branch test detect default init branch name [`3807877`](https://github.com/cladam/tbdflow/commit/3807877dee2e96c2b0e0cd9238bb6ad09876da82)

### ⚙️ Maintenance
- **(release):** bump version and publish to GH Releases and crates.io [`412878f`](https://github.com/cladam/tbdflow/commit/412878f8ff95d940006df35c464996cfbcbfbf56)
- **(fmt):** fix fmt [`1200f0d`](https://github.com/cladam/tbdflow/commit/1200f0d9409c0d2013a1e5384ed25776cbfd671a)
- update CHANGELOG with latest changes [`7ef473f`](https://github.com/cladam/tbdflow/commit/7ef473f4a08acede4dcf98b469b9c82c6e6b1920)


# [0.15.1](https://github.com/cladam/tbdflow/releases/tag/v0.15.1) (2025-08-25)

### 🐛 Bug Fixes
- **(cli):** add --from_commit option to branch command [`c15bcc9`](https://github.com/cladam/tbdflow/commit/c15bcc998c6b4203022ae5d7b8eef4413f7a801d)

### ⚙️ Maintenance
- **(release):** bump version and publish to GH Releases and crates.io [`5cd1bd0`](https://github.com/cladam/tbdflow/commit/5cd1bd07ff0446211dac135ad586844e42347aed)
- update example in README [`65e5440`](https://github.com/cladam/tbdflow/commit/65e5440f32b4ebe6328e1771778347f0bb338646)
- update README with branch --from_commit [`b16e8bb`](https://github.com/cladam/tbdflow/commit/b16e8bb999e7ec46b20642d956a2ab3f8cc50c51)
- update CHANGELOG with latest changes [`5a36c9f`](https://github.com/cladam/tbdflow/commit/5a36c9fd1130dd8dfb0ffb473a642b16c582f4ab)


# [0.15.0](https://github.com/cladam/tbdflow/releases/tag/v0.15.0) (2025-08-22)

### ⚠️ BREAKING CHANGES
- **(branch):** add new branch command [`489ee8a`](https://github.com/cladam/tbdflow/commit/489ee8ae263c1ccd481989682a9c3f2169cce9ac)

### ✨ Features
- **(dryrun):** add global --dry-run flag [`3a186b2`](https://github.com/cladam/tbdflow/commit/3a186b240afd760dab459426cd6f91c10bb22432)
- check for conflicting flags when issu handling strategy is commit-scope [`cca7cb8`](https://github.com/cladam/tbdflow/commit/cca7cb8876986cfe793148f9722fa54295837f21)
- **(branch):** add new branch command [`489ee8a`](https://github.com/cladam/tbdflow/commit/489ee8ae263c1ccd481989682a9c3f2169cce9ac)

### 🐛 Bug Fixes
- **(branch):** fix failing test, prefix was set wrong [`0013fd7`](https://github.com/cladam/tbdflow/commit/0013fd7b7ca5e75948f2773d0da31d3d365f4e23)
- **(branch):** fix failing test due to backward compability of release/feature/hotfix [`2dc5d48`](https://github.com/cladam/tbdflow/commit/2dc5d48d8d588d920c0729c3042d1116d7b118f3)
- **(branch):** small fix in main [`40c3f38`](https://github.com/cladam/tbdflow/commit/40c3f3869c9a59d6c81dcd25316fb2ae9624bbcd)
- **(branch):** update logic of the command [`c1a85cc`](https://github.com/cladam/tbdflow/commit/c1a85cc7fc9f36239eb95dd31f6f31caa870f658)

### 🔨 Code Refactoring
- move branch type and error handling to misc, remove duplication [`50daade`](https://github.com/cladam/tbdflow/commit/50daade01c991348db213af35f3836ecf61c84be)

### ⚙️ Maintenance
- **(release):** bump version and publish to GH Releases and crates.io [`0cb1794`](https://github.com/cladam/tbdflow/commit/0cb1794dd2b9d575f8cd35bd4bbcaae5c295fa85)
- update README markdown [`a3906d0`](https://github.com/cladam/tbdflow/commit/a3906d06e5354e374256aca7b55a5718a9ef0b91)
- clean up README [`995ccf4`](https://github.com/cladam/tbdflow/commit/995ccf481cc9591b73bcfcfac8b4aac1e99c2489)
- update README with branch command, and dry-run [`bba72e8`](https://github.com/cladam/tbdflow/commit/bba72e804f1086b8755edf782460e5e1b4831d47)
- **(branch):** add testcase for the new branch command [`652dd51`](https://github.com/cladam/tbdflow/commit/652dd5156c253202380d4ab2959785885ed2beec)
- **(fmt):** run cargo fmt on the code base [`a5c936d`](https://github.com/cladam/tbdflow/commit/a5c936db45be3ace7cb6e8c957c9d29caf0c21af)
- update config with my username and repo [`d62b88e`](https://github.com/cladam/tbdflow/commit/d62b88ef7f3d2a488a8bf9c637e3dbf6076a0491)
- update broken link in CHANGELOG.md [`12f4579`](https://github.com/cladam/tbdflow/commit/12f4579c85cef274852bf440280d981c028e202d)
- update CHANGELOG.md [`a8955d0`](https://github.com/cladam/tbdflow/commit/a8955d0d5e1c26745e399d3b659240b071c4388b)
- generate a new changelog entry for v0.14.0 [`b0434d3`](https://github.com/cladam/tbdflow/commit/b0434d3aedb978440802043bc942059b0a8dbf1a)
- add cargo fmt check to CI [`5f2e2fc`](https://github.com/cladam/tbdflow/commit/5f2e2fc56bcf28fa8af2dd8e6e70f04d29882300)


# [0.14.0](https://github.com/cladam/tbdflow/releases/tag/v0.14.0) (2025-08-20)

### ✨ Features
- **(changelog):** add CHANGELOG.md based on changelog command and GH Releases [`bdcdd01`](https://github.com/cladam/tbdflow/commit/bdcdd01f8e37fea534982a994c36ae59fe107fe9)
- **(changelog):** update heading for unreleased changes [`7b4b4b6`](https://github.com/cladam/tbdflow/commit/7b4b4b648bb4e42097570edc91ddd4fe4a019d41)
- **(changelog):** update heading with link from .tbdflow.yml [`68fbdc9`](https://github.com/cladam/tbdflow/commit/68fbdc998f4f35c7202b0664dd12fe37243c75f2)
- **(changelog):** update output format och changelog [`11b1cb2`](https://github.com/cladam/tbdflow/commit/11b1cb2ee770969ac2be7e94f485145a278ef9db)
- **(changelog):** add initial changelog command [`5a8d7c7`](https://github.com/cladam/tbdflow/commit/5a8d7c7dd36499ef7630318fb2996aec1a5e37d0)

### ⚙️ Maintenance
- **(release):** bump version and publish to GH Releases and crates.io [`83044c0`](https://github.com/cladam/tbdflow/commit/83044c038b83ea9909c37527a535c55d7baaa1c5)
- update README with changelog documentation [`1d5a0cc`](https://github.com/cladam/tbdflow/commit/1d5a0cc20d568be0a6980ec6918cfbb29e30c6a6)
- update README [`40ab254`](https://github.com/cladam/tbdflow/commit/40ab254e2f42a3627c2d5c23f191e7ee2182b494)


# [0.13.1](https://github.com/cladam/tbdflow/releases/tag/v0.13.1)

### ⚠️ BREAKING CHANGES
- **(linting):** add new linting rules and update tbdflow config [`282f930`](https://github.com/cladam/tbdflow/commit/282f9309ac202c7242f959904198ee9b8bb43c9a)

### ✨ Features
- **(linting):** add new linting rules and update tbdflow config [`282f930`](https://github.com/cladam/tbdflow/commit/282f9309ac202c7242f959904198ee9b8bb43c9a)

### 🐛 Bug Fixes
- uncomment failing linting code, already checking leading blank [`c4b5af8`](https://github.com/cladam/tbdflow/commit/c4b5af8cc7a51945cf63af153cd4ba529986cb8c)

### ⚙️ Maintenance
- **(release):** bump version and publish to GH Releases and crates.io [`c96bd40`](https://github.com/cladam/tbdflow/commit/c96bd4034dcba271dfcfb3afbdc4510e6c4d3f7f)
- update README [`bdb0758`](https://github.com/cladam/tbdflow/commit/bdb0758898372833d35023170a0a2f247b429219)
- Initialise tbdflow configuration [`3ec2354`](https://github.com/cladam/tbdflow/commit/3ec23541857386f02a4be3fa311457e42f82a78e)
- improve git output assertion [`e0657ef`](https://github.com/cladam/tbdflow/commit/e0657eff04ef02b6091e9b755126ca3c14ad75b1)


# [0.13.0](https://github.com/cladam/tbdflow/releases/tag/v0.13.0)

### 🐛 Bug Fixes
- remove old sub dir [`eaddc2c`](https://github.com/cladam/tbdflow/commit/eaddc2cfbe043538e498a5d438d5dfc731460c2d)
- update CI script to run from root dir [`6c8b0ec`](https://github.com/cladam/tbdflow/commit/6c8b0ec6cc4d95d5ae6aa5d57be0f95ab371b836)

### ⚙️ Maintenance
- **(release):** bump version and publish to GH Releases and crates.io [`428628c`](https://github.com/cladam/tbdflow/commit/428628c394cce08017e6e2a434799f900e8d198a)
- moving dot files from sub-dir [`af92896`](https://github.com/cladam/tbdflow/commit/af928966555733ebde9833670ba18526ce004ec2)
- move files from sub-dir to root [`645b085`](https://github.com/cladam/tbdflow/commit/645b0850fa8f8f3c785fa966cf38fc0a0c516654)
- update README [`c10b8a4`](https://github.com/cladam/tbdflow/commit/c10b8a46cff8b49f93182ca191fcc3eeea1d163b)
- removing F# code and adding a gif of commit commands [`b70d7ab`](https://github.com/cladam/tbdflow/commit/b70d7ab8c5e0e7d10fb81e9afa5f382fd2b9f440)


# [0.12.3](https://github.com/cladam/tbdflow/releases/tag/v0.12.3)

### 🐛 Bug Fixes
- tell clap to use --breaking-description flag [`c8a897f`](https://github.com/cladam/tbdflow/commit/c8a897f493cff612d7de494a852ca2294b538d80)

### ⚙️ Maintenance
- **(release):** bump version and publish to GH Releases and crates.io [`17c9cd9`](https://github.com/cladam/tbdflow/commit/17c9cd9cdf2062fd81dda4dafb27dffd53d30f5e)


# [0.12.2](https://github.com/cladam/tbdflow/releases/tag/v0.12.2)

### 🔨 Code Refactoring
- **(commit):** moved commit handler to seperate file to keep main clean-er [`8975588`](https://github.com/cladam/tbdflow/commit/89755887be8322a6cd0739772731432278098a47)

### ⚙️ Maintenance
- **(release):** bump version and publish to GH Releases and crates.io [`63e2b70`](https://github.com/cladam/tbdflow/commit/63e2b70e42090037547c722a6eeae9ce26d88662)


# [0.12.1](https://github.com/cladam/tbdflow/releases/tag/v0.12.1)

### 🔨 Code Refactoring
- updated init command to handle populated remote repo [`87e9e5e`](https://github.com/cladam/tbdflow/commit/87e9e5ee25798f677f552506dfcef4cfa25247df)

### ⚙️ Maintenance
- **(release):** bump version and publish to GH Releases and crates.io [`6f9ae6c`](https://github.com/cladam/tbdflow/commit/6f9ae6c62285fea83d0b6f15ae603d9a3459b68b)


# [0.12.0](https://github.com/cladam/tbdflow/releases/tag/v0.12.0)

### 🐛 Bug Fixes
- update README.md [`a23067f`](https://github.com/cladam/tbdflow/commit/a23067fae5bcf62a699e4db3947671aee496f934)

### 🔨 Code Refactoring
- updated init command and placed functionality in seperate file [`998fbb7`](https://github.com/cladam/tbdflow/commit/998fbb77ad6d0107df1deb6ab9c802874793970b)

### ⚙️ Maintenance
- **(release):** bump version and publish to GH Releases and crates.io [`d8d7924`](https://github.com/cladam/tbdflow/commit/d8d79248344a042af27ef0dfbe852e04acf7eee7)


# [0.11.2](https://github.com/cladam/tbdflow/releases/tag/v0.11.2)

### 🐛 Bug Fixes
- update CI.yml [`6870431`](https://github.com/cladam/tbdflow/commit/687043121ce1068b9aa7f97c183490ab5061da11)
- update CI.yml [`1854521`](https://github.com/cladam/tbdflow/commit/1854521f0d60a8cc2da1a45e40e6c3c04e013a7e)
- update CI.yml [`eb01e86`](https://github.com/cladam/tbdflow/commit/eb01e8608459e4a1cd2c7e8e1fd01240619665f7)
- update CI.yml [`97657c9`](https://github.com/cladam/tbdflow/commit/97657c9d2c2c14d5a237d30e7608b0663e21d9b5)
- update CI.yml [`8715fac`](https://github.com/cladam/tbdflow/commit/8715fac3a6d2db910a7cee0e39998302ecc19338)

### ⚙️ Maintenance
- **(release):** bump version and publish to GH Releases and crates.io [`ee5a3ae`](https://github.com/cladam/tbdflow/commit/ee5a3ae1a2579f8b365bd5afb127b62e611ea44d)
- **(release):** bump version and publish to GH Releases and crates.io [`f46eba8`](https://github.com/cladam/tbdflow/commit/f46eba838ec8cfad917db1dc65b4f9e6bdc3a41f)
- **(release):** bump version and publish to GH Releases and crates.io [`094675f`](https://github.com/cladam/tbdflow/commit/094675f77983fe0ce2a4d0c9b27b840170551715)
- **(release):** bump version and publish to GH Releases and crates.io [`34aca7c`](https://github.com/cladam/tbdflow/commit/34aca7c1dc5babd7b32789a8c8f1c66dc9047a9e)
- add tempdir to all functions [`70e3a35`](https://github.com/cladam/tbdflow/commit/70e3a353150d5ec586e8a46bd064af2cc70b0038)
- add serial to every test, I have a race condition somewhere [`192a824`](https://github.com/cladam/tbdflow/commit/192a82422d44c464d855a778ecd77a5b2a8c1690)
- fix test failing due to missing upstream [`a3d6bd7`](https://github.com/cladam/tbdflow/commit/a3d6bd7a2ddf56a51acbbb8e606a9efc0af6659a)
- describe crates.io installation and updates [`51dace2`](https://github.com/cladam/tbdflow/commit/51dace21001c3ed73e0ff1edef343085843da7ce)


# [0.11.1](https://github.com/cladam/tbdflow/releases/tag/v0.11.1)

### ⚙️ Maintenance
- **(release):** bump version and publish to GH Releases and crates.io [`33f1bd2`](https://github.com/cladam/tbdflow/commit/33f1bd2d943e5963369c94f740e9824fb145c65d)
- update comments in code [`50b6519`](https://github.com/cladam/tbdflow/commit/50b65191d0e9e4927874dc303d3db09fced77e2b)
- add CONTRIBUTING.md file [`6b48e36`](https://github.com/cladam/tbdflow/commit/6b48e3660b7f8c4e10fea98b391753c415c788f6)
- update README [`f2c3485`](https://github.com/cladam/tbdflow/commit/f2c3485b65cfe63a772cf5e8730586abcdcb85b5)


# [0.11.0](https://github.com/cladam/tbdflow/releases/tag/v0.11.0)

### ⚙️ Maintenance
- **(release):** Bump version and publish to GH and crates.io [`d31c7e4`](https://github.com/cladam/tbdflow/commit/d31c7e4047228cf7ef2da0abd57638f92490670e)


# [0.10.12](https://github.com/cladam/tbdflow/releases/tag/v0.10.12)

### ⚙️ Maintenance
- **(release):** Bump version and publish to GH and crates.io [`00eb563`](https://github.com/cladam/tbdflow/commit/00eb5638bf6f5b5d76cc8c88808be91a5f9acadb)
- Fixed failing test, bc of linting [`2f69aa8`](https://github.com/cladam/tbdflow/commit/2f69aa894a3c645162d24a1811f975b03e8b665b)


# [0.10.11](https://github.com/cladam/tbdflow/releases/tag/v0.10.11)

### 🐛 Bug Fixes
- **(linting):** Corrected logic for subject_line_not_capitalized [`1f66ca4`](https://github.com/cladam/tbdflow/commit/1f66ca41e13b42179f5fe768fa498d6f48133336)

### ⚙️ Maintenance
- **(release):** Bump version and publish to GH and crates.io [`c0f36ab`](https://github.com/cladam/tbdflow/commit/c0f36abeda1d388ee3088b26836fbf2fa22ea886)


# [0.10.10](https://github.com/cladam/tbdflow/releases/tag/v0.10.10)

### 🐛 Bug Fixes
- Protect main, and print friendlier error message if someone tries [`3bb66f9`](https://github.com/cladam/tbdflow/commit/3bb66f9efcc85e3b7c972c9cd1c6b6bf3eb46325)

### ⚙️ Maintenance
- **(release):** Bump version and publish to GH and crates.io [`6c4eb94`](https://github.com/cladam/tbdflow/commit/6c4eb946084a5c13b49f47f1e633c133547a9e27)


# [0.10.9](https://github.com/cladam/tbdflow/releases/tag/v0.10.9)

### 🐛 Bug Fixes
- Handle mixed case in branch names [`f142254`](https://github.com/cladam/tbdflow/commit/f142254177e16692f52ddca85f0d02367b66b53a)

### ⚙️ Maintenance
- **(release):** Bump version and publish to GH and crates.io [`207c50e`](https://github.com/cladam/tbdflow/commit/207c50e6021db1a2d5c600f47ed69bcda040fcef)


# [0.10.8](https://github.com/cladam/tbdflow/releases/tag/v0.10.8)

### 🐛 Bug Fixes
- Hide some prints behind verbose flag [`ac800df`](https://github.com/cladam/tbdflow/commit/ac800df442b783c3996c000cd09db4999fac6699)

### ⚙️ Maintenance
- **(release):** Bump version and publish to GH and crates.io [`fd4e651`](https://github.com/cladam/tbdflow/commit/fd4e65139ba81348b6b553a35acf0557cc44498c)


# [0.10.7](https://github.com/cladam/tbdflow/releases/tag/v0.10.7)

### 🐛 Bug Fixes
- **(complete):** Update complete command with a pre-flight check to see if a tag exists already [`d11bc56`](https://github.com/cladam/tbdflow/commit/d11bc567e4f07f6fa789c440eac100d912effc13)
- **(complete):** Update complete command with a pre-flight check for local branch [`88cc132`](https://github.com/cladam/tbdflow/commit/88cc132295598153531063aaf7fd943e5b1c3487)

### ⚙️ Maintenance
- **(cargo):** Bump version and publish [`58e9eb1`](https://github.com/cladam/tbdflow/commit/58e9eb1cd7e76303535874b085596920940425c3)
- **(cargo):** Bump version and publish [`64ce2bb`](https://github.com/cladam/tbdflow/commit/64ce2bbb55fbbbe7fe6d44a4604801706c282673)


# [0.10.6](https://github.com/cladam/tbdflow/releases/tag/v0.10.6)

### 🐛 Bug Fixes
- **(complete):** Update complete command with a pre-flight check for local branch [`88cc132`](https://github.com/cladam/tbdflow/commit/88cc132295598153531063aaf7fd943e5b1c3487)

### ⚙️ Maintenance
- **(cargo):** Bump version and publish [`64ce2bb`](https://github.com/cladam/tbdflow/commit/64ce2bbb55fbbbe7fe6d44a4604801706c282673)


# [0.10.5](https://github.com/cladam/tbdflow/releases/tag/v0.10.5)

### 🐛 Bug Fixes
- **(sync):** Update sync and complete commands [`63f50df`](https://github.com/cladam/tbdflow/commit/63f50df411237d3ccc91c13420dc0e5e94d979d7)

### ⚙️ Maintenance
- **(cargo):** Bump version and publish [`c24d70c`](https://github.com/cladam/tbdflow/commit/c24d70ca0577b5574fc8f831980cd3cdc0dfec25)


# [0.10.4](https://github.com/cladam/tbdflow/releases/tag/v0.10.4)

### 🐛 Bug Fixes
- **(sync):** Update sync to handle branches [`cb8d763`](https://github.com/cladam/tbdflow/commit/cb8d7632b289ae08e95a4ec639b7ce285000c73c)

### ⚙️ Maintenance
- **(cargo):** Bump version and publish [`f92a786`](https://github.com/cladam/tbdflow/commit/f92a7862cb9a3d9d7beb51667abe7953a1c1501a)


# [0.10.3](https://github.com/cladam/tbdflow/releases/tag/v0.10.3)

### 🐛 Bug Fixes
- **(completion):** Fixed a bug in shell completion, the print statement from my command messes with the shells [`cc3c147`](https://github.com/cladam/tbdflow/commit/cc3c147beba6f2d9797bbf804dda35c7ebaec362)

### ⚙️ Maintenance
- **(cargo):** Bump version and publish [`25b6d1d`](https://github.com/cladam/tbdflow/commit/25b6d1d102a5b159ab45375086109be61a336574)


# [0.10.2](https://github.com/cladam/tbdflow/releases/tag/v0.10.2)

### ✨ Features
- **(update):** Add self_update in new update command [`5b0c0cd`](https://github.com/cladam/tbdflow/commit/5b0c0cdc9efda16f81d1df18ab0023b958594308)
- Add completion using clap_completion as a hidden command [`ca93241`](https://github.com/cladam/tbdflow/commit/ca932410d7b42b0b9d6994e6259088f8083ae282)
- **(linting):** Update README with default rules for linting [`8f2b7b9`](https://github.com/cladam/tbdflow/commit/8f2b7b99fe1cb513790adda39b97dd585994d5d3)

### 🐛 Bug Fixes
- **(completion):** Fixed a bug in shell completion, do not look for a git repo in (~) [`21b5646`](https://github.com/cladam/tbdflow/commit/21b5646d883109a249995e8d0503d20c8ff22e40)
- **(status):** Check if current dir is a git repo before any other operation [`e662cfb`](https://github.com/cladam/tbdflow/commit/e662cfbc188bfd69f71958c948bb7fab3cf658ef)
- **(commit):** Check for staged changes before trying to commit [`80f401a`](https://github.com/cladam/tbdflow/commit/80f401a0934300be233241a7560e3fb441a47c7b)
- **(init):** Fixing a locical error in init, need to commit locally first [`185c536`](https://github.com/cladam/tbdflow/commit/185c5364923c01f60086ca93cf5eebd174f81eda)

### ⚙️ Maintenance
- **(cargo):** Bump version and publish [`933ca50`](https://github.com/cladam/tbdflow/commit/933ca500e1b771826a8d3a18a510bc5d97bf43a4)
- **(cargo):** Bump version and publish [`b8967fc`](https://github.com/cladam/tbdflow/commit/b8967fcddcd728706ac7b53d3b24144c78199747)
- **(cargo):** Bump version and publish [`e0596da`](https://github.com/cladam/tbdflow/commit/e0596dad5c041aeda78a093a083c94a3e52ea314)
- Update README with update command [`a26edee`](https://github.com/cladam/tbdflow/commit/a26edee92445de5652550cd02a3063c8cd5ab424)
- **(cargo):** Bump version and publish [`d5dbe36`](https://github.com/cladam/tbdflow/commit/d5dbe3650a38df3029318d7a38046475986b248d)
- **(cargo):** Bump version and publish [`a86999e`](https://github.com/cladam/tbdflow/commit/a86999ec09e9f82bb9c13f514279b138dd7f1b53)
- **(cargo):** Bump version and publish [`406a289`](https://github.com/cladam/tbdflow/commit/406a28924ebe93778deed7155a5d75027df9d8b5)
- Update README with advanced usage [`9f68451`](https://github.com/cladam/tbdflow/commit/9f68451380dd87654afe1541d5d68551be95175d)
- **(cargo):** Bump version and publish [`da9a879`](https://github.com/cladam/tbdflow/commit/da9a879ed7a30aee15acb4bf3b3278e045ec195f)
- **(cargo):** Bump version and publish [`af1868d`](https://github.com/cladam/tbdflow/commit/af1868d036e853464cf3be6a884a219b1f91c699)


# [0.9.0](https://github.com/cladam/tbdflow/releases/tag/v0.9.0)

### ✨ Features
- **(linting):** Implement all rules and updated documentation [`07ddfb1`](https://github.com/cladam/tbdflow/commit/07ddfb1c4eb76fee83f91dbc45f150bf5e972c02)
- **(init):** Start to implement linting based on .tbdflow.yml settings [`9c20716`](https://github.com/cladam/tbdflow/commit/9c2071602c833893c3ccb5bf2216f6c70526f31c)
- **(init):** Improve init command\n\nNow it only creaates file if tbdflow is run in a repo, it will kindly ask if you want to create one tho [`675b580`](https://github.com/cladam/tbdflow/commit/675b580ce65d5920b9567864b336b853e2bb5d5b)

### 🐛 Bug Fixes
- **(linting):** Fix a bug in how the mapping from yaml file to a rust struct was done (wrong names) [`c78da2f`](https://github.com/cladam/tbdflow/commit/c78da2f8415d3d07333ae18e9ad68518ebdcd690)
- **(cargo):** Bump version and publish [`f32fea6`](https://github.com/cladam/tbdflow/commit/f32fea68f2697ebb7fb9a9d06f4ddcb2d49f10ab)
- **(cargo):** Update Cargo.toml with the right version [`e84098c`](https://github.com/cladam/tbdflow/commit/e84098c77899ce97abcdd2c4c8db9e3cb7477030)
- **(cargo):** Update README with the --issue flag and bump version [`ed546cb`](https://github.com/cladam/tbdflow/commit/ed546cbbf71961bfaa75928bbc4dc37c674a700d)

### 🔨 Code Refactoring
- **(linting):** Modify Commit command to handle header and footer in a cleaner way [`a8623a5`](https://github.com/cladam/tbdflow/commit/a8623a541af9d072ac0125e73b71d35c50bc56ad)
- **(linting):** Restructure the code and add linting for the issue key [`d28d640`](https://github.com/cladam/tbdflow/commit/d28d6406519bfa35630f5a8b8a653aae3427a024)

### ⚙️ Maintenance
- **(linting):** Bump version and publish [`075f0b2`](https://github.com/cladam/tbdflow/commit/075f0b2fa888cf059cdd9d01c4d2bc8b79ad7773)
- Tesing absence of --issue [`479bcb9`](https://github.com/cladam/tbdflow/commit/479bcb9db4befdaa70319175c263ec97ef9b8d17)
- Tesing absence of --issue [`7f718fa`](https://github.com/cladam/tbdflow/commit/7f718fa88c3c049e834beeb17864ca68a8797a62)

### Miscellaneous
- test [`4fec880`](https://github.com/cladam/tbdflow/commit/4fec880a3a02bfcf4071e59ddc2a1220b1b22007)
- **(lint):** Add lint configuration to tbdflow.yml, these checks will run before commit and DoD [`91a6b83`](https://github.com/cladam/tbdflow/commit/91a6b83b41861edba83b6fa206084d91f89d0976)


# [0.8.0](https://github.com/cladam/tbdflow/releases/tag/v0.8.0)

### ✨ Features
- **(commit):** Add functionality to validate if the provided --type is one of the standard Conventional Commit types [`7f3c36e`](https://github.com/cladam/tbdflow/commit/7f3c36e44446d6267644934d98b5ec413bad0d64)
- **(init):** Add functionality for the default branch and update the README [`4417c3f`](https://github.com/cladam/tbdflow/commit/4417c3f1f029a91318b2aea4d06d5f2508ce375e)
- **(init):** Add a new command called init, it will create the .tbdflow.yml and .dod.yml files with sensible defaults [`5ca73a5`](https://github.com/cladam/tbdflow/commit/5ca73a54dede59f1c7910d6f6fe7f33acc3682be)

### ⚙️ Maintenance
- **(cargo):** Bump version for the type validation and refactor of code [`923acc8`](https://github.com/cladam/tbdflow/commit/923acc892eeeb34f7992c8f621696916a0c84628)
- **(cargo):** Update README and bump version [`1f32c42`](https://github.com/cladam/tbdflow/commit/1f32c4211876eec56a38f9cf81d4741dd3ffc92c)
- **(DoD):** Add a document about DoD Check background and philosophy [`b77c474`](https://github.com/cladam/tbdflow/commit/b77c474053fa5a0f572fb77c34e767032b86c8d6)

### Miscellaneous
- Refactor structure to make the code easier to browse and read [`c449c1e`](https://github.com/cladam/tbdflow/commit/c449c1ec34eabaf9ea946f3898e65da38c8fa924)


# [0.6.1](https://github.com/cladam/tbdflow/releases/tag/v0.6.1)

### ✨ Features
- **(DoD):** Bump the release version to add the latest README [`2eaa947`](https://github.com/cladam/tbdflow/commit/2eaa947f26a7a4df7df0f65e01aeb0cc9b57f085)
- **(DoD):** Update README about DoD functionality [`99c233b`](https://github.com/cladam/tbdflow/commit/99c233b6426b9c7b448f4f46d9a8192975c22134)
- **(DoD):** Update Commit command to handle interactive commits with the .dod.yml file [`9fa80a5`](https://github.com/cladam/tbdflow/commit/9fa80a57bd7a6ec13f38db22cfed1054d47a888a)
- **(DoD):** Add functionality for handling the interactive commit and adding an example .dod.yml file [`a4ed63b`](https://github.com/cladam/tbdflow/commit/a4ed63b9606f929caae448a89ebbd938ad1d365d)
- **(DoD):** Add functionality from check-commit for the DoD check - interactive checklist with dialoguer [`a501d36`](https://github.com/cladam/tbdflow/commit/a501d36d8eec71137aee65e931dd8eaf0fcb52d2)
- Add dod config and a function to reaad the file [`499a602`](https://github.com/cladam/tbdflow/commit/499a6021b031238ada7a41f5e6c5c50f8ce11661)

### 🐛 Bug Fixes
- Change delimeter for branches to _ to not confuse git with a /. Update tests. [`679530c`](https://github.com/cladam/tbdflow/commit/679530c9241278c4c3e166ca2c53e1126c83bdcb)

### ⚙️ Maintenance
- **(DoD):** Update README [`f283b7d`](https://github.com/cladam/tbdflow/commit/f283b7d8b45d82d933ef7935335491266bde42f9)
- Update README with proper table [`02adc74`](https://github.com/cladam/tbdflow/commit/02adc74688c0e9495d12d36798146015910c9db3)
- Update README with global --verbose flag [`008a717`](https://github.com/cladam/tbdflow/commit/008a717971a34e59d65e2ba406aa88d3fc83c70e)


# [0.5.0](https://github.com/cladam/tbdflow/releases/tag/v0.5.0)

### ✨ Features
- **(debug):** Add a global verbose flag and hide which underlying git commands we are running [`57625ad`](https://github.com/cladam/tbdflow/commit/57625ad358d2ead22f87e68f31e0d69a4b16df5e)


# [0.4.3-1](https://github.com/cladam/tbdflow/releases/tag/v0.4.3-1)

### 🐛 Bug Fixes
- **(cargo):** Update version in Cargo.toml and publish new tag [`56d1c64`](https://github.com/cladam/tbdflow/commit/56d1c64b4c3727051964d481e313ff5cc80202f4)
- Make check-branches command only work on main [`4f10a07`](https://github.com/cladam/tbdflow/commit/4f10a07760d7995d7edfd00c06eb3f201dcf4611)

### ⚙️ Maintenance
- **(cargo):** Fixed a test that failed due to previous commit [`8c05045`](https://github.com/cladam/tbdflow/commit/8c05045c04323468d6b27611c4599b7b6b14d043)


# [0.4.2](https://github.com/cladam/tbdflow/releases/tag/v0.4.2)

### ✨ Features
- **(docs):** Updating documentation for the commands and adding examples, see '--help' per command. Also adding a hidden command to generate a man page. Adding a breaking description to the commit command that will be put in the footer like 'BREAKING CHANGE:' [`c24a6a5`](https://github.com/cladam/tbdflow/commit/c24a6a5a1d42e7c09d0c3c8a4bbb9b9732a49f07)

### 🐛 Bug Fixes
- **(docs):** updating the README with the '--breaking-description' argument [`280d567`](https://github.com/cladam/tbdflow/commit/280d5674ca672bf504cd0d972499bdc2a51ec490)
- **(test):** Disabling the version flaf for release, as it parses my example and fails... [`05990c6`](https://github.com/cladam/tbdflow/commit/05990c6144d6b764103e0b8f3284b0ef87be8c9d)
- **(doc):** Updating README, removed docs passing badge [`48400ad`](https://github.com/cladam/tbdflow/commit/48400ad0c3de8291baaba440c05b54b7191bb3da)
- **(doc):** Updating README with formatting [`85b02d3`](https://github.com/cladam/tbdflow/commit/85b02d3169327b818203cc6744cd7d7c909e47b2)
- **(doc):** Updating README, removed indents [`2611a62`](https://github.com/cladam/tbdflow/commit/2611a624f9687a931a882803e9ebc527d20174f5)
- **(doc):** Updating README with badges [`cccef19`](https://github.com/cladam/tbdflow/commit/cccef190dd0245b49f911d5088bdb4f80408d932)


# [0.3.2](https://github.com/cladam/tbdflow/releases/tag/0.3.2)

### ✨ Features
- **(workflow):** Create ci-and-publish.yml [`6bd8aa8`](https://github.com/cladam/tbdflow/commit/6bd8aa8ec5b874d2e1010a908845ad48cb395377)

### 🐛 Bug Fixes
- Splitting to two workflows [`b5375c3`](https://github.com/cladam/tbdflow/commit/b5375c3a22aebd8b72e16f5c556fea665e1f4e7b)
- Removing old workflow files [`b6db5ed`](https://github.com/cladam/tbdflow/commit/b6db5ed4dc2a482d860a42893981f81a3cb0060d)
- **(crate):** Bumping patch version to publish a new package on crates.io [`5b5214d`](https://github.com/cladam/tbdflow/commit/5b5214d53d7fc5b82d88d443a11ef8c921d02c33)
- **(crate):** Update publish-crate.yml [`8ff63d6`](https://github.com/cladam/tbdflow/commit/8ff63d6b3e30e4b15b31e9cf453cca434577f6db)

