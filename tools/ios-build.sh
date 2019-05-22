#!/bin/bash
# rm -rf build

# mkdir build
# cd build
# cmake ../../trezor-crypto -Wno-error=dev -G Xcode -DCMAKE_TOOLCHAIN_FILE=../ios.toolchain.cmake -DPLATFORM=OS64 
# cmake --build . --config Release


pushd ../rust/jni
cbindgen src/lib.rs -l c > rust.h
cargo lipo --release

mkdir -p ../../TokenCoreXExample/TokenCoreX/Include
mkdir -p ../../TokenCoreXExample/TokenCoreX/Libs
cp rust.h ../../TokenCoreXExample/TokenCoreX/Include
cp target/universal/release/librust.a ../../TokenCoreXExample/TokenCoreX/Libs
popd
