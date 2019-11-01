pushd ../
export CARGO_INCREMENTAL=0
export RUSTFLAGS="-Zprofile -Ccodegen-units=1 -Cinline-threshold=0 -Clink-dead-code -Coverflow-checks=off -Zno-landing-pads"
cargo clean
find . -name "*.gcda" -print0 | xargs -0 rm
cargo build
cargo test
#mkdir ccov
zip -0 ccov/ccov.zip `find . \( -name "*.gc*" \) -print`
grcov ccov/ccov.zip -s . -t lcov --llvm --branch --ignore-not-existing --ignore "/*" -o ccov/lcov.info
genhtml -o ccov/ --show-details --highlight --ignore-errors source --legend ccov/lcov.info
open ccov/index.html
unset CARGO_INCREMENTAL
unset RUSTFLAGS

popd