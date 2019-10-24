#!/bin/bash

pushd ../tcx
RUST_BACKTRACE=1 cbindgen src/lib.rs -l c > cheader/tcx.h
cargo lipo --release

if [ ! -d "../examples/iOSExample/TokenCoreX" ]; then
  mkdir -p ../examples/iOSExample/TokenCoreX/Include
  mkdir -p ../examples/iOSExample/TokenCoreX/Libs
fi

cp cheader/tcx.h ../examples/iOSExample/TokenCoreX/Include
cp ../target/universal/release/libtcx.a ../examples/iOSExample/TokenCoreX/Libs
popd
