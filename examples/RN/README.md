# RN Example

This is React Native Demo Project

## requirements
iOS:
- Make sure you have Xcode installed (tested with Xcode 11.2.1).
- Make sure you have iOS simulators installed (tested with iPhone 11 Pro Max/iOS 13.2).
- Make sure you have node installed (tested with node 12.12).

Android:
- Make sure you have Android Studio installed(Android SDK、Android SDK Platform、Performance (Intel ® HAXM)、Android Virtual Device)
- Make sure you have Android SDK installed(tested with Android SDK Platform 28, Android 9 (Pie))
- Make sure you have created a Android Virtual Device(tested with Pixel_3_API_28)
- Make sure you have set the `$HOME/.bashrc` or `$HOME/.bash_profile`
```bash
export ANDROID_HOME=$HOME/Library/Android/sdk
export PATH=$PATH:$ANDROID_HOME/emulator
export PATH=$PATH:$ANDROID_HOME/tools
export PATH=$PATH:$ANDROID_HOME/tools/bin
export PATH=$PATH:$ANDROID_HOME/platform-tools
```

## generate the compiled files

In the root directory

- iOS:
```bash
make build-ios-rn-example
```

- Android:
```bash
make build-android-rn-example
```

## install

1. install node_modules
```bash
yarn install
```

2. install pod
```bash
(cd ios && pod install)
```
3. generate the Protocol Buffers
```bash
yarn pbjs
```

## run

- run iOS
```bash
yarn ios
```

- run Android
```bash
yarn android
```

## e2e
We use [Detox](https://github.com/wix/Detox) for e2e test.

### run iOS

1. install applesimutils

A collection of utils for Apple simulators, Detox uses it to communicate with the simulator.
```bash
brew tap wix/brew
brew install applesimutils
```

2. run e2e
```bash
yarn e2e
```

or run debug mode
```bash
yarn e2e:debug
```

###  run Android

```bash
yarn e2e:android
```

or run debug mode
```bash
yarn e2e:debug:android
```
