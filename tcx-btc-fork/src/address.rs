use bitcoin::network::constants::Network;
use bitcoin::util::address::Payload;
use bitcoin::util::base58;
use bitcoin::{Address, PrivateKey, PublicKey};
use secp256k1::{Secp256k1, SecretKey};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BtcForkAddress {
    pub network: BtcForkNetwork,
    pub payload: Payload,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BtcForkNetwork {
    pub desc: &'static str,
    pub hrp: &'static str,
    pub p2pkh_prefix: u8,
    pub p2sh_prefix: u8,
    pub xpub_prefix: [u8; 4],
    pub xprv_prefix: [u8; 4],
}

impl BtcForkAddress {
    pub fn p2pkh(pub_key: &PublicKey, network: &BtcForkNetwork) -> BtcForkAddress {
        let addr = Address::p2pkh(pub_key, Network::Bitcoin);
        BtcForkAddress {
            payload: addr.payload,
            network: network.clone(),
        }
    }

    pub fn p2shwpkh(pub_key: &PublicKey, network: &BtcForkNetwork) -> BtcForkAddress {
        let addr = Address::p2shwpkh(pub_key, Network::Bitcoin);
        BtcForkAddress {
            payload: addr.payload,
            network: network.clone(),
        }
    }

    pub fn p2wpkh(pub_key: &PublicKey, network: &BtcForkNetwork) -> BtcForkAddress {
        let addr = Address::p2wpkh(pub_key, Network::Bitcoin);
        BtcForkAddress {
            payload: addr.payload,
            network: network.clone(),
        }
    }
}

impl Display for BtcForkAddress {
    fn fmt(&self, fmt: &mut Formatter) -> core::fmt::Result {
        match self.payload {
            Payload::PubkeyHash(ref hash) => {
                let mut prefixed = [0; 21];
                prefixed[0] = self.network.p2pkh_prefix;
                prefixed[1..].copy_from_slice(&hash[..]);
                base58::check_encode_slice_to_fmt(fmt, &prefixed[..])
            }
            Payload::ScriptHash(ref hash) => {
                let mut prefixed = [0; 21];
                prefixed[0] = self.network.p2sh_prefix;
                prefixed[1..].copy_from_slice(&hash[..]);
                base58::check_encode_slice_to_fmt(fmt, &prefixed[..])
            }
            Payload::WitnessProgram {
                version: ver,
                program: ref prog,
            } => {
                let hrp = self.network.hrp;
                let mut bech32_writer = bech32::Bech32Writer::new(hrp, fmt)?;
                bech32::WriteBase32::write_u5(&mut bech32_writer, ver)?;
                bech32::ToBase32::write_base32(&prog, &mut bech32_writer)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn ltc_address_test() {
        // LTC address prefix: https://bitcoin.stackexchange.com/questions/62781/litecoin-constants-and-prefixes
        // hrp: https://github.com/satoshilabs/slips/blob/master/slip-0173.md
        // BTC https://en.bitcoin.it/wiki/List_of_address_prefixes
        let ltc_network = BtcForkNetwork {
            desc: "LTC",
            hrp: "ltc",
            p2pkh_prefix: 0x30,
            p2sh_prefix: 0x32,
            xpub_prefix: [0x04, 0x88, 0xB2, 0x1E],
            xprv_prefix: [0x04, 0x88, 0xAD, 0xE4],
        };
        let pub_key_str = "0285e9737a74c30a873f74df05124f2aa6f53042c2fc0a130d6cbd7d16b944b004";
        let pub_key = PublicKey::from_str(pub_key_str).unwrap();
        let addr = BtcForkAddress::p2pkh(&pub_key, &ltc_network);
        assert_eq!("LTuEM81fckTP7hSdGEPAqQYKrCpu5Wp7F2", addr.to_string());

        let pri_key =
            SecretKey::from_str("4646464646464646464646464646464646464646464646464646464646464646")
                .unwrap();
        let pri_key = PrivateKey {
            compressed: true,
            network: Network::Bitcoin,
            key: pri_key,
        };
        let pub_key = pri_key.public_key(&Secp256k1::new());
        let addr = BtcForkAddress::p2wpkh(&pub_key, &ltc_network);
        assert_eq!(
            "ltc1qhkfq3zahaqkkzx5mjnamwjsfpq2jk7z0tamvsu",
            addr.to_string()
        );
    }
}
