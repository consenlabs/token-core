# Add Blockchain Integration

How many code you have to add this repository

## Cryptograph

Now we only support secp256k1 curve, and will add other curve soon. And import any hasher rust package as you need.

## Chain Functions

### Trait to implement

Add new package, like `tcx-a-chain`, implement following interfaces:

| Trait Name                                         | Package   | Responbility                                         |
| -------------------------------------------------- | --------- | ---------------------------------------------------- |
| [Address](tcx-chain/src/keystore/mod.rs)           | tcx-chain | Convert public key to                                |
| [ChainSigner](tcx-chain/src/signer.rs)             | tcx-chain | Take binary data to do ecc signing                   |
| [TransactionSigner](tcx-chain/src/signer.rs)       | tcx-chain | Take input & output to do chain-specify data signing |
| [MessageSigner](tcx-chain/src/signer.rs)(Optional) | tcx-chain | Take input & output to do chain-specify data signing |

### Chain Wallet Spec

Declare blockchain wallet information to [coin_info.rs](tcx-constans/src/coin_info.rs) like following,

```rust
coin_infos.push(CoinInfo {
    coin: "BITCOIN".to_string(), // Chain full name
    derivation_path: "m/44'/0'/0'/0/0".to_string(), // BIP44 or SLIP44 path
    curve: CurveType::SECP256k1, // Curve type, now only support secp256k1
    network: "MAINNET".to_string(), // Network corresponding to path above
    seg_wit: "NONE".to_string(), // Segwit type, options is ['', 'NONE', 'P2WPKH']
});
coin_infos.push(CoinInfo {
    coin: "BITCOIN".to_string(),
    derivation_path: "m/44'/1'/0'/0/0".to_string(),
    curve: CurveType::SECP256k1,
    network: "TESTNET".to_string(),
    seg_wit: "NONE".to_string(),
});
```

### Transaction Input & Output

Define transaction input & output's data structure in Protobuf file.

```protobuf
// file: btc_fork.proto
// FUNCTION: sign_tx(SignParam{input: BtcForkTxInput}): BtcForkSignedTxOutput
message Utxo {
    string txHash = 1;
    int32 vout = 2;
    int64 amount = 3;
    string address = 4;
    string scriptPubKey = 5;
    string derivedPath = 6;
    int64 sequence = 7;
}

message BtcForkTxInput {
    string to = 1;
    int64 amount = 2;
    repeated Utxo unspents = 3;
    int64 fee = 4;
    uint32 changeAddressIndex = 5;
    string changeAddress = 6;
    string network = 7;
    string segWit = 8;
}

message BtcForkSignedTxOutput {
    string signature = 1;
    string txHash = 2;
}
```

## Testing

As rust community convention, developer should add inline unit testcases after the functions. And at lease including
happy case and common exception handling.
