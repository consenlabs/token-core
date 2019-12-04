#!/bin/bash

LIBS=../examples/RN/ios/TokenCoreX


if [ ! -d $LIBS ]; then
  mkdir -p $LIBS/Include
  mkdir -p $LIBS/Libs
fi

pushd ../libs/secp256k1
if ! type "cargo-lipo" > /dev/null; then
    cargo install cargo-lipo
    rustup target add aarch64-apple-ios x86_64-apple-ios armv7-apple-ios armv7s-apple-ios i386-apple-ios
fi
LIBS=../../examples/RN/ios/TokenCoreX
cargo lipo --release --targets aarch64-apple-ios,armv7-apple-ios,armv7s-apple-ios,x86_64-apple-ios,i386-apple-ios

cp target/universal/release/libsecp256k1.a $LIBS/Libs
popd

pushd ../tcx
LIBS=../examples/RN/ios/TokenCoreX
if [ ! -d cheader ]; then
  mkdir -p cheader
fi
RUST_BACKTRACE=1 cbindgen src/lib.rs -l c > cheader/tcx.h
cargo lipo --release  --targets aarch64-apple-ios,armv7-apple-ios,armv7s-apple-ios,x86_64-apple-ios,i386-apple-ios

cp cheader/tcx.h $LIBS/Include
cp ../target/universal/release/libtcx.a $LIBS/Libs
popd
