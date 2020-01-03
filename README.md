# TokenCoreX

[![Build status](https://travis-ci.org/consenlabs/token-core.svg?branch=dev)](https://travis-ci.org/consenlabs/token-core)
[![Gitter chat](https://badges.gitter.im/gitterHQ/gitter.png)](https://gitter.im/imtoken-wallet/token-core)

Next generation core inside imToken Wallet.

A cross-platform library that implements crypto wallet functions for blockchains, exporting c interfaces in Protobuf
protocol. This library totally wrote in Rust, and now provides friendly interfaces for the mobile platform including
ReactNative, iOS, and Android.

## Goals
* Abstration interfaces that provide crypto wallet common logic with multi blockchain support
* Cross platform, on mobile, desktop, server side
* Multi keystore support, with file, HSM, KMS, hardware-wallet

## Packages
* `tcx` wallet interface wrapper
* `tcx-bch` | `tcx-btc-fork` | `tcx-tron` | `tcx-ckb` packages contain particular chain operations
* `tcx-chain` keystore and signer interface
* `tcx-proto` protobuf datastructure definition
* `tcx-constants` blockchain spec definition
* [`tcx-primitive` | `tcx-crypto`] low level component

## Examples
We provide three example applications, 

* [ReactNative](examples/RN), full functions including all expose API, and e2e testing
* [iOS](examples/iOSExample)
* [Android](examples/android)

## Test Coverage
We can use [tarpaulin](https://github.com/xd009642/tarpaulin) to know the coverage rate.

The easy way to run coverage test is using docker,

```
docker run --security-opt seccomp=unconfined -v "${PWD}:/volume" xd009642/tarpaulin sh -c "cargo tarpaulin --out Html"
```

After couple minutes, it will generate html report of project root directory named `tarpaulin-report.html`. 

## Code Styles
This project is using pre-commit. Please run `cargo clean && cargo test` to install the git pre-commit hooks on you clone.

Every time you will try to commit, pre-commit will run checks on your files to make sure they follow our style standards
and they aren't affected by some simple issues. If the checks fail, pre-commit won't let you commit.

## Read More
* [How to build project](docs/BUILD.zh.md)
* [Crypto keys abstraction design](docs/KEYS.zh.md)
* [Architecture design](docs/TECH.zh.md)
* [How to add more blockchain support](docs/INTEGRATION.md)
* [Security](Security.md) including the bug bounty program
* [FAQ](docs/FAQ.md)

## License
Apache Licence v2.0
