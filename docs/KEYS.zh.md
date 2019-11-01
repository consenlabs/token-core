# 公私钥以及与相关的派生

将公私钥与派生公钥结合在一起使用。公钥与私钥都具有派生能力, 私钥会实现 sign 方法来签名，如果同一种椭圆曲线，比如 curve25519 有不同的签名算法。则会采用不同的 Pair 实现。

## 类结构

### TypedKey 
usize 类型，主要是标记它的实际类型，不同的签名类型和椭圆曲线有不同的实现。比如曲线 curve25519 有两种签名算法 ed25519与 sr25519 有没的 TypedKey, 同样有不同的 Pair 实现。

### Pair

公私钥对，并且具有派生特性。Pair中保存了seed。如果是BIP32, seed前可以从私钥派生子 Pair, Pair也保存了相对应的 Public。

```
dervie() 派生 Pair
sign(message) -> Signature 签名
to_ss58check() -> str 转换成 base58 checker。便于持久化
from_ss58check(s58: &str) -> 从base58中读取
public() -> Public 取相对应的 Public
```

### Public
公钥, 公钥接口提供 as_ref() 转换成 [u8], 不同的椭圆曲线对公钥的保存方式是不一样的，比如 curve25519 就没有 uncompressed 与 compressed 的公钥区别, as_ref() 可以使用 Public::from_slice(data:&[u8]) 来生成公钥。

```
derive() 派生 Public
sign(message) -> Signature 签名
to_ss58Check() -> str 转换成 base58 checker。
from_ss58Check() -> Self 从 base58 创建 Public
```

### Derivation
派生处理，目前会实现 Bip32 与 Substrate

```
derive<Iter: Iteror<Item=DeriveJunction>(path: Iterator) -> Self (childkey, childcode)
```

### DeriveJunction
保存派生的连接信息，可以使用 DeriveJunction::Hard(1) 代表 harden index为1。

### 错误
所有的错误都是 enum， 有 PublicError, DeriveError, SecretError 几个。

### 持久化
Pair 会存成自已格式的 base58checker。 这个结构还未完全确定， 目前会是 [version, vec[key,chaincode]]

## FAQ
1. 为什么要采用自有的格式，而不是采用 BIP32。
因为 BIP32 的格式对不同的链需要注入 x_pub_version与 x_priv_version。这种方式会在 Chain 中注入。而且 substrate 的地址派生方式与 BIP32 不兼容。它可以支持A/Alice这种派生方式

2. 为什么要将派生算法签名融合到一个 Pair 结构中
因为 不同的签名对派生的要求不一样，比如 ed25519 就没有 public child key => public child key, 采用这种方式可以很容易报错

3. 从keystore获取Pair 或者 Public是否要指定类型
会，会采用 keystore.get_pair<T:Pair>(address) -> Result<T>，主要原因是 rust 不能将 impl Pair 转换为实际实现的  Pair 或者用 keystore.get_public<T:Public>(address) -> Result<T>, 获取 Public 的信息

4. 可以混用 bip32 与 substrate 的派生方式吗
可以，pair.derive(bip32).derive(substrate) 就可以向下派生。

5. 单条链支持不同的椭圆曲线算法怎么处理。
keystore.get_account(address) 来获取当时创建的 TypedKeyId。然后根据 TypedKeyId 用不同的再从 Keystore 里取。

6. 为什么TypedKeyId 不是 enum 型
主要是未来扩展方便，如果有完全不同的签名，可以在链的包里自已实现，0-2048 是我们系统保留。

## 例子

取 m/44'/194'/0'/0'/0/0 用secp256k1+ecdsa来签名

```rust
let pair = secp256k1::Pair::from_seed(seed);

let sign = pair.dervie(Derivation::from(Derivation::Bip32, "m/44'/194'/0'/0'/0/0")).sign(message)
```

取 substrate 派生算法用 ed25519来签名

```rust
let pair = ed25519::Pair::from_seed(seed);

let sign = pair.dervie(Derivation::from(Derivation::Substrate, "Test/Alice")).sign(message)
```

