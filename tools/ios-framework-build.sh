#!/bin/bash

# Step1. compile rust

LIBS=examples/TokenCoreX/TokenCoreX

# if [ ! -d $LIBS ]; then
#  mkdir -p $LIBS/Include
#  mkdir -p $LIBS/Libs
# fi

#pushd libs/secp256k1
#if ! type "cargo-lipo" > /dev/null; then
#   cargo install cargo-lipo
#   rustup target add aarch64-apple-ios x86_64-apple-ios
#fi
#LIBS=../../examples/TokenCoreX/TokenCoreX
#cargo lipo --release --targets aarch64-apple-ios x86_64-apple-ios
#
#cp target/universal/release/libsecp256k1.a $LIBS
#popd

pushd tcx
LIBS=../examples/TokenCoreX/TokenCoreX
if [ ! -d cheader ]; then
  mkdir -p cheader
fi
RUST_BACKTRACE=1 cbindgen src/lib.rs -l c > cheader/tcx.h
cargo lipo --release  --targets aarch64-apple-ios x86_64-apple-ios

cp cheader/tcx.h $LIBS
cp ../target/universal/release/libtcx.a $LIBS
popd


pushd examples/TokenCoreX
# xcodebuild build -project TokenCoreX.xcodeproj -scheme TokenCoreX-Universal -sdk iphoneos13.2
# xcodebuild build -project TokenCoreX.xcodeproj -scheme TokenCoreX-Universal -sdk iphonesimulator13.2

BUILD_DIR=./Products
BUILD_ROOT=./Products
SYMROOT=./Products
BUILD_PRODUCTS=./Products
CONFIGURATION=Release
PROJECT_NAME=TokenCoreX

mkdir -p $BUILD_DIR
UNIVERSAL_OUTPUTFOLDER=$BUILD_DIR/$CONFIGURATION-Universal
rm -rf ../../ios-release/*


# Next, work out if we're in SIMULATOR or REAL DEVICE
xcodebuild clean
# Make sure the output directory exists
mkdir -p $UNIVERSAL_OUTPUTFOLDER
xcodebuild -target $PROJECT_NAME ONLY_ACTIVE_ARCH=NO -configuration $CONFIGURATION -sdk iphoneos BUILD_DIR=$BUILD_DIR BUILD_ROOT=$BUILD_ROOT build
#export XCODE_XCCONFIG_FILE=/Users/xyz/Documents/code/token-core/examples/TokenCoreX/excluded_duplicated_arm64.xcconfig
#xcodebuild -target $PROJECT_NAME -configuration $CONFIGURATION -sdk iphonesimulator ONLY_ACTIVE_ARCH=NO BUILD_DIR=$BUILD_DIR BUILD_ROOT=$BUILD_ROOT -xcconfig /Users/xyz/Documents/code/token-core/examples/TokenCoreX/excluded_duplicated_arm64.xcconfig  build
xcodebuild -target $PROJECT_NAME -configuration Debug -sdk iphonesimulator ONLY_ACTIVE_ARCH=NO BUILD_DIR=$BUILD_DIR BUILD_ROOT=$BUILD_ROOT EXCLUDED_ARCHS=arm64 build

# Step 2. Copy the framework structure (from iphoneos build) to the universal folder
cp -R $BUILD_DIR/$CONFIGURATION-iphoneos/$PROJECT_NAME.framework $UNIVERSAL_OUTPUTFOLDER/
# Step 3. Copy Swift modules from iphonesimulator build (if it exists) to the copied framework directory
# BUILD_PRODUCTS=$SYMROOT/../../../../Products
#cp -R $BUILD_PRODUCTS/Debug-iphonesimulator/$PROJECT_NAME.framework/Modules/$PROJECT_NAME.swiftmodule/. $UNIVERSAL_OUTPUTFOLDER/$PROJECT_NAME.framework/Modules/$PROJECT_NAME.swiftmodule
# Step 4. Create universal binary file using lipo and place the combined executable in the copied framework directory
lipo -create -output $UNIVERSAL_OUTPUTFOLDER/$PROJECT_NAME.framework/$PROJECT_NAME $BUILD_PRODUCTS/Debug-iphonesimulator/$PROJECT_NAME.framework/$PROJECT_NAME $BUILD_DIR/$CONFIGURATION-iphoneos/$PROJECT_NAME.framework/$PROJECT_NAME
#lipo -create -output $UNIVERSAL_OUTPUTFOLDER/$PROJECT_NAME.framework/$PROJECT_NAME $BUILD_DIR/$CONFIGURATION-iphoneos/$PROJECT_NAME.framework/$PROJECT_NAME
# Step 5. Convenience step to copy the framework to the project's directory
cp -R $UNIVERSAL_OUTPUTFOLDER/$PROJECT_NAME.framework ../../ios-release
rm -rf $UNIVERSAL_OUTPUTFOLDER

popd