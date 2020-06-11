# Build Token Core X 

## Install Rust 
1. Install Rustup    
`curl https://sh.rustup.rs -sSf | sh`    
Run `rustc --version` to check if the installation is success, this command will install cargo too.    

2. Install Android targets    
`rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android x86_64-linux-android`    

3. Install iOS targets    
`rustup target add aarch64-apple-ios armv7-apple-ios armv7s-apple-ios i386-apple-ios x86_64-apple-ios`    


## Setup iOS compile tool-chain    
1. Install Xcode     
2. Install lipo    
```    
cargo install cargo-lipo   
cargo install cbindgen   
```   
3. Run `tools/ios-token-v2-build.sh`. Set the right DIR in `tools/ios-token-v2-build.sh`. After running this shell, the 
lib files will be compiled and copied to the directory you set before.   

## Setup Android compile tool-chain  
1. Install Android SDK or use the SDK providing by Android Studio (you can find it in `/Users/$$your_username$$/Library/Android/sdk` on macOS)   
2. Config `~/.cargo/config`   

```
[target.aarch64-linux-android]
ar = "/Users/xxx/Library/Android/sdk/ndk-bundle/toolchains/llvm/prebuilt/darwin-x86_64/bin/aarch64-linux-android-ar"
linker = "/Users/xxx/Library/Android/sdk/ndk-bundle/toolchains/llvm/prebuilt/darwin-x86_64/bin/aarch64-linux-android22-clang"


[target.armv7-linux-androideabi]
ar = "/Users/xxx/Library/Android/sdk/ndk-bundle/toolchains/llvm/prebuilt/darwin-x86_64/bin/arm-linux-androideabi-ar"
linker = "/Users/xxx/Library/Android/sdk/ndk-bundle/toolchains/llvm/prebuilt/darwin-x86_64/bin/armv7a-linux-androideabi22-clang"

[target.i686-linux-android]
ar = "/Users/xxx/Library/Android/sdk/ndk-bundle/toolchains/llvm/prebuilt/darwin-x86_64/bin/i686-linux-android-ar"
linker = "/Users/xxx/Library/Android/sdk/ndk-bundle/toolchains/llvm/prebuilt/darwin-x86_64/bin/i686-linux-android22-clang"


[target.x86_64-linux-android]
ar = "/Users/xxx/Library/Android/sdk/ndk-bundle/toolchains/llvm/prebuilt/darwin-x86_64/bin/x86_64-linux-android-ar"
linker = "/Users/xxx/Library/Android/sdk/ndk-bundle/toolchains/llvm/prebuilt/darwin-x86_64/bin/x86_64-linux-android22-clang"
```

3. Run `tools/android-build.sh`, and check the DIR environment variable in `android-build.sh` too.    
