## Unreleased ([da242e3..39c244b](https://github.com/dsxragnarok/ffbetool/compare/da242e3..39c244b))

- - -

## [0.6.0](https://github.com/dsxragnarok/ffbetool/compare/0.5.0..0.6.0) - 2025-08-20
#### Features
- impl autodiscovery & proc all cgs anims in given input dir - ([da242e3](https://github.com/dsxragnarok/ffbetool/commit/da242e3172117a9510710333b05ce5a5bc189a5a)) - [@dsxragnarok](https://github.com/dsxragnarok)

- - -

## [0.5.0](https://github.com/dsxragnarok/ffbetool/compare/0.4.0..0.5.0) - 2025-08-20
#### Bug Fixes
- check against params len of 4 instead of 2 - ([3c929fd](https://github.com/dsxragnarok/ffbetool/commit/3c929fd7adb227834f16d18d661c21cb7bf3a863)) - [@dsxragnarok](https://github.com/dsxragnarok)
- incorrect rotation handling - ([6d1c0f3](https://github.com/dsxragnarok/ffbetool/commit/6d1c0f34ca33e2524a715e3bd93f01d4d9f2a939)) - [@dsxragnarok](https://github.com/dsxragnarok)
#### Features
- impl --empty flag to include empty frames - ([269dd7d](https://github.com/dsxragnarok/ffbetool/commit/269dd7d33fadaccc7127f9cfd31f00471b3a894a)) - [@dsxragnarok](https://github.com/dsxragnarok)
#### Refactoring
- large main fn into small single responsibility fns - ([5db6e83](https://github.com/dsxragnarok/ffbetool/commit/5db6e834a7e4981a0486815fe3d788b4f0b928e9)) - [@dsxragnarok](https://github.com/dsxragnarok)
- replace magic numbers with constants - ([5152738](https://github.com/dsxragnarok/ffbetool/commit/5152738f78f5af16866506aaeb88e748d01d4a89)) - [@dsxragnarok](https://github.com/dsxragnarok)
- replace expects with proper error propagation and handling - ([10b0ffe](https://github.com/dsxragnarok/ffbetool/commit/10b0ffe84cd2185bf6bc175e6ced11f87ed3001c)) - [@dsxragnarok](https://github.com/dsxragnarok)
- replace magic numbers with constants - ([f708070](https://github.com/dsxragnarok/ffbetool/commit/f708070f7712cadac05e700d508e1150037894ea)) - [@dsxragnarok](https://github.com/dsxragnarok)
- clarify the canvas size magic number - ([b8502ad](https://github.com/dsxragnarok/ffbetool/commit/b8502adf22df3b5969408ab39c55b87e35e5304d)) - [@dsxragnarok](https://github.com/dsxragnarok)
- extract bounding box calc into its own fn - ([c989999](https://github.com/dsxragnarok/ffbetool/commit/c9899997b94173e59ac3cc5c122fd0552e8a9258)) - [@dsxragnarok](https://github.com/dsxragnarok)
- do processing in parallel - ([dc96cfe](https://github.com/dsxragnarok/ffbetool/commit/dc96cfefcb5f24373feb6cb4de7ddddd320920bd)) - [@dsxragnarok](https://github.com/dsxragnarok)
- impl early input validation - ([6eb8c5e](https://github.com/dsxragnarok/ffbetool/commit/6eb8c5e2d80199b9d6fb6031f3d87f31306de3cf)) - [@dsxragnarok](https://github.com/dsxragnarok)
- impl proper error handling and propagation - ([338cbbe](https://github.com/dsxragnarok/ffbetool/commit/338cbbeb552f42caeb34f2d73cd0ed1c0a499f8a)) - [@dsxragnarok](https://github.com/dsxragnarok)
- replace unwrap and string errors with custom errors - ([31f49a6](https://github.com/dsxragnarok/ffbetool/commit/31f49a6847c780907bbeecf0af2e3221e5fe2205)) - [@dsxragnarok](https://github.com/dsxragnarok)
- create custom error types - ([79c70f6](https://github.com/dsxragnarok/ffbetool/commit/79c70f6ed84f89dd11e010895def8b4d3c0d526a)) - [@dsxragnarok](https://github.com/dsxragnarok)

- - -

## [0.4.0](https://github.com/dsxragnarok/ffbetool/compare/0.3.0..0.4.0) - 2025-08-20
#### Features
- impl more robust cmdline arg parsing - ([7c17e7c](https://github.com/dsxragnarok/ffbetool/commit/7c17e7c2edda1a83499edac591f22816d4864e51)) - [@dsxragnarok](https://github.com/dsxragnarok)

- - -

## [0.3.0](https://github.com/dsxragnarok/ffbetool/compare/0.2.0..0.3.0) - 2025-08-20
#### Bug Fixes
- incorrect rotate direction - ([8d39559](https://github.com/dsxragnarok/ffbetool/commit/8d395596fc015d9699fc01302ccb89fea6d94bde)) - [@dsxragnarok](https://github.com/dsxragnarok)
#### Features
- wip - impl save animated apng file - ([bca495b](https://github.com/dsxragnarok/ffbetool/commit/bca495b35fa4bb91815e89699e7a229a5c5dc3a8)) - [@dsxragnarok](https://github.com/dsxragnarok)
#### Refactoring
- save apng - ([e0fb8b8](https://github.com/dsxragnarok/ffbetool/commit/e0fb8b8178e7d03ae88144627a10c5f54a7ce372)) - [@dsxragnarok](https://github.com/dsxragnarok)

- - -

## [0.2.0](https://github.com/dsxragnarok/ffbetool/compare/0.1.0..0.2.0) - 2025-08-20
#### Features
- impl save animated gif - ([3804883](https://github.com/dsxragnarok/ffbetool/commit/380488329265f9eb360a4ea25d7cce73a42a57f5)) - [@dsxragnarok](https://github.com/dsxragnarok)
#### Refactoring
- crop the frame images beforehand - ([7d59ce0](https://github.com/dsxragnarok/ffbetool/commit/7d59ce078a2d6044a4fa7bba3dcdfa7c91f2632b)) - [@dsxragnarok](https://github.com/dsxragnarok)
- Point and Rect into proper structs for maintainability - ([3168fb9](https://github.com/dsxragnarok/ffbetool/commit/3168fb9331ccd12a88a2988437627370436d454d)) - [@dsxragnarok](https://github.com/dsxragnarok)

- - -

## [0.1.0](https://github.com/dsxragnarok/ffbetool/compare/35821413ae6e776c568902948979d33f5a3e6849..0.1.0) - 2025-08-20
#### Features
- render spritesheet - ([49b1463](https://github.com/dsxragnarok/ffbetool/commit/49b14637b385d71871bb80572c9d216273bced6c)) - [@dsxragnarok](https://github.com/dsxragnarok)
- impl get_color_bounds_rect - ([d254b5e](https://github.com/dsxragnarok/ffbetool/commit/d254b5e411727388729a91fac15f93936caadd6a)) - [@dsxragnarok](https://github.com/dsxragnarok)
- apply opacity and save frames as individual png files - ([e8c9ddb](https://github.com/dsxragnarok/ffbetool/commit/e8c9ddb81d4701a77115b648ffe03556b71674e4)) - [@dsxragnarok](https://github.com/dsxragnarok)
- impl opacity trait - ([bdd7025](https://github.com/dsxragnarok/ffbetool/commit/bdd702566182509c6f90ce20d42044da7d1951a6)) - [@dsxragnarok](https://github.com/dsxragnarok)
- wip - render parts [blend, flip, rotate] - ([cc6f2c3](https://github.com/dsxragnarok/ffbetool/commit/cc6f2c33cab0ff17216e7c39f0fb0df57d1e3c66)) - [@dsxragnarok](https://github.com/dsxragnarok)
- wip - render each frame part - ([a33bc95](https://github.com/dsxragnarok/ffbetool/commit/a33bc958965b8c978b621d026ac2d75f9bd511de)) - [@dsxragnarok](https://github.com/dsxragnarok)
- process cgs data into cgs frames - ([cadcbee](https://github.com/dsxragnarok/ffbetool/commit/cadcbeeec73591d9c67068a3251f45b1121dceb2)) - [@dsxragnarok](https://github.com/dsxragnarok)
- wip - impl src img load and cgs process - ([d83a91e](https://github.com/dsxragnarok/ffbetool/commit/d83a91e450e1a67903ee23ad2910296a5b11bdf7)) - [@dsxragnarok](https://github.com/dsxragnarok)
- take unit_id and input_path from cli args - ([7991ee1](https://github.com/dsxragnarok/ffbetool/commit/7991ee1b156b87eac68a6bf348886b97cf951b4e)) - [@dsxragnarok](https://github.com/dsxragnarok)
- wip - impl cgg parsing logic - ([bdc41ed](https://github.com/dsxragnarok/ffbetool/commit/bdc41eda854c76558fe9650dd06379fad69d276b)) - [@dsxragnarok](https://github.com/dsxragnarok)
- setup main, lib and cgg module - ([9aef1ab](https://github.com/dsxragnarok/ffbetool/commit/9aef1abe0f9fad156f9c652ae746f41ffc65b044)) - [@dsxragnarok](https://github.com/dsxragnarok)
#### Refactoring
- consolidate frame data into different frame states - ([89817fb](https://github.com/dsxragnarok/ffbetool/commit/89817fbf9036917bada7375ed490e84c1d5347ce)) - [@dsxragnarok](https://github.com/dsxragnarok)
- cgs does not need to duplicate cgg PartData - ([6590636](https://github.com/dsxragnarok/ffbetool/commit/659063648f20bbbc1d1a885282b6a277cd9b0072)) - [@dsxragnarok](https://github.com/dsxragnarok)
- impl cgs processing - ([8a8c999](https://github.com/dsxragnarok/ffbetool/commit/8a8c999c44693dd1c977f721ab4ed2174d72daa1)) - [@dsxragnarok](https://github.com/dsxragnarok)
- collect into frames - ([6daa360](https://github.com/dsxragnarok/ffbetool/commit/6daa3606a8e73f52cf2165900e5f782491b40618)) - [@dsxragnarok](https://github.com/dsxragnarok)


