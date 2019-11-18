use std::sync::RwLock;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BtcForkNetwork {
    pub coin: &'static str,
    pub hrp: &'static str,
    pub p2pkh_prefix: u8,
    pub p2sh_prefix: u8,
    pub xpub_prefix: [u8; 4],
    pub xprv_prefix: [u8; 4],
}

lazy_static! {
    static ref BTC_FORK_NETWORKS: RwLock<Vec<BtcForkNetwork>> = {
        let mut networks = Vec::new();
        networks.push(BtcForkNetwork {
            coin: "LITECOIN",
            hrp: "ltc",
            p2pkh_prefix: 0x30,
            p2sh_prefix: 0x32,
            xpub_prefix: [0x04, 0x88, 0xB2, 0x1E],
            xprv_prefix: [0x04, 0x88, 0xAD, 0xE4],
        });
        networks.push(BtcForkNetwork {
            coin: "LITECOIN-P2WPKH",
            hrp: "ltc",
            p2pkh_prefix: 0x30,
            p2sh_prefix: 0x32,
            xpub_prefix: [0x04, 0x88, 0xB2, 0x1E],
            xprv_prefix: [0x04, 0x88, 0xAD, 0xE4],
        });
        networks.push(BtcForkNetwork {
            coin: "LITECOIN-SEGWIT",
            hrp: "ltc",
            p2pkh_prefix: 0x30,
            p2sh_prefix: 0x32,
            xpub_prefix: [0x04, 0x88, 0xB2, 0x1E],
            xprv_prefix: [0x04, 0x88, 0xAD, 0xE4],
        });
        networks.push(BtcForkNetwork {
            coin: "LITECOIN-TESTNET",
            hrp: "ltc",
            p2pkh_prefix: 0x6f,
            p2sh_prefix: 0x3a,
            //            043587CF
            //            04358394
            xpub_prefix: [0x04, 0x35, 0x87, 0xCF],
            xprv_prefix: [0x04, 0x35, 0x83, 0x94],
        });
        networks.push(BtcForkNetwork {
            coin: "LITECOIN-TESTNET-P2WPKH",
            hrp: "ltc",
            p2pkh_prefix: 0x6f,
            p2sh_prefix: 0x3a,
            xpub_prefix: [0x04, 0x35, 0x87, 0xCF],
            xprv_prefix: [0x04, 0x35, 0x83, 0x94],
        });
        networks.push(BtcForkNetwork {
            coin: "BITCOIN",
            hrp: "bc",
            p2pkh_prefix: 0x0,
            p2sh_prefix: 0x05,
            xpub_prefix: [0x04, 0x88, 0xB2, 0x1E],
            xprv_prefix: [0x04, 0x88, 0xAD, 0xE4],
        });
        networks.push(BtcForkNetwork {
            coin: "BITCOIN-P2WPKH",
            hrp: "bc",
            p2pkh_prefix: 0x0,
            p2sh_prefix: 0x05,
            xpub_prefix: [0x04, 0x88, 0xB2, 0x1E],
            xprv_prefix: [0x04, 0x88, 0xAD, 0xE4],
        });
        networks.push(BtcForkNetwork {
            coin: "BITCOIN-SEGWIT",
            hrp: "bc",
            p2pkh_prefix: 0x0,
            p2sh_prefix: 0x05,
            xpub_prefix: [0x04, 0x88, 0xB2, 0x1E],
            xprv_prefix: [0x04, 0x88, 0xAD, 0xE4],
        });
        networks.push(BtcForkNetwork {
            coin: "BITCOIN-TESTNET",
            hrp: "bc",
            p2pkh_prefix: 0x6f,
            p2sh_prefix: 0xc4,
            xpub_prefix: [0x04, 0x35, 0x87, 0xCF],
            xprv_prefix: [0x04, 0x35, 0x83, 0x94],
        });
        //Definition of BitcoinCash networks https://github.com/bitpay/bitcore/blob/master/packages/bitcore-lib-cash/lib/networks.js#L168
        networks.push(BtcForkNetwork {
            coin: "BITCOINCASH",
            hrp: "bitcoincash",
            p2pkh_prefix: 0x0,
            p2sh_prefix: 0x05,
            xpub_prefix: [0x04, 0x88, 0xB2, 0x1E],
            xprv_prefix: [0x04, 0x88, 0xAD, 0xE4],
        });
        networks.push(BtcForkNetwork {
            coin: "BITCOINCASH-TESTNET",
            hrp: "bitcoincash",
            p2pkh_prefix: 0x6f,
            p2sh_prefix: 0xc4,
            xpub_prefix: [0x04, 0x35, 0x87, 0xCF],
            xprv_prefix: [0x04, 0x35, 0x83, 0x94],
        });
        RwLock::new(networks)
    };
}

// LTC address prefix: https://bitcoin.stackexchange.com/questions/62781/litecoin-constants-and-prefixes
// hrp: https://github.com/satoshilabs/slips/blob/master/slip-0173.md
// BTC https://en.bitcoin.it/wiki/List_of_address_prefixes

pub fn network_from_coin(coin: &str) -> Option<BtcForkNetwork> {
    let networks = BTC_FORK_NETWORKS.read().unwrap();
    let coin_uppercase = coin.to_uppercase();
    networks
        .iter()
        .find(|x| x.coin.eq(&coin_uppercase))
        .map(|x| x.clone())
}

pub fn network_form_hrp(hrp: &str) -> Option<BtcForkNetwork> {
    match hrp {
        "bitcoincash" => network_from_coin("BITCOINCASH"),
        "ltc" => network_from_coin("LITECOIN-SEGWIT"),
        "bc" => network_from_coin("BITCOIN-SEGWIT"),
        _ => None,
    }
}

pub fn coin_from_xpub_prefix(prefix: &[u8]) -> Option<String> {
    let networks = BTC_FORK_NETWORKS.read().unwrap();
    networks
        .iter()
        .find(|x| x.xpub_prefix.eq(prefix))
        .map(|x| x.coin.to_string())
}

pub fn coin_from_xprv_prefix(prefix: &[u8]) -> Option<String> {
    let networks = BTC_FORK_NETWORKS.read().unwrap();
    networks
        .iter()
        .find(|x| x.xprv_prefix.eq(prefix))
        .map(|x| x.coin.to_string())
}
