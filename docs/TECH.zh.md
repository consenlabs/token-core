# Token Core X  
Token Core X 旨在开发专为多链设计的跨平台的方案。

## 设计说明
### 软件钱包
Token Core X(以下简称tcx)主要提供钱包管理和签名功能，钱包管理包括生成，基本信息展示，导出，删除等操作，签名功能包括转账签名，消息体签名等。    
tcx统计了常见的链，每条链都都会    
1. 包含一个唯一的名称，如BCH,ETH等
2. 符合Bip32或者其他确定性钱包的派生方案，例如BCH符合bip32
3. 包含一种或多种签名算法，例如BCH使用Secp256k1曲线
4. 包含部分自己特有的信息，例如BCH会含有encXPub, externalAddress等
5. 包含自己特有的地址生产算法
6. 包含自己特有的transaction序列化方式等    
软件钱包的所有设计都是围绕的助记词推导的Seed，tcx平衡了灵活性和接入方便程度提供了通用的 HdKeystore 结构体用来加密保存助记词，提供了Bip32等派生方案以及内置了部分曲线，并且提供了方便的机制可以用于扩展每条链的地址生成方式和交易签名逻辑。下面详解了tcx的关键部分

### 钱包设计详解

#### CoinInfo & HdKeystore
软件钱包的多链设计主要依赖于Bip39(助记词推导seed)以及Bip32,Bip44等其他确定性钱包的派生方案。    
tcx 提供了 HdKeystore 结构体，用以提供助记词的加密存储及seed推导。你可以通过`HdKeystore::new(password: &str, meta: Metadata)`和`HdKeystore::from_mnemonic(mnemonic: &str, password: &str, meta: Metadata)`生成Keystore      

根据确定性钱包方案，你可以在同一个Seed上根据不同的Path派生出不同的私钥，每组私钥用户不同的币种。      
在tcx中使用CoinInfo结构体用以定义链的名称，所使用的曲线以及所在的路径。 HdKeystore 提供了 `derive_coin` 方法，可以根据你在CoinInfo中所定义的曲线和路径派生出相应的信息。       
派生出的信息将存储在 HdKeystore 中的activeAccounts列表中。派生的信息将存储为Account结构体。       
Account结构体用来存储和链常用的信息，例如地址，所在曲线，派生路径，部分链还会包含额外的一些自己特有的信息。下面展示了bch的Account    
```json
{
    "address": "bitcoincash:qqyta3mqzeaxe8hqcdsgpy4srwd4f0fc0gj0njf885",
    "derivationPath": "m/44'/145'/0'",
    "curve": "SECP256k1",
    "coin": "BITCOINCASH",
    "extra": {
        "encXPub": "wAKUeR6fOGFL+vi50V+MdVSH58gLy8Jx7zSxywz0tN++l2E0UNG7zv+R1FVgnrqU6d0wl699Q/I7O618UxS7gnpFxkGuK0sID4fi7pGf9aivFxuKy/7AJJ6kOmXH1Rz6FCS6b8W7NKlzgbcZpJmDsQ==",
        "externalAddress": {
            "address": "bitcoincash:qzyrtfn4a7cdkn7sp60tw7hl8zndt0tk0sst3p6qr5",
            "derivedPath": "0/1",
            "type": "EXTERNAL"
        }
    }
}
```    

实际上CoinInfo只提供了链所在曲线和派生路径，HdKeystore内部也只提供了常见曲线以及根据路径派生私钥的方案。所以地址信息和链特有信息需要其他方式提供。为此tcx提供了`Address`和`Extra`两类Trait，并通过泛型的方式这两类信息注入到钱包派生的过程中。如下展示了HdKeystore提供的派生的完整的函数声明      
`derive_coin<A: Address, E: Extra>(&mut self, coin_info: &CoinInfo, password: &str) -> Result<(&Account, Extra)>`    

#### Address  
tcx 提供了`Address`这个trait，链开发者可以实现这个`Address::from_public_key`方法来实现自己特定的通过公钥推导地址的计算逻辑。在HdKeystore::derive_coin内部会将这部分逻辑注入到推导过程中，如下所示：    
```rust
    fn derive_account_from_coin<A: Address, E: Extra>(
        coin_info: &CoinInfo,
        seed: &Seed,
    ) -> Result<(Account, E)> {
        ...
        let pub_key = key.public_key();
        let address = A::from_public_key(&pub_key, Some(&coin_info.coin))?;
        ...
    }
```    
因为该函数只接受pub_key和可选Coin参数，方便区分一条链上fork出的多个币种，例如btc系列会在不同的币种区分p2pkh header及p2sh header等，BtcForkAddress根据Coin不同会有不同的配置。如下所示    
```rust

pub fn network_from_coin(coin: &str) -> Option<BtcForkNetwork> {
    match coin.to_lowercase().as_str() {
        "ltc" => Some(BtcForkNetwork {
            coin: "LITECOIN",
            hrp: "ltc",
            p2pkh_prefix: 0x30,
            p2sh_prefix: 0x32,
            xpub_prefix: [0x04, 0x88, 0xB2, 0x1E],
            xprv_prefix: [0x04, 0x88, 0xAD, 0xE4],
            fork_id: 0,
        }),
        "ltc-testnet" => Some(BtcForkNetwork {
            coin: "LITECOIN-TESTNET",
            hrp: "ltc",
            p2pkh_prefix: 0x6f,
            p2sh_prefix: 0x3a,
            xpub_prefix: [0x04, 0x88, 0xB2, 0x1E],
            xprv_prefix: [0x04, 0x88, 0xAD, 0xE4],
            fork_id: 0,
        }),
        ...
        "bitcoincash" | "bch" => Some(BtcForkNetwork {
            coin: "BITCOINCASH",
            hrp: "bitcoincash",
            p2pkh_prefix: 0x0,
            p2sh_prefix: 0x05,
            xpub_prefix: [0x04, 0x88, 0xB2, 0x1E],
            xprv_prefix: [0x04, 0x88, 0xAD, 0xE4],
            fork_id: 0x40,
        }),
        _ => None,
    }
}
...
impl Address for BtcForkAddress {
    fn from_public_key(public_key: &impl PublicKey, coin: Option<&str>) -> Result<String> {
        let pub_key = Secp256k1PublicKey::from_slice(&public_key.to_bytes())?;
        let coin = coin.expect("coin from address_pub_key");
        let network = network_from_coin(&coin);
        tcx_ensure!(network.is_some(), Error::UnsupportedChain);
        let network = network.expect("network");
        let addr = match coin.to_lowercase().as_str() {
            "bch" => {
                let legacy = BtcForkAddress::p2pkh(&pub_key, &network)?;
                let converter = Converter::new();
                converter
                    .to_cash_addr(&legacy.to_string())
                    .map_err(|_| Error::ConvertToCashAddressFailed(legacy.to_string()))
            }
            "ltc" | "btc" | "ltc-testnet" => {
                Ok(BtcForkAddress::p2shwpkh(&pub_key, &network)?.to_string())
            }
            _ => Err(Error::UnsupportedChain),
        }?;
        Ok(addr.to_string())
    }
}
```

#### Extra    
因为对于每天链来说除了Account中定义的常见的address，cruve等必须属性，部分链还提供了额外信息，，例如bch还会包含enc_xpub和externalAddress等信息。有这部分需求的开发者可以实现Extra这个trait来扩展。例如Bch的实现如下:     
```rust
pub struct ExtendedPubKeyExtra {
    pub enc_xpub: String,
    pub external_address: ExternalAddress,
}

impl Extra for ExtendedPubKeyExtra {
    // 注：此处的实现本不该将seed的概念暴露在HdKeystore类型之外，但是计算xpub确实需要seed，并且HdKeystore内部为了安全并未缓存Seed，所以无法将此类型改成impl Extra for HdKeystore
    fn from(coin_info: &CoinInfo, seed: &Seed) -> Result<Self> {
        ...
        let derivation_info = Secp256k1Curve::extended_pub_key(&coin_info.derivation_path, &seed)?;
        let xpub = address::BtcForkAddress::extended_public_key(&derivation_info);
        calc_external_address
        ...
    }
}
```
由于Extra需要存储的数据字段过于灵活，最终extra信息会被序列化成json存入Account的extra字段中。该计算逻辑也会被放置到derive_coin的推导过程中，如下所示：    
```rust
fn derive_account_from_coin<A: Address, E: Extra>(
        coin_info: &CoinInfo,
        seed: &Seed,
    ) -> Result<(Account, E)> {
        ...
        let extra = E::from(coin_info, seed)?;
        let acc = Account {
            ...
            extra: serde_json::to_value(extra.clone()).expect("extra_error"),
        };
        ...
    }
```
对于部分没有额外信息的链，我们提供了EmptyExtra，开发者可直接使用该结构体。    

#### Curve
在tcx中提供了PrivateKey和PublicKey两个trait，PrivateKey提供了推导同曲线的公钥和所在曲线的签名实现。每条曲线的公私钥都应该实现这些trait。HdKeystore部分接口会返回这些Trait，直接使用Trait在部分情况下可以避免你转换成具体的类型。例如在签名时通过调用`key_at_paths(&self, symbol: &str, paths: &[impl AsRef<str>], password: &str) -> Result<Vec<impl PrivateKey>>`该方法会根据symbol找到相应的Account并确定其所在的曲线。最终返回的私钥可以直接调用其sign方法而不需要考虑曲线，私钥转换等问题。(目前正在讨论一种更高抽象的一种方案，可以参见tcx-primitive包，不过暂未确定具体思路暂不讨论)     
由于曲线的设计不会暴露给普通的链的开发者，并且每条链的特性不尽相同。所以tcx在内部提供了多个具体的结构体来实现曲线相关的算法，例如`Secp256k1Curve`，其内部除了提供了根据seed和path派生相应私钥的方法。并且在HdKeystore内部根据传入的CoinInfo来调用相关方法，如下所示    
```rust
fn key_at_paths_with_seed(
        curve: CurveType,
        paths: &[impl AsRef<str>],
        seed: &Seed,
    ) -> Result<Vec<impl PrivateKey>> {
        match curve {
            CurveType::SECP256k1 => Secp256k1Curve::key_at_paths_with_seed(paths, seed),
            _ => Err(Error::UnsupportedCurve.into()),
        }
    }
```
最初的设计中所有的曲线相关的操作全部放在各个Chain之中，Keystore只包含seed的存储。但是考虑到1）常见链一共就四五条，2.每个Chain实现一遍相似模板代码会过多。所以现在改现在的由CoinInfo定义曲线链所在的曲线，派生私钥等过程在Keystore根据定义的曲线内部判断处理。    
一般情况下用户是无需直接使用`Secp256k1Curve`等结构体，但部分特殊情况下，例如Bch Account的extra部分包含xpub，在其明确知道自己意图之后可以使用`Secp256k1Curve`提供的公开方法。如开发者需要tcx不支持的曲线可以和我们内部沟通由我们共同添加。极端特殊情况下，开发者可使用`HdKesytore::seed(password: &str)`方法导出Seed进行完全自定义的开发。    

#### Transaction
tcx提供了`TransactionSigner<Input: Transaction, Output: SignedTransaction>`每条链可实现该trait添加自己的签名实现。    
其中`Transaction`和`SignedTransaction`属于Maker Trait, 你可以实现包含任意字段的结构体并标明他实现了Transaction，即可用来作为签名输入输入束。对于btc家族的链，在其内部定义了`BitcoinForkTransaction`,其中包含了btc系列转账所签名所需的输入信息, 如下所示：    
```rust
pub struct BitcoinForkTransaction {
    pub to: String,
    pub amount: i64,
    pub unspents: Vec<Utxo>,
    pub memo: String,
    pub fee: i64,
    pub change_idx: u32,
}

impl TraitTransaction for BitcoinForkTransaction {}
```
TransactionSigner提供了签名接口的约束。开发者需要实现该Trait并加入自己所在链的签名实现。参见bch的实现：    
```rust
// 感谢@孙哥提到 impl ... for HdKeystore 思路，可以对于硬件特定的实现可以使用impl ... for HdWallet,你会看到tcx中的Presenter使用了同样的思路。但是该方案同时限制死了软件钱包和硬件钱包必须使用相同的接口，导致Extra暂时不行能用该方案    
impl TransactionSigner<BitcoinForkTransaction, TxSignResult> for HdKeystore {
    fn sign(&self, tx: &BitcoinForkTransaction, password: Option<&str>) -> Result<TxSignResult> {
        let account = self
            .account(&"BITCOINCASH")
            .ok_or(format_err!("account_not_found"))?;
        let path = &account.derivation_path;
        let extra = ExtendedPubKeyExtra::from(account.extra.clone());

        let paths = tx.collect_key_pairs_paths(path)?;
        tcx_ensure!(password.is_some(), tcx_crypto::Error::InvalidPassword);
        let priv_keys = &self.key_at_paths("BITCOINCASH", &paths, password.unwrap())?;
        let xpub = extra.xpub()?;
        tx.sign_transaction(&priv_keys, &xpub)
    }
}
```
SignedTransaction 用以定义签名的输出物，tcx默认提供了TxSignResult作为输出的一个默认实现，如果的你的签名结果比较简单，你可以采用该实现。    

#### 接口层   
目前接口层主要是面向 imToken 功能设计，提供了标准的C导出接口。   
对于需要集成到imToken的链需要在此编写部分代码。例如常见的 `import_wallet_from_mnemonic` 和 `sign_transaction`。如下展示了bch的部分实现   
```rust
fn _import_wallet_from_mnemonic(v: &Value) -> Result<String> {
    // parse some arguments
    let mut ks = HdKeystore::from_mnemonic(mnemonic, password, meta);
    let mut pw = Map::new();

    let (account, extra) = match chain_type {
            "BITCOINCASH" | "LITECOIN" | "LITECOIN-TESTNET" => {
                let mut coin_info = _coin_info_from_symbol(chain_type)?;
                coin_info.derivation_path = path.to_string();
                ks.derive_coin::<BtcForkAddress, ExtendedPubKeyExtra>(&coin_info, password)
            }
            _ => Err(format_err!("{}", "chain_type_not_support")),
        }?;

    // check exists
    ...

    _flush_keystore(&ks)?;
    let json = ks.present();
    cache_keystore(ks);

    json
}
```

```rust
fn _sign_transaction(json_str: &str) -> Result<String> {
    let v: Value = serde_json::from_str(json_str).unwrap();
    // parse arguments and get keystore
    match chain_type {
        "BITCOINCASH" | "LITECOIN" | "LITECOIN-TESTNET" => _sign_btc_fork_transaction(json_str, keystore, password),
        _ => Err(format_err!("{}", "chain_type_not_support")),
    }
}

fn _sign_btc_fork_transaction(json: &str, keystore: &HdKeystore, password: &str) -> Result<String> {
    let v: Value = serde_json::from_str(json).expect("sign_transaction_json");
    // parse arguments
    let tran = BitcoinForkTransaction {
        to: to.to_owned(),
        amount,
        unspents,
        memo: "".to_string(),
        fee,
        change_idx: change_idx as u32,
        coin: chain_type,
        is_seg_wit,
    };
    let ret = keystore.sign_transaction(&tran, Some(&password))?;
    Ok(serde_json::to_string(&ret)?)
}
```
目前还需要手动定义CoinInfo，并且在以上两个函数内部会有一个match pattern来根据链区别实现Coin。后续会考虑使用宏或者代码生成简化该模板代码    



### 硬件钱包（略）    

## 项目架构说明    
该项目除了必要的 Rust 代码还包含了部分示例以及多平台编译工具。各自分散在不同的目录下，如下描述了个目录的作用。其中所有相关的 rust 的包均以tcx-为前缀。     
`docs`: 文档目录    
`examples`: 示例目录，包含了 Android 和 iOS 等演示程序    
`tools`: Android 和 iOS 跨平台编译工具   
`test-data`: 单元测试用例需要的文件   

`tcx-crypto`: 相关加密算法，例如 aes, kdf及pbkdf2等具体的实现     
`tcx-primitive(暂未启用)`: 对于私钥和Bip32及 substrate等派生方案的封装。    
`tcx-chain`: 包含了所有和链相关的实现及Trait约束，如公私钥封装，曲线，地址，签名约束以及Keystore等。    
`tcx`: Token Core X(下简称tcx) 软件钱包层接口。用于适配当前 imToken 2.5 所有相关接口    
`tcx-`: 其他以 tcx 开头的包均为各个链的具体实现    


`generated(暂未启用)`: 代码生成目录    
`trezor-crypto(暂未启用)`: Trezor 提供的各种加密库，由 C 语言编写    


## 新链接入（简略版）
1. 添加一个新的package，命名为`tcx-blockchain`其中的blockchain用链的名称替换，将如下代码写入该package里面   
2. 定义CoinInfo, CoinInfo可以指定你要添加的链的名称，所在的曲线以及助记词的派生路径。    
3. 可选）如果你所添加的链包含有特殊的需要保存的信息，你需要实现你自己的`tcx_chain::Extra`。该接口用于根据`CoinInfo`和`Seed`生成特定信息，最终该信息会被序列化成JSON数据存储在account中的extra字段内。     
4. 实现tcx_chain::Address，添加你所在的链的地址推导方法，分别添加判断地址是否有效，根据pub_key生成地址    
5. 实现tcx_chain::Transaction，添加你签名是需要输入的字段， 实现tcx_chain::SignedTransaction，添加你签名结束后的输出内容。如果的的签名结果比较简单，你可以使用tcx默认提供的TxSignResult结构。   
6. 实现tcx_chain::TransactionSigner，添加你所在的链特定的签名算法    
7. 在tcx层添加match pattern 指向正确的处理逻辑    
