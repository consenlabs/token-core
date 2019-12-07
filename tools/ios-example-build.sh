#!/usr/bin/env bash

WORKDIR=$(pwd)
LIBS=$WORKDIR/../examples/iOSExample/TokenCoreX
RNLIBS=$WORKDIR/../examples/RN/ios/TokenCoreX

if [ ! -d $LIBS ]; then
  mkdir -p $LIBS/Include $LIBS/Libs
fi

if [ ! -d $RNLIBS ]; then
  mkdir -p $RNLIBS/Include $RNLIBS/Libs
fi

pushd ../libs/secp256k1
git submodule update --init --recursive
if ! type "cargo-lipo" > /dev/null; then
    cargo install cargo-lipo
    # rustup target add aarch64-apple-ios x86_64-apple-ios armv7-apple-ios armv7s-apple-ios i386-apple-ios
    rustup target add x86_64-apple-ios
fi
# cargo lipo --release --targets aarch64-apple-ios,armv7-apple-ios,armv7s-apple-ios,x86_64-apple-ios,i386-apple-ios
cargo lipo --release --targets x86_64-apple-ios
cp target/universal/release/libsecp256k1.a $LIBS/Libs
cp target/universal/release/libsecp256k1.a $RNLIBS/Libs
popd

pushd ../tcx
if [ ! -d cheader ]; then
  mkdir -p cheader
fi
RUST_BACKTRACE=1 cbindgen src/lib.rs -l c > cheader/tcx.h
cargo lipo --release  --targets x86_64-apple-ios
# cargo lipo --release  --targets aarch64-apple-ios,armv7-apple-ios,armv7s-apple-ios,x86_64-apple-ios,i386-apple-ios
cp cheader/tcx.h $LIBS/Include
cp cheader/tcx.h $RNLIBS/Include
cp ../target/universal/release/libtcx.a $LIBS/Libs
cp ../target/universal/release/libtcx.a $RNLIBS/Libs
popd
