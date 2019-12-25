#!/bin/bash

export ANDROID_NDK_TOOLCHAINS=$HOME/Library/Android/sdk/ndk-bundle/toolchains/llvm/prebuilt/darwin-x86_64/bin
export PATH=$ANDROID_NDK_TOOLCHAINS:$PATH

JNI_LIBS=../examples/android/app/src/main/jniLibs
if [ ! -d $JNI_LIBS ]; then
    mkdir $JNI_LIBS
    mkdir $JNI_LIBS/arm64-v8a
    mkdir $JNI_LIBS/armeabi-v7a
    mkdir $JNI_LIBS/x86
    mkdir $JNI_LIBS/x86_64
fi

pushd $ANDROID_NDK_TOOLCHAINS

if [ ! -d aarch64-linux-android-clang ]; then
    ln -s -f aarch64-linux-android22-clang aarch64-linux-android-clang
    ln -s -f armv7a-linux-androideabi22-clang arm-linux-androideabi-clang
    ln -s -f i686-linux-android22-clang i686-linux-android-clang
    ln -s -f x86_64-linux-android22-clang x86_64-linux-android-clang
fi
popd

pushd ../libs/secp256k1
JNI_LIBS=../../examples/android/app/src/main/jniLibs

cargo build --target aarch64-linux-android --release
cargo build --target armv7-linux-androideabi --release
cargo build --target i686-linux-android --release
cargo build --target x86_64-linux-android --release


cp target/aarch64-linux-android/release/libsecp256k1.so $JNI_LIBS/arm64-v8a
cp target/armv7-linux-androideabi/release/libsecp256k1.so $JNI_LIBS/armeabi-v7a
cp target/i686-linux-android/release/libsecp256k1.so $JNI_LIBS/x86
cp target/x86_64-linux-android/release/libsecp256k1.so $JNI_LIBS/x86_64


popd


pushd ../tcx
JNI_LIBS=../../token-v2/android/app/src/main/jniLibs

cargo build --target aarch64-linux-android --release
cargo build --target armv7-linux-androideabi --release
cargo build --target i686-linux-android --release
cargo build --target x86_64-linux-android --release

cp ../target/aarch64-linux-android/release/libtcx.so $JNI_LIBS/arm64-v8a
cp ../target/armv7-linux-androideabi/release/libtcx.so $JNI_LIBS/armeabi-v7a
cp ../target/i686-linux-android/release/libtcx.so $JNI_LIBS/x86
cp ../target/x86_64-linux-android/release/libtcx.so $JNI_LIBS/x86_64

popd
