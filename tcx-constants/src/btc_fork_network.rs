use crate::CoinInfo;
use parking_lot::RwLock;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BtcForkNetwork {
    pub coin: &'static str,
    pub network: &'static str,
    pub seg_wit: &'static str,
    pub hrp: &'static str,
    pub p2pkh_prefix: u8,
    pub p2sh_prefix: u8,
    pub private_prefix: u8,
    pub xpub_prefix: [u8; 4],
    pub xprv_prefix: [u8; 4],
}

pub struct HdVersion {
    pub_version: [u8; 4],
    prv_version: [u8; 4],
}

lazy_static! {
    static ref BTC_FORK_NETWORKS: RwLock<Vec<BtcForkNetwork>> = {
        let mut networks = Vec::new();
        networks.push(BtcForkNetwork {
            coin: "LITECOIN",
            network: "MAINNET",
            seg_wit: "NONE",
            hrp: "",
            p2pkh_prefix: 0x30,
            p2sh_prefix: 0x32,
            private_prefix: 0xb0,
            xpub_prefix: [0x04, 0x88, 0xB2, 0x1E],
            xprv_prefix: [0x04, 0x88, 0xAD, 0xE4],
        });
        networks.push(BtcForkNetwork {
            coin: "LITECOIN",
            network: "MAINNET",
            seg_wit: "P2WPKH",
            hrp: "",
            p2pkh_prefix: 0x30,
            p2sh_prefix: 0x32,
            private_prefix: 0xb0,
            xpub_prefix: [0x04, 0x88, 0xB2, 0x1E],
            xprv_prefix: [0x04, 0x88, 0xAD, 0xE4],
        });
        networks.push(BtcForkNetwork {
            coin: "LITECOIN",
            network: "MAINNET",
            seg_wit: "SEGWIT",
            hrp: "ltc",
            p2pkh_prefix: 0x30,
            p2sh_prefix: 0x32,
            private_prefix: 0xb0,
            xpub_prefix: [0x04, 0x88, 0xB2, 0x1E],
            xprv_prefix: [0x04, 0x88, 0xAD, 0xE4],
        });
        networks.push(BtcForkNetwork {
            coin: "LITECOIN",
            network: "TESTNET",
            seg_wit: "NONE",
            hrp: "",
            p2pkh_prefix: 0x6f,
            p2sh_prefix: 0x3a,
            private_prefix: 0xef,
            //            043587CF
            //            04358394
            xpub_prefix: [0x04, 0x35, 0x87, 0xCF],
            xprv_prefix: [0x04, 0x35, 0x83, 0x94],
        });
        networks.push(BtcForkNetwork {
            coin: "LITECOIN",
            network: "TESTNET",
            seg_wit: "P2WPKH",
            hrp: "",
            p2pkh_prefix: 0x6f,
            p2sh_prefix: 0x3a,
            private_prefix: 0xef,
            xpub_prefix: [0x04, 0x35, 0x87, 0xCF],
            xprv_prefix: [0x04, 0x35, 0x83, 0x94],
        });
        networks.push(BtcForkNetwork {
            coin: "BITCOIN",
            network: "MAINNET",
            seg_wit: "NONE",
            hrp: "",
            p2pkh_prefix: 0x0,
            p2sh_prefix: 0x05,
            private_prefix: 0x80,
            xpub_prefix: [0x04, 0x88, 0xB2, 0x1E],
            xprv_prefix: [0x04, 0x88, 0xAD, 0xE4],
        });
        networks.push(BtcForkNetwork {
            coin: "BITCOIN",
            network: "MAINNET",
            seg_wit: "P2WPKH",
            hrp: "",
            p2pkh_prefix: 0x0,
            p2sh_prefix: 0x05,
            private_prefix: 0x80,
            xpub_prefix: [0x04, 0x88, 0xB2, 0x1E],
            xprv_prefix: [0x04, 0x88, 0xAD, 0xE4],
        });
        networks.push(BtcForkNetwork {
            coin: "BITCOIN",
            network: "MAINNET",
            seg_wit: "SEGWIT",
            hrp: "bc",
            p2pkh_prefix: 0x0,
            p2sh_prefix: 0x05,
            private_prefix: 0x80,
            xpub_prefix: [0x04, 0x88, 0xB2, 0x1E],
            xprv_prefix: [0x04, 0x88, 0xAD, 0xE4],
        });
        networks.push(BtcForkNetwork {
            coin: "BITCOIN",
            network: "TESTNET",
            seg_wit: "NONE",
            hrp: "",
            p2pkh_prefix: 0x6f,
            p2sh_prefix: 0xc4,
            private_prefix: 0xef,
            xpub_prefix: [0x04, 0x35, 0x87, 0xCF],
            xprv_prefix: [0x04, 0x35, 0x83, 0x94],
        });
        networks.push(BtcForkNetwork {
            coin: "BITCOIN",
            network: "TESTNET",
            seg_wit: "P2WPKH",
            hrp: "",
            p2pkh_prefix: 0x6f,
            p2sh_prefix: 0xc4,
            private_prefix: 0xef,
            xpub_prefix: [0x04, 0x35, 0x87, 0xCF],
            xprv_prefix: [0x04, 0x35, 0x83, 0x94],
        });
        //Definition of BitcoinCash networks https://github.com/bitpay/bitcore/blob/master/packages/bitcore-lib-cash/lib/networks.js#L168
        networks.push(BtcForkNetwork {
            coin: "BITCOINCASH",
            network: "MAINNET",
            seg_wit: "NONE",
            hrp: "bitcoincash",
            p2pkh_prefix: 0x0,
            p2sh_prefix: 0x05,
            private_prefix: 0x80,
            xpub_prefix: [0x04, 0x88, 0xB2, 0x1E],
            xprv_prefix: [0x04, 0x88, 0xAD, 0xE4],
        });
        networks.push(BtcForkNetwork {
            coin: "BITCOINCASH",
            network: "TESTNET",
            seg_wit: "NONE",
            hrp: "bitcoincash",
            p2pkh_prefix: 0x6f,
            p2sh_prefix: 0xc4,
            private_prefix: 0xef,
            xpub_prefix: [0x04, 0x35, 0x87, 0xCF],
            xprv_prefix: [0x04, 0x35, 0x83, 0x94],
        });
        RwLock::new(networks)
    };

    static ref HD_VERSIONS: RwLock<Vec<HdVersion>> = {
        let mut versions = Vec::new();
        versions.push(HdVersion {
            pub_version: [0x04, 0x88, 0xB2, 0x1E],
            prv_version: [0x04, 0x88, 0xAD, 0xE4],
        });
        versions.push(HdVersion {
            pub_version: [0x04, 0x35, 0x87, 0xCF],
            prv_version: [0x04, 0x35, 0x83, 0x94],
        });
        RwLock::new(versions)
    };
}

// LTC address prefix: https://bitcoin.stackexchange.com/questions/62781/litecoin-constants-and-prefixes
// hrp: https://github.com/satoshilabs/slips/blob/master/slip-0173.md
// BTC https://en.bitcoin.it/wiki/List_of_address_prefixes

pub fn network_from_coin(coin_info: &CoinInfo) -> Option<BtcForkNetwork> {
    network_from_param(&coin_info.coin, &coin_info.network, &coin_info.seg_wit)
}

pub fn network_from_param(
    chain_type: &str,
    network: &str,
    seg_wit: &str,
) -> Option<BtcForkNetwork> {
    let networks = BTC_FORK_NETWORKS.read();
    //    let coin_uppercase = coin.to_uppercase();
    let mut ret: Vec<BtcForkNetwork> = networks
        .iter()
        .filter(|x| x.coin.eq(&chain_type.to_uppercase()))
        .filter(|x| x.network.eq(&network.to_uppercase()))
        .filter(|x| x.seg_wit.eq(&seg_wit.to_uppercase()))
        .map(|x| x.clone())
        .collect::<Vec<BtcForkNetwork>>();
    ret.pop()
}

pub fn network_form_hrp(hrp: &str) -> Option<BtcForkNetwork> {
    let networks = BTC_FORK_NETWORKS.read();
    let mut ret: Vec<BtcForkNetwork> = networks
        .iter()
        .filter(|x| x.hrp.eq(hrp))
        .map(|x| x.clone())
        .collect::<Vec<BtcForkNetwork>>();
    ret.pop()
}

pub fn coin_from_xpub_prefix(prefix: &[u8]) -> Option<String> {
    let networks = BTC_FORK_NETWORKS.read();
    networks
        .iter()
        .find(|x| x.xpub_prefix.eq(prefix))
        .map(|x| x.coin.to_string())
}

pub fn pub_version_from_prv_version(prefix: &[u8]) -> Option<[u8; 4]> {
    let networks = HD_VERSIONS.read();
    networks.iter().find(|x| x.prv_version.eq(prefix)).map(|x| {
        let mut version = [0; 4];
        version.copy_from_slice(&x.pub_version);
        version
    })
}
