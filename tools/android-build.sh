#!/bin/bash

JNI_LIBS=../examples/android/app/src/main/jniLibs
TREZOR_CRYPTO_LIBS=../examples/android/tokencore/build/intermediates/cmake/release/obj

# pushd ../android
# ./gradlew assembleRelease
# popd


# export ANDROID_NDK_STANDALONE_TOOLCHAINS=$HOME/Users/xyz/Library/Android/sdk/ndk-bundle/toolchains/llvm/prebuilt/darwin-x86_64/
# export PATH="$ANDROID_NDK_STANDALONE_TOOLCHAINS/arm/bin:$PATH"
# export PATH="$ANDROID_NDK_STANDALONE_TOOLCHAINS/arm64/bin:$PATH"
# export PATH="$ANDROID_NDK_STANDALONE_TOOLCHAINS/x86/bin:$PATH"
# export PATH="$ANDROID_NDK_STANDALONE_TOOLCHAINS/x86_64/bin:$PATH”


pushd ../tcx
export ANDROID_NDK_TOOLCHAINS=$HOME/Library/Android/sdk/ndk-bundle/toolchains/llvm/prebuilt/darwin-x86_64/bin

AR=$ANDROID_NDK_TOOLCHAINS/aarch64-linux-android-ar CC=$ANDROID_NDK_TOOLCHAINS/aarch64-linux-android22-clang cargo build --target aarch64-linux-android --release
AR=$ANDROID_NDK_TOOLCHAINS/arm-linux-androideabi-ar CC=$ANDROID_NDK_TOOLCHAINS/armv7a-linux-androideabi22-clang cargo build --target armv7-linux-androideabi --release
AR=$ANDROID_NDK_TOOLCHAINS/i686-linux-android-ar CC=$ANDROID_NDK_TOOLCHAINS/i686-linux-android22-clang cargo build --target i686-linux-android --release
AR=$ANDROID_NDK_TOOLCHAINS/x86_64-linux-android-ar CC=$ANDROID_NDK_TOOLCHAINS/x86_64-linux-android22-clang cargo build --target x86_64-linux-android --release

# cargo build --target aarch64-linux-android
# cargo build --target armv7-linux-androideabi
# cargo build --target i686-linux-android
# cargo build --target x86_64-linux-android


# rm -rf $JNI_LIBS
# mkdir $JNI_LIBS
# mkdir $JNI_LIBS/arm64-v8a
# mkdir $JNI_LIBS/armeabi-v7a
# mkdir $JNI_LIBS/x86
# mkdir $JNI_LIBS/x86_64


cp ../target/aarch64-linux-android/release/libtcx.so $JNI_LIBS/arm64-v8a
cp ../target/armv7-linux-androideabi/release/libtcx.so $JNI_LIBS/armeabi-v7a
cp ../target/i686-linux-android/release/libtcx.so $JNI_LIBS/x86
cp ../target/x86_64-linux-android/release/libtcx.so $JNI_LIBS/x86_64

# cp $TREZOR_CRYPTO_LIBS/arm64-v8a/libTrezorCrypto.so $JNI_LIBS/arm64-v8a
# cp $TREZOR_CRYPTO_LIBS/armeabi-v7a/libTrezorCrypto.so $JNI_LIBS/armeabi-v7a
# cp $TREZOR_CRYPTO_LIBS/x86/libTrezorCrypto.so $JNI_LIBS/x86
# cp $TREZOR_CRYPTO_LIBS/x86_64/libTrezorCrypto.so $JNI_LIBS/x86_64

popd