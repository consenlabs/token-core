//use tcx_chain::curve::{PublicKey, Secp256k1PublicKey};

use crate::transaction::ScriptPubKeyComponent;
use crate::Error;
use crate::Result;
use bitcoin::network::constants::Network;
use bitcoin::util::address::Error as BtcAddressError;
use bitcoin::util::address::Payload;
use bitcoin::util::base58;
use bitcoin::{Address as BtcAddress, Script};
use bitcoin_hashes::hash160;
use bitcoin_hashes::Hash;
use core::result;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use tcx_chain::keystore::Address;
use tcx_chain::CoinInfo;
use tcx_primitive::{
    ArbitraryNetworkExtendedPrivKey, ArbitraryNetworkExtendedPubKey, Public, Secp256k1PublicKey,
};

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BtcForkAddress {
    pub network: BtcForkNetwork,
    pub payload: Payload,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BtcForkNetwork {
    pub coin: &'static str,
    pub hrp: &'static str,
    pub p2pkh_prefix: u8,
    pub p2sh_prefix: u8,
    pub xpub_prefix: [u8; 4],
    pub xprv_prefix: [u8; 4],
}

// LTC address prefix: https://bitcoin.stackexchange.com/questions/62781/litecoin-constants-and-prefixes
// hrp: https://github.com/satoshilabs/slips/blob/master/slip-0173.md
// BTC https://en.bitcoin.it/wiki/List_of_address_prefixes

pub fn network_from_coin(coin: &str) -> Option<BtcForkNetwork> {
    match coin.to_uppercase().as_str() {
        "LITECOIN" => Some(BtcForkNetwork {
            coin: "LITECOIN",
            hrp: "ltc",
            p2pkh_prefix: 0x30,
            p2sh_prefix: 0x32,
            xpub_prefix: [0x04, 0x88, 0xB2, 0x1E],
            xprv_prefix: [0x04, 0x88, 0xAD, 0xE4],
        }),
        "LITECOIN-P2WPKH" => Some(BtcForkNetwork {
            coin: "LITECOIN-P2WPKH",
            hrp: "ltc",
            p2pkh_prefix: 0x30,
            p2sh_prefix: 0x32,
            xpub_prefix: [0x04, 0x88, 0xB2, 0x1E],
            xprv_prefix: [0x04, 0x88, 0xAD, 0xE4],
        }),
        "LITECOIN-SEGWIT" => Some(BtcForkNetwork {
            coin: "LITECOIN-SEGWIT",
            hrp: "ltc",
            p2pkh_prefix: 0x30,
            p2sh_prefix: 0x32,
            xpub_prefix: [0x04, 0x88, 0xB2, 0x1E],
            xprv_prefix: [0x04, 0x88, 0xAD, 0xE4],
        }),
        "LITECOIN-TESTNET" => Some(BtcForkNetwork {
            coin: "LITECOIN-TESTNET",
            hrp: "ltc",
            p2pkh_prefix: 0x6f,
            p2sh_prefix: 0x3a,
            //            043587CF
            //            04358394
            xpub_prefix: [0x04, 0x35, 0x87, 0xCF],
            xprv_prefix: [0x04, 0x35, 0x83, 0x94],
        }),
        "LITECOIN-TESTNET-P2WPKH" => Some(BtcForkNetwork {
            coin: "LITECOIN-TESTNET-P2WPKH",
            hrp: "ltc",
            p2pkh_prefix: 0x6f,
            p2sh_prefix: 0x3a,
            xpub_prefix: [0x04, 0x35, 0x87, 0xCF],
            xprv_prefix: [0x04, 0x35, 0x83, 0x94],
        }),
        "BITCOIN" => Some(BtcForkNetwork {
            coin: "BITCOIN",
            hrp: "bc",
            p2pkh_prefix: 0x0,
            p2sh_prefix: 0x05,
            xpub_prefix: [0x04, 0x88, 0xB2, 0x1E],
            xprv_prefix: [0x04, 0x88, 0xAD, 0xE4],
        }),
        "BITCOIN-P2WPKH" => Some(BtcForkNetwork {
            coin: "BITCOIN-P2WPKH",
            hrp: "bc",
            p2pkh_prefix: 0x0,
            p2sh_prefix: 0x05,
            xpub_prefix: [0x04, 0x88, 0xB2, 0x1E],
            xprv_prefix: [0x04, 0x88, 0xAD, 0xE4],
        }),
        "BITCOIN-SEGWIT" => Some(BtcForkNetwork {
            coin: "BITCOIN-SEGWIT",
            hrp: "bc",
            p2pkh_prefix: 0x0,
            p2sh_prefix: 0x05,
            xpub_prefix: [0x04, 0x88, 0xB2, 0x1E],
            xprv_prefix: [0x04, 0x88, 0xAD, 0xE4],
        }),
        "BITCOIN-TESTNET" => Some(BtcForkNetwork {
            coin: "BITCOIN-TESTNET",
            hrp: "bc",
            p2pkh_prefix: 0x6f,
            p2sh_prefix: 0xc4,
            xpub_prefix: [0x04, 0x35, 0x87, 0xCF],
            xprv_prefix: [0x04, 0x35, 0x83, 0x94],
        }),
        "BITCOINCASH" => Some(BtcForkNetwork {
            coin: "BITCOINCASH",
            hrp: "bitcoincash",
            p2pkh_prefix: 0x0,
            p2sh_prefix: 0x05,
            xpub_prefix: [0x04, 0x88, 0xB2, 0x1E],
            xprv_prefix: [0x04, 0x88, 0xAD, 0xE4],
        }),
        _ => None,
    }
}

pub fn network_form_hrp(hrp: &str) -> Option<BtcForkNetwork> {
    match hrp {
        "bitcoincash" => network_from_coin("BITCOINCASH"),
        "ltc" => network_from_coin("LITECOIN-SEGWIT"),
        "bc" => network_from_coin("BITCOIN-SEGWIT"),
        _ => None,
    }
}

impl Address for BtcForkAddress {
    fn is_valid(_address: &str) -> bool {
        unimplemented!()
    }

    fn from_public_key(public_key: &[u8], coin: Option<&str>) -> Result<String> {
        //        let pub_key = Secp256k1PublicKey::from_slice(&public_key)?.public_key();
        let coin = coin.expect("coin from address_pub_key");
        let network = network_from_coin(&coin);
        tcx_ensure!(network.is_some(), Error::UnsupportedChain);
        let network = network.expect("network");
        let addr: String;
        if coin.to_uppercase().contains("P2WPKH") {
            addr = BtcForkAddress::p2shwpkh(public_key, &network)?.to_string();
        } else {
            addr = BtcForkAddress::p2pkh(public_key, &network)?.to_string();
        }
        //        let addr = BtcForkAddress::p2shwpkh(&pub_key, &network)?.to_string();
        Ok(addr.to_string())
    }
}

impl BtcForkAddress {
    pub fn p2pkh(pub_key: &[u8], network: &BtcForkNetwork) -> Result<BtcForkAddress> {
        let pub_key = Secp256k1PublicKey::from_slice(&pub_key)?;
        let addr = BtcAddress::p2pkh(&pub_key.public_key(), Network::Bitcoin);
        Ok(BtcForkAddress {
            payload: addr.payload,
            network: network.clone(),
        })
    }

    pub fn p2shwpkh(pub_key: &[u8], network: &BtcForkNetwork) -> Result<BtcForkAddress> {
        let pub_key = Secp256k1PublicKey::from_slice(&pub_key)?;
        let addr = BtcAddress::p2shwpkh(&pub_key.public_key(), Network::Bitcoin);
        Ok(BtcForkAddress {
            payload: addr.payload,
            network: network.clone(),
        })
    }

    pub fn p2wpkh(pub_key: &[u8], network: &BtcForkNetwork) -> Result<BtcForkAddress> {
        let pub_key = Secp256k1PublicKey::from_slice(&pub_key)?;
        let addr = BtcAddress::p2wpkh(&pub_key.public_key(), Network::Bitcoin);
        Ok(BtcForkAddress {
            payload: addr.payload,
            network: network.clone(),
        })
    }

    pub fn script_pubkey(&self) -> Script {
        self.payload.script_pubkey()
    }

    pub fn address_like(target_addr: &str, pub_key: &[u8]) -> Result<BtcForkAddress> {
        let target = BtcForkAddress::from_str(target_addr)?;
        match target.payload {
            Payload::PubkeyHash(_) => BtcForkAddress::p2pkh(pub_key, &target.network),
            Payload::ScriptHash(_) => BtcForkAddress::p2shwpkh(pub_key, &target.network),
            Payload::WitnessProgram {
                version: _ver,
                program: ref _prog,
            } => BtcForkAddress::p2wpkh(pub_key, &target.network),
        }
    }

    pub fn extended_public_key(
        derivation_info: &ArbitraryNetworkExtendedPubKey,
        coin_info: &CoinInfo,
    ) -> Result<String> {
        let network = network_from_coin(&coin_info.symbol);
        tcx_ensure!(network.is_some(), Error::UnsupportedChain);
        let anepk = ArbitraryNetworkExtendedPubKey {
            network: network.unwrap().xpub_prefix,
            extended_pub_key: derivation_info.extended_pub_key,
        };
        Ok(anepk.to_string())
    }

    pub fn extended_private_key(
        extended_priv_key: &ArbitraryNetworkExtendedPrivKey,
        coin_info: &CoinInfo,
    ) -> Result<String> {
        let network = network_from_coin(&coin_info.symbol);
        tcx_ensure!(network.is_some(), Error::UnsupportedChain);
        let anepk = ArbitraryNetworkExtendedPrivKey {
            network: network.unwrap().xpub_prefix,
            extended_priv_key: extended_priv_key.extended_priv_key,
        };
        Ok(anepk.to_string())
    }
}

/// Extract the bech32 prefix.
/// Returns the same slice when no prefix is found.
fn bech32_network(bech32: &str) -> Option<BtcForkNetwork> {
    let bech32_prefix = match bech32.rfind("1") {
        None => None,
        Some(sep) => Some(bech32.split_at(sep).0),
    };
    match bech32_prefix {
        Some(prefix) => network_form_hrp(prefix),
        None => None,
    }
}

fn _decode_base58(addr: &str) -> result::Result<Vec<u8>, BtcAddressError> {
    // Base58
    if addr.len() > 50 {
        return Err(BtcAddressError::Base58(base58::Error::InvalidLength(
            addr.len() * 11 / 15,
        )));
    }
    let data = base58::from_check(&addr)?;
    if data.len() != 21 {
        return Err(BtcAddressError::Base58(base58::Error::InvalidLength(
            data.len(),
        )));
    } else {
        return Ok(data);
    }
}

impl FromStr for BtcForkAddress {
    type Err = BtcAddressError;

    fn from_str(s: &str) -> result::Result<BtcForkAddress, BtcAddressError> {
        // try bech32
        let bech32_network = bech32_network(s);
        if let Some(network) = bech32_network {
            // decode as bech32
            let (_, payload) = bech32::decode(s)?;
            if payload.len() == 0 {
                return Err(BtcAddressError::EmptyBech32Payload);
            }

            // Get the script version and program (converted from 5-bit to 8-bit)
            let (version, program): (bech32::u5, Vec<u8>) = {
                let (v, p5) = payload.split_at(1);
                (v[0], bech32::FromBase32::from_base32(p5)?)
            };

            // Generic segwit checks.
            if version.to_u8() > 16 {
                return Err(BtcAddressError::InvalidWitnessVersion(version.to_u8()));
            }
            if program.len() < 2 || program.len() > 40 {
                return Err(BtcAddressError::InvalidWitnessProgramLength(program.len()));
            }

            // Specific segwit v0 check.
            if version.to_u8() == 0 && (program.len() != 20 && program.len() != 32) {
                return Err(BtcAddressError::InvalidSegwitV0ProgramLength(program.len()));
            }

            return Ok(BtcForkAddress {
                payload: Payload::WitnessProgram {
                    version: version,
                    program: program,
                },
                network: network,
            });
        }

        let data = _decode_base58(s)?;
        let (network, payload) = match data[0] {
            0 => (
                network_from_coin("BITCOIN").expect("btc"),
                Payload::PubkeyHash(hash160::Hash::from_slice(&data[1..]).unwrap()),
            ),
            5 => (
                network_from_coin("BITCOIN-P2WPKH").expect("btc"),
                Payload::ScriptHash(hash160::Hash::from_slice(&data[1..]).unwrap()),
            ),
            0x30 => (
                network_from_coin("LITECOIN").expect("ltc-L"),
                Payload::PubkeyHash(hash160::Hash::from_slice(&data[1..]).unwrap()),
            ),
            0x32 => (
                network_from_coin("LITECOIN-P2WPKH").expect("ltc"),
                Payload::ScriptHash(hash160::Hash::from_slice(&data[1..]).unwrap()),
            ),
            0x3a => (
                network_from_coin("LITECOIN-TESTNET-P2WPKH").expect("ltc-testnet"),
                Payload::ScriptHash(hash160::Hash::from_slice(&data[1..]).unwrap()),
            ),
            111 => (
                network_from_coin("BITCOIN-TESTNET").expect("btc-testnet"),
                Payload::PubkeyHash(hash160::Hash::from_slice(&data[1..]).unwrap()),
            ),
            196 => (
                network_from_coin("BITCOIN-TESTNET-P2WPKH").expect("btc-testnet"),
                Payload::ScriptHash(hash160::Hash::from_slice(&data[1..]).unwrap()),
            ),
            x => {
                return Err(BtcAddressError::Base58(base58::Error::InvalidVersion(
                    vec![x],
                )));
            }
        };

        Ok(BtcForkAddress { network, payload })
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

pub trait PubKeyScript: Sized {
    fn script_pub_key(&self) -> Script;
}

impl PubKeyScript for BtcForkAddress {
    fn script_pub_key(&self) -> Script {
        self.script_pubkey()
    }
}

impl ScriptPubKeyComponent for BtcForkAddress {
    fn address_like(target_addr: &str, pub_key: &bitcoin::PublicKey) -> Result<Script> {
        let addr = BtcForkAddress::address_like(target_addr, &pub_key.to_bytes())?;
        Ok(addr.script_pubkey())
    }

    fn address_script_pub_key(target_addr: &str) -> Result<Script> {
        let addr = BtcForkAddress::from_str(target_addr)?;
        Ok(addr.script_pubkey())
    }
}

#[cfg(test)]
mod tests {
    use crate::address::{network_from_coin, BtcForkAddress};

    use std::str::FromStr;

    #[test]
    pub fn test_btc_fork_address() {
        let pub_key_str = "02506bc1dc099358e5137292f4efdd57e400f29ba5132aa5d12b18dac1c1f6aaba";
        let pub_key = hex::decode(pub_key_str).unwrap();
        let network = network_from_coin("LITECOIN").unwrap();
        let addr = BtcForkAddress::p2shwpkh(&pub_key, &network)
            .unwrap()
            .to_string();
        assert_eq!(addr, "MR5Hu9zXPX3o9QuYNJGft1VMpRP418QDfW");
        let addr = BtcForkAddress::p2wpkh(&pub_key, &network)
            .unwrap()
            .to_string();
        assert_eq!(addr, "ltc1qum864wd9nwsc0u9ytkctz6wzrw6g7zdn08yddf");

        let network = network_from_coin("BITCOIN").unwrap();
        let addr = BtcForkAddress::p2shwpkh(&pub_key, &network)
            .unwrap()
            .to_string();
        assert_eq!(addr, "3Js9bGaZSQCNLudeGRHL4NExVinc25RbuG");

        let addr = BtcForkAddress::p2wpkh(&pub_key, &network)
            .unwrap()
            .to_string();
        assert_eq!(addr, "bc1qum864wd9nwsc0u9ytkctz6wzrw6g7zdntm7f4e");
    }

    #[test]
    pub fn test_btc_fork_address_from_str() {
        let addr = BtcForkAddress::from_str("MR5Hu9zXPX3o9QuYNJGft1VMpRP418QDfW").unwrap();
        assert_eq!(addr.network.coin, "LITECOIN-P2WPKH");
        let addr = BtcForkAddress::from_str("ltc1qum864wd9nwsc0u9ytkctz6wzrw6g7zdn08yddf").unwrap();
        assert_eq!(addr.network.coin, "LITECOIN-SEGWIT");

        let addr = BtcForkAddress::from_str("3Js9bGaZSQCNLudeGRHL4NExVinc25RbuG").unwrap();
        assert_eq!(addr.network.coin, "BITCOIN-P2WPKH");
        let addr = BtcForkAddress::from_str("bc1qum864wd9nwsc0u9ytkctz6wzrw6g7zdntm7f4e").unwrap();
        assert_eq!(addr.network.coin, "BITCOIN-SEGWIT");
    }
}
