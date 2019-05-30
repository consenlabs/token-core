#!/bin/bash
# rm -rf build

# mkdir build
# cd build
# cmake ../../trezor-crypto -Wno-error=dev -G Xcode -DCMAKE_TOOLCHAIN_FILE=../ios.toolchain.cmake -DPLATFORM=OS64 
# cmake --build . --config Release


pushd ../rust/tcx-lib
cbindgen src/lib.rs -l c > tcx.h
cargo lipo --release

mkdir -p ../../TokenCoreXExample/TokenCoreX/Include
mkdir -p ../../TokenCoreXExample/TokenCoreX/Libs
cp tcx.h ../../TokenCoreXExample/TokenCoreX/Include
cp ../target/universal/release/libtcx.a ../../TokenCoreXExample/TokenCoreX/Libs
popd
