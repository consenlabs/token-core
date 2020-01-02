# Security Policy

## Introduction
TokenCore is a cross-platform library that implements crypto wallet functions for blockchains, exporting c interfaces in Protobuf protocol. This library is totally written in Rust, and now provides friendly interfaces for the mobile platform including ReactNative, iOS, and Android.

imToken embed TokenCore library as the low-level cryptograph wallet layer and built the build user interface on this library.

If you are new to the imToken wallet, please visit [our product help center](https://support.token.im/hc/en-us) before submitting reports.

## Bounty Scope
TokenCore Bug Bounty is only based on TokenCore open-sourced code, the GitHub repository is https://github.com/consenlabs/token-core.

The following are what we are interested in:

- Vulnerabilities that can steal assets or cause loss of assets
- Defects in core encryption algorithm implementation, such as Keystore, Wallet Generation, Transaction Signature, etc.
- Vulnerabilities in chain-related logic code
- Vulnerabilities in the wallet application layer
- Vulnerabilities that can cause software unavailability, such as App crashes, etc.
- Insecure and irregular code implementations
- Vulnerability messages for third-party libraries

The following are out of scope:

- Anything that isn’t in this repository
- 3rd party library dependencies
- Example code for demonstrating 

Please also note that any bugs already reported are considered out of scope.

## Severity and rewards
The severity of reported vulnerabilities will be graded according to CVSS (Common Vulnerability Scoring Standard, https://www.first.org/cvss). The following table will serve as a guideline for reward decisions:

| Vulnerability Tier         | Reward            |
| -------------------------- | ----------------- |
| Critical (CVSS 9.0 - 10.0) | 5000 - 10000 USDT |
| High (CVSS 6.0 - 8.9)      | 1000 - 5000 USDT  |
| Medium (CVSS 4.0 - 5.9)    | 500 - 1000 USDT   |
| Low (CVSS 0.1 - 3.9)       | 0 - 500 USDT      |

The reward we pay is in [Tether USD token](https://etherscan.io/token/0xdac17f958d2ee523a2206206994597c13d831ec7) on ethereum, so please prepare your ethereum wallet address in advance.

## Guidelines for Crafting a Report
Contact us, sending vulnerability details to email <sec@token.im>

Report requirements:

- Vulnerability title and tier
- Description of the vulnerability 
- PoC(e.g. Sample code, screenshot, video)
- Suggestion for how to fix (optional)
- Vulnerability is basically confirmed to be valid or not within two business days after submission. After confirmation and grading, the reward will be issued to your wallet address within two weeks.
- Do not publicly disclose your submission until imToken evaluated the impact.
