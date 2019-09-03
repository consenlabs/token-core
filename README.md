# TokenCoreX

TODO. add banner image

Next generation core inside imToken Wallet.

TODO. add architeture image here.

TODO. list components

## Test Coverage
We can use [tarpaulin](https://github.com/xd009642/tarpaulin) to know the coverage rate.

The easy way to run coverage test is using docker,

```
docker run --security-opt seccomp=unconfined -v "${PWD}:/volume" xd009642/tarpaulin sh -c "cargo tarpaulin --out Html"
```

After couple minutes, it will generate html report of project root directory named `tarpaulin-report.html`. 

## Auto Formatting
This project is using pre-commit. Please run `cargo clean && cargo test` to install the git pre-commit hooks on you clone.

Every time you will try to commit, pre-commit will run checks on your files to make sure they follow our style standards and they aren't affected by some simple issues. If the checks fail, pre-commit won't let you commit.

## License
Apache Licence v2.0
