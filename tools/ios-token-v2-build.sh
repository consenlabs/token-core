#!/bin/bash
# rm -rf build

# mkdir build
# cd build
# cmake ../../trezor-crypto -Wno-error=dev -G Xcode -DCMAKE_TOOLCHAIN_FILE=../ios.toolchain.cmake -DPLATFORM=OS64 
# cmake --build . --config Release


pushd ../tcx
RUST_BACKTRACE=1 cbindgen src/lib.rs -l c > cheader/tcx.h
cargo lipo --release --targets aarch64-apple-ios,armv7-apple-ios,armv7s-apple-ios,x86_64-apple-ios,i386-apple-ios

#mkdir -p ../examples/iOSExample/TokenCoreX/Include
#mkdir -p ../examples/iOSExample/TokenCoreX/Libs
cp cheader/tcx.h ../../token-v2/ios/TokenCoreX/Include
cp ../target/universal/release/libtcx.a ../../token-v2/ios/TokenCoreX/Libs
popd
