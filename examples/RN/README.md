# RN Example

This is React Native Demo Project

## Requirements
- Make sure you have Xcode installed (tested with Xcode 11.2.1).
- Make sure you have iOS simulators installed (tested with iPhone 11 Pro Max/iOS 13.2).
- Make sure you have node installed (tested with node 12.12).

## generate the compiled files

```
./tools/ios-example-build.sh
```

## install

1. install node_modules
```
yarn install
```

2. install pod
```bash
(cd ios && pod install)
```

## run

- run iOS
```
yarn ios
```

- run Android
```
yarn android
```

## e2e
We use [Detox](https://github.com/wix/Detox) for e2e test.

1. install applesimutils

A collection of utils for Apple simulators, Detox uses it to communicate with the simulator.
```
brew tap wix/brew
brew install applesimutils
```

2. run
```
yarn e2e
```
