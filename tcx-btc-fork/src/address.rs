use tcx_chain::curve::{PublicKey, Secp256k1PublicKey};

use crate::Error;
use crate::Result;
use bch_addr::Converter;
use bitcoin::network::constants::Network;
use bitcoin::util::address::Error as BtcAddressError;
use bitcoin::util::address::Payload;
use bitcoin::util::base58;
use bitcoin::{Address as BtcAddress, AddressType, Script};
use bitcoin_hashes::hash160;
use bitcoin_hashes::Hash;
use core::fmt::Debug;
use core::result;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use tcx_chain::keystore::Address;
use tcx_chain::{CoinInfo, DerivationInfo};

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
    pub fork_id: u8,
}

// LTC address prefix: https://bitcoin.stackexchange.com/questions/62781/litecoin-constants-and-prefixes
// hrp: https://github.com/satoshilabs/slips/blob/master/slip-0173.md
// BTC https://en.bitcoin.it/wiki/List_of_address_prefixes

pub fn network_from_coin(coin: &str) -> Option<BtcForkNetwork> {
    match coin.to_lowercase().as_str() {
        "ltc" => Some(BtcForkNetwork {
            coin: "LTC",
            hrp: "ltc",
            p2pkh_prefix: 0x30,
            p2sh_prefix: 0x32,
            xpub_prefix: [0x04, 0x88, 0xB2, 0x1E],
            xprv_prefix: [0x04, 0x88, 0xAD, 0xE4],
            fork_id: 0,
        }),
        "ltc-testnet" => Some(BtcForkNetwork {
            coin: "LTC-TESTNET",
            hrp: "ltc",
            p2pkh_prefix: 0x6f,
            p2sh_prefix: 0x3a,
            xpub_prefix: [0x04, 0x88, 0xB2, 0x1E],
            xprv_prefix: [0x04, 0x88, 0xAD, 0xE4],
            fork_id: 0,
        }),
        "btc" | "bc" => Some(BtcForkNetwork {
            coin: "BTC",
            hrp: "bc",
            p2pkh_prefix: 0x0,
            p2sh_prefix: 0x05,
            xpub_prefix: [0x04, 0x88, 0xB2, 0x1E],
            xprv_prefix: [0x04, 0x88, 0xAD, 0xE4],
            fork_id: 0,
        }),
        "btc-testnet" => Some(BtcForkNetwork {
            coin: "BTC-TESTNET",
            hrp: "bc",
            p2pkh_prefix: 0x6f,
            p2sh_prefix: 0xc4,
            xpub_prefix: [0x04, 0x88, 0xB2, 0x1E],
            xprv_prefix: [0x04, 0x88, 0xAD, 0xE4],
            fork_id: 0,
        }),
        "bitcoincash" | "bch" => Some(BtcForkNetwork {
            coin: "BCH",
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

impl Address for BtcForkAddress {
    fn is_valid(address: &str) -> bool {
        unimplemented!()
    }

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

impl BtcForkAddress {
    pub fn p2pkh(pub_key: &impl PublicKey, network: &BtcForkNetwork) -> Result<BtcForkAddress> {
        let pub_key = Secp256k1PublicKey::from_slice(&pub_key.to_bytes())?;
        let addr = BtcAddress::p2pkh(&pub_key, Network::Bitcoin);
        Ok(BtcForkAddress {
            payload: addr.payload,
            network: network.clone(),
        })
    }

    pub fn p2shwpkh(pub_key: &impl PublicKey, network: &BtcForkNetwork) -> Result<BtcForkAddress> {
        let pub_key = Secp256k1PublicKey::from_slice(&pub_key.to_bytes())?;
        let addr = BtcAddress::p2shwpkh(&pub_key, Network::Bitcoin);
        Ok(BtcForkAddress {
            payload: addr.payload,
            network: network.clone(),
        })
    }

    pub fn p2wpkh(pub_key: &impl PublicKey, network: &BtcForkNetwork) -> Result<BtcForkAddress> {
        let pub_key = Secp256k1PublicKey::from_slice(&pub_key.to_bytes())?;
        let addr = BtcAddress::p2wpkh(&pub_key, Network::Bitcoin);
        Ok(BtcForkAddress {
            payload: addr.payload,
            network: network.clone(),
        })
    }

    pub fn script_pubkey(&self) -> Script {
        self.payload.script_pubkey()
    }

    pub fn address_like(target_addr: &str, pub_key: &impl PublicKey) -> Result<BtcForkAddress> {
        let target = BtcForkAddress::from_str(target_addr)?;
        match target.payload {
            Payload::PubkeyHash(_) => BtcForkAddress::p2pkh(pub_key, &target.network),
            Payload::ScriptHash(_) => BtcForkAddress::p2shwpkh(pub_key, &target.network),
            Payload::WitnessProgram {
                version: ver,
                program: ref prog,
            } => BtcForkAddress::p2wpkh(pub_key, &target.network),
        }
    }

    pub fn extended_public_key(
        derivation_info: &DerivationInfo,
        coin_info: &CoinInfo,
    ) -> Result<String> {
        let network = network_from_coin(&coin_info.symbol);
        tcx_ensure!(network.is_some(), Error::UnsupportedChain);
        Ok(derivation_info.encode_with_network(network.expect("network").xpub_prefix))
    }

    pub fn extended_private_key(
        derivation_info: &DerivationInfo,
        coin_info: &CoinInfo,
    ) -> Result<String> {
        let network = network_from_coin(&coin_info.symbol);
        tcx_ensure!(network.is_some(), Error::UnsupportedChain);
        Ok(derivation_info.encode_with_network(network.expect("network").xprv_prefix))
    }

    pub fn is_main_net(addr: &str) -> bool {
        let convert = Converter::new();
        convert.is_mainnet_addr(addr)
    }

    pub fn convert_to_legacy_if_need(addr: &str) -> Result<String> {
        if None == addr.rfind("bitcoincash:") {
            return Ok(addr.to_string());
        }
        _bch_to_legacy(addr)
    }
}

fn _legacy_to_bch(addr: &str) -> Result<String> {
    let convert = Converter::new();
    if convert.is_legacy_addr(&addr) {
        convert
            .to_cash_addr(&addr)
            .map_err(|_| Error::ConvertToCashAddressFailed(addr.to_string()).into())
    } else {
        Ok(addr.to_string())
    }
}

fn _bch_to_legacy(addr: &str) -> Result<String> {
    let convert = Converter::new();
    if !convert.is_legacy_addr(&addr) {
        convert
            .to_legacy_addr(&addr)
            .map_err(|_| Error::ConvertToLegacyAddressFailed(addr.to_string()).into())
    } else {
        Ok(addr.to_string())
    }
}

/// Extract the bech32 prefix.
/// Returns the same slice when no prefix is found.
fn bech32_network(bech32: &str) -> Option<BtcForkNetwork> {
    let bech32_prefix = match bech32.rfind(":") {
        None => match bech32.rfind("1") {
            None => None,
            Some(sep) => Some(bech32.split_at(sep).0),
        },
        Some(sep) => Some(bech32.split_at(sep).0),
    };
    match bech32_prefix {
        Some(prefix) => network_from_coin(prefix),
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
            // try bch first
            if network.coin.starts_with("BCH") {
                let addr = _bch_to_legacy(s).expect("bch_to_legacy");
                let data = _decode_base58(&addr)?;
                return Ok(BtcForkAddress {
                    network,
                    payload: Payload::PubkeyHash(hash160::Hash::from_slice(&data[1..]).unwrap()),
                });
            } else {
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
        }

        let data = _decode_base58(s)?;
        let (network, payload) = match data[0] {
            0 => (
                bech32_network.unwrap_or(network_from_coin("btc").expect("btc")),
                Payload::PubkeyHash(hash160::Hash::from_slice(&data[1..]).unwrap()),
            ),
            5 => (
                bech32_network.unwrap_or(network_from_coin("btc").expect("btc")),
                Payload::ScriptHash(hash160::Hash::from_slice(&data[1..]).unwrap()),
            ),
            0x30 => (
                network_from_coin("ltc").expect("ltc-L"),
                Payload::PubkeyHash(hash160::Hash::from_slice(&data[1..]).unwrap()),
            ),
            0x32 => (
                network_from_coin("ltc").expect("ltc"),
                Payload::ScriptHash(hash160::Hash::from_slice(&data[1..]).unwrap()),
            ),
            0x3a => (
                network_from_coin("ltc-testnet").expect("ltc-testnet"),
                Payload::ScriptHash(hash160::Hash::from_slice(&data[1..]).unwrap()),
            ),
            111 => (
                bech32_network.unwrap_or(network_from_coin("btc-testnet").expect("btc-testnet")),
                Payload::PubkeyHash(hash160::Hash::from_slice(&data[1..]).unwrap()),
            ),
            196 => (
                bech32_network.unwrap_or(network_from_coin("btc-testnet").expect("btc-testnet")),
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
                if self.network.coin.to_lowercase() == "bch" {
                    let legacy = base58::check_encode_slice(&prefixed[..]);
                    let baddr = _legacy_to_bch(&legacy).expect("legacy_to_bch");
                    std::fmt::Display::fmt(&baddr, fmt)
                //                    baddr.fmt(fmt)
                } else {
                    base58::check_encode_slice_to_fmt(fmt, &prefixed[..])
                }
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
    use crate::address::{network_from_coin, BtcForkAddress};
    use bitcoin::consensus::encode::Error::Secp256k1;
    use bitcoin::util::misc::hex_bytes;
    use bitcoin_hashes::hex::ToHex;
    use std::str::FromStr;
    use tcx_chain::keystore::Address;
    use tcx_chain::Secp256k1PublicKey;
    use tcx_chain::{PublicKey, Secp256k1PrivateKey};

    #[test]
    pub fn test_convert() {
        assert_eq!(
            BtcForkAddress::convert_to_legacy_if_need("2N54wJxopnWTvBfqgAPVWqXVEdaqoH7Suvf")
                .unwrap(),
            "2N54wJxopnWTvBfqgAPVWqXVEdaqoH7Suvf"
        );
        assert_eq!(
            BtcForkAddress::convert_to_legacy_if_need(
                "bitcoincash:qqyta3mqzeaxe8hqcdsgpy4srwd4f0fc0gj0njf885"
            )
            .unwrap(),
            "1oEx5Ztg2DUDYJDxb1AeaiG5TYesikMVU"
        );

        assert_eq!(
            format!(
                "{}",
                BtcForkAddress::convert_to_legacy_if_need("bitcoincash:")
                    .err()
                    .unwrap()
            ),
            "bch_convert_to_legacy_address_failed# address: bitcoincash:"
        );
    }

    #[test]
    pub fn test_from_pub_key() {
        let pub_key =
            hex_bytes("026b5b6a9d041bc5187e0b34f9e496436c7bff261c6c1b5f3c06b433c61394b868")
                .unwrap();
        let addr = BtcForkAddress::from_public_key(
            &Secp256k1PublicKey::from_slice(&pub_key).unwrap(),
            Some("bch"),
        )
        .unwrap();
        assert_eq!(
            addr,
            "bitcoincash:qq2ug6v04ht22n0daxxzl0rzlvsmzwcdwuymj77ymy"
        );
    }

    #[test]
    pub fn test_btc_fork_address() {
        let pub_key = Secp256k1PublicKey::from_str(
            "02506bc1dc099358e5137292f4efdd57e400f29ba5132aa5d12b18dac1c1f6aaba",
        )
        .unwrap();

        let network = network_from_coin("ltc").unwrap();
        let addr = BtcForkAddress::p2shwpkh(&pub_key, &network)
            .unwrap()
            .to_string();
        assert_eq!(addr, "MR5Hu9zXPX3o9QuYNJGft1VMpRP418QDfW");
        let addr = BtcForkAddress::p2wpkh(&pub_key, &network)
            .unwrap()
            .to_string();
        assert_eq!(addr, "ltc1qum864wd9nwsc0u9ytkctz6wzrw6g7zdn08yddf");

        let network = network_from_coin("bch").unwrap();
        let addr = BtcForkAddress::p2pkh(&pub_key, &network)
            .unwrap()
            .to_string();
        assert_eq!(
            addr,
            "bitcoincash:qrnvl24e5kd6rpls53wmpvtfcgdmfrcfkv8fhnq9kr"
        );

        let network = network_from_coin("btc").unwrap();
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
        assert_eq!(addr.network.coin, "LTC");
        let addr = BtcForkAddress::from_str("ltc1qum864wd9nwsc0u9ytkctz6wzrw6g7zdn08yddf").unwrap();
        assert_eq!(addr.network.coin, "LTC");

        let addr =
            BtcForkAddress::from_str("bitcoincash:qrnvl24e5kd6rpls53wmpvtfcgdmfrcfkv8fhnq9kr")
                .unwrap();
        assert_eq!(addr.network.coin, "BCH");

        let addr = BtcForkAddress::from_str("3Js9bGaZSQCNLudeGRHL4NExVinc25RbuG").unwrap();
        assert_eq!(addr.network.coin, "BTC");
        let addr = BtcForkAddress::from_str("bc1qum864wd9nwsc0u9ytkctz6wzrw6g7zdntm7f4e").unwrap();
        assert_eq!(addr.network.coin, "BTC");
    }

}
