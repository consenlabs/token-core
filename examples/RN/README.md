# RN Example

This is React Native Demo Project

## requirements
- Make sure you have Xcode installed (tested with Xcode 11.2.1).
- Make sure you have iOS simulators installed (tested with iPhone 11 Pro Max/iOS 13.2).
- Make sure you have node installed (tested with node 12.12).

## generate the compiled files

In the root directory

- iOS:
```bash
build-ios-rn-example
```

- Android:
```bash
build-android-rn-example
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

1. install applesimutils

A collection of utils for Apple simulators, Detox uses it to communicate with the simulator.
```bash
brew tap wix/brew
brew install applesimutils
```

- run iOS
```bash
yarn e2e
```

or debug
```bash
yarn e2e:debug
```

- run Android
```bash
yarn e2e:android
```

or debug
```bash
yarn e2e:debug:android
```
