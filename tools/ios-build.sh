#!/bin/bash
# rm -rf build

mkdir build
cd build
cmake ../../trezor-crypto -Wno-error=dev -G Xcode -DCMAKE_TOOLCHAIN_FILE=../ios.toolchain.cmake -DPLATFORM=OS64 
cmake --build . --config Release
