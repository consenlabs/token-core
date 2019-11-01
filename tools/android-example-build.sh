#!/bin/bash



export ANDROID_NDK_TOOLCHAINS=$HOME/Library/Android/sdk/ndk-bundle/toolchains/llvm/prebuilt/darwin-x86_64/bin
JNI_LIBS=../examples/android/app/src/main/jniLibs
if [ ! -d $JNI_LIBS ]; then
    mkdir $JNI_LIBS
    mkdir $JNI_LIBS/arm64-v8a
    mkdir $JNI_LIBS/armeabi-v7a
    mkdir $JNI_LIBS/x86
    mkdir $JNI_LIBS/x86_64
fi

pushd ../libs/secp256k1
JNI_LIBS=../../examples/android/app/src/main/jniLibs

AR=$ANDROID_NDK_TOOLCHAINS/aarch64-linux-android-ar CC=$ANDROID_NDK_TOOLCHAINS/aarch64-linux-android22-clang cargo build --target aarch64-linux-android --release
AR=$ANDROID_NDK_TOOLCHAINS/arm-linux-androideabi-ar CC=$ANDROID_NDK_TOOLCHAINS/armv7a-linux-androideabi22-clang cargo build --target armv7-linux-androideabi --release
AR=$ANDROID_NDK_TOOLCHAINS/i686-linux-android-ar CC=$ANDROID_NDK_TOOLCHAINS/i686-linux-android22-clang cargo build --target i686-linux-android --release
AR=$ANDROID_NDK_TOOLCHAINS/x86_64-linux-android-ar CC=$ANDROID_NDK_TOOLCHAINS/x86_64-linux-android22-clang cargo build --target x86_64-linux-android --release

cp target/aarch64-linux-android/release/libtcx.so $JNI_LIBS/arm64-v8a
cp target/armv7-linux-androideabi/release/libtcx.so $JNI_LIBS/armeabi-v7a
cp target/i686-linux-android/release/libtcx.so $JNI_LIBS/x86
cp target/x86_64-linux-android/release/libtcx.so $JNI_LIBS/x86_64


popd


pushd ../tcx
JNI_LIBS=../examples/android/app/src/main/jniLibs
AR=$ANDROID_NDK_TOOLCHAINS/aarch64-linux-android-ar CC=$ANDROID_NDK_TOOLCHAINS/aarch64-linux-android22-clang cargo build --target aarch64-linux-android --release
AR=$ANDROID_NDK_TOOLCHAINS/arm-linux-androideabi-ar CC=$ANDROID_NDK_TOOLCHAINS/armv7a-linux-androideabi22-clang cargo build --target armv7-linux-androideabi --release
AR=$ANDROID_NDK_TOOLCHAINS/i686-linux-android-ar CC=$ANDROID_NDK_TOOLCHAINS/i686-linux-android22-clang cargo build --target i686-linux-android --release
AR=$ANDROID_NDK_TOOLCHAINS/x86_64-linux-android-ar CC=$ANDROID_NDK_TOOLCHAINS/x86_64-linux-android22-clang cargo build --target x86_64-linux-android --release

# cargo build --target aarch64-linux-android
# cargo build --target armv7-linux-androideabi
# cargo build --target i686-linux-android
# cargo build --target x86_64-linux-android



cp ../target/aarch64-linux-android/release/libtcx.so $JNI_LIBS/arm64-v8a
cp ../target/armv7-linux-androideabi/release/libtcx.so $JNI_LIBS/armeabi-v7a
cp ../target/i686-linux-android/release/libtcx.so $JNI_LIBS/x86
cp ../target/x86_64-linux-android/release/libtcx.so $JNI_LIBS/x86_64

popd
