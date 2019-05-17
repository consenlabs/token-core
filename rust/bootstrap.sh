#!/bin/sh
JNI_LIBS=../android/tokencore/src/main/jniLibs
TREZOR_CRYPTO_LIBS=../android/tokencore/build/intermediates/cmake/release/obj

cargo build --target aarch64-linux-android --release
cargo build --target armv7-linux-androideabi --release
cargo build --target i686-linux-android --release
cargo build --target x86_64-linux-android --release

# cargo build --target aarch64-linux-android
# cargo build --target armv7-linux-androideabi
# cargo build --target i686-linux-android
# cargo build --target x86_64-linux-android


rm -rf $JNI_LIBS
mkdir $JNI_LIBS
mkdir $JNI_LIBS/arm64-v8a
mkdir $JNI_LIBS/armeabi-v7a
mkdir $JNI_LIBS/x86
mkdir $JNI_LIBS/x86_64


cp target/aarch64-linux-android/release/librust.so $JNI_LIBS/arm64-v8a
cp $TREZOR_CRYPTO_LIBS/arm64-v8a/libTrezorCrypto.so $JNI_LIBS/arm64-v8a
cp target/armv7-linux-androideabi/release/librust.so $JNI_LIBS/armeabi-v7a
cp $TREZOR_CRYPTO_LIBS/armeabi-v7a/libTrezorCrypto.so $JNI_LIBS/armeabi-v7a
cp target/i686-linux-android/release/librust.so $JNI_LIBS/x86
cp $TREZOR_CRYPTO_LIBS/x86/libTrezorCrypto.so $JNI_LIBS/x86
cp target/x86_64-linux-android/release/librust.so $JNI_LIBS/x86_64
cp $TREZOR_CRYPTO_LIBS/x86_64/libTrezorCrypto.so $JNI_LIBS/x86_64
