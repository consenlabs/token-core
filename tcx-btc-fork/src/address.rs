use crate::signer::ScriptPubKeyComponent;
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
use tcx_chain::Address;
use tcx_constants::btc_fork_network::{network_form_hrp, network_from_coin, BtcForkNetwork};
use tcx_constants::coin_info::coin_info_from_param;
use tcx_constants::CoinInfo;
use tcx_primitive::{Ss58Codec, TypedPrivateKey, TypedPublicKey};

pub trait WifDisplay {
    fn fmt(&self, coin_info: &CoinInfo) -> Result<String>;
}

impl WifDisplay for TypedPrivateKey {
    fn fmt(&self, coin_info: &CoinInfo) -> Result<String> {
        let network = network_from_coin(coin_info);
        tcx_ensure!(network.is_some(), Error::UnsupportedChain);
        // let typed_pk = TypedPrivateKey::from_slice(CurveType::SECP256k1, &data)?;
        let key = self.as_secp256k1()?;
        let version = vec![network.unwrap().private_prefix];
        Ok(key.to_ss58check_with_version(&version))
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BtcForkAddress {
    pub network: BtcForkNetwork,
    pub payload: Payload,
}

impl Address for BtcForkAddress {
    fn from_public_key(public_key: &TypedPublicKey, coin: &CoinInfo) -> Result<String> {
        let network = network_from_coin(&coin);
        tcx_ensure!(network.is_some(), Error::MissingNetwork);
        let network = network.expect("network");

        let addr = if coin.seg_wit.as_str() == "P2WPKH" {
            BtcForkAddress::p2shwpkh(&public_key.to_bytes(), &network)?.to_string()
        } else {
            BtcForkAddress::p2pkh(&public_key.to_bytes(), &network)?.to_string()
        };
        Ok(addr.to_string())
    }

    fn is_valid(address: &str, coin: &CoinInfo) -> bool {
        let ret = BtcForkAddress::from_str(address);
        if ret.is_err() {
            false
        } else {
            let addr: BtcForkAddress = ret.unwrap();
            addr.network.network == coin.network
        }
    }
}

impl BtcForkAddress {
    pub fn p2pkh(pub_key: &[u8], network: &BtcForkNetwork) -> Result<BtcForkAddress> {
        let pub_key = bitcoin::PublicKey::from_slice(&pub_key)?;
        let addr = BtcAddress::p2pkh(&pub_key, Network::Bitcoin);
        Ok(BtcForkAddress {
            payload: addr.payload,
            network: network.clone(),
        })
    }

    pub fn p2shwpkh(pub_key: &[u8], network: &BtcForkNetwork) -> Result<BtcForkAddress> {
        let pub_key = bitcoin::PublicKey::from_slice(&pub_key)?;
        let addr = BtcAddress::p2shwpkh(&pub_key, Network::Bitcoin);
        Ok(BtcForkAddress {
            payload: addr.payload,
            network: network.clone(),
        })
    }

    pub fn p2wpkh(pub_key: &[u8], network: &BtcForkNetwork) -> Result<BtcForkAddress> {
        let pub_key = bitcoin::PublicKey::from_slice(&pub_key)?;
        let addr = BtcAddress::p2wpkh(&pub_key, Network::Bitcoin);
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
        derivation_info: &impl Ss58Codec,
        coin_info: &CoinInfo,
    ) -> Result<String> {
        let network = network_from_coin(&coin_info);
        tcx_ensure!(network.is_some(), Error::UnsupportedChain);
        Ok(derivation_info.to_ss58check_with_version(&network.unwrap().xpub_prefix))
    }

    pub fn extended_private_key(
        extended_priv_key: &impl Ss58Codec,
        coin_info: &CoinInfo,
    ) -> Result<String> {
        let network = network_from_coin(&coin_info);
        tcx_ensure!(network.is_some(), Error::UnsupportedChain);
        Ok(extended_priv_key.to_ss58check_with_version(&network.unwrap().xprv_prefix))
    }
}

/// Extract the bech32 prefix.
/// Returns the same slice when no prefix is found.
fn bech32_network(bech32: &str) -> Option<BtcForkNetwork> {
    let bech32_prefix = match bech32.rfind('1') {
        None => None,
        Some(sep) => Some(bech32.split_at(sep).0),
    };
    match bech32_prefix {
        Some(prefix) => network_form_hrp(prefix),
        None => None,
    }
}

fn decode_base58(addr: &str) -> result::Result<Vec<u8>, BtcAddressError> {
    // Base58
    if addr.len() > 50 {
        return Err(BtcAddressError::Base58(base58::Error::InvalidLength(
            addr.len() * 11 / 15,
        )));
    }
    let data = base58::from_check(&addr)?;
    if data.len() != 21 {
        Err(BtcAddressError::Base58(base58::Error::InvalidLength(
            data.len(),
        )))
    } else {
        Ok(data)
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
            if payload.is_empty() {
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
                payload: Payload::WitnessProgram { version, program },
                network,
            });
        }

        let data = decode_base58(s)?;
        let (network, payload) = match data[0] {
            0 => {
                let coin_info = coin_info_from_param("BITCOIN", "MAINNET", "NONE")
                    .expect("BtcForkNetwork coin_info");
                (
                    network_from_coin(&coin_info).expect("btc"),
                    Payload::PubkeyHash(hash160::Hash::from_slice(&data[1..]).unwrap()),
                )
            }
            5 => {
                let coin_info = coin_info_from_param("BITCOIN", "MAINNET", "P2WPKH")
                    .expect("BITCOIN-P2WPKH coin_info");
                (
                    network_from_coin(&coin_info).expect("btc"),
                    Payload::ScriptHash(hash160::Hash::from_slice(&data[1..]).unwrap()),
                )
            }
            0x30 => {
                let coin_info = coin_info_from_param("LITECOIN", "MAINNET", "NONE")
                    .expect("LITECOIN coin_info");
                (
                    network_from_coin(&coin_info).expect("ltc-L"),
                    Payload::PubkeyHash(hash160::Hash::from_slice(&data[1..]).unwrap()),
                )
            }
            0x32 => {
                let coin_info = coin_info_from_param("LITECOIN", "MAINNET", "P2WPKH")
                    .expect("LITECOIN-P2WPKH coin_info");
                (
                    network_from_coin(&coin_info).expect("ltc"),
                    Payload::ScriptHash(hash160::Hash::from_slice(&data[1..]).unwrap()),
                )
            }
            0x3a => {
                let coin_info = coin_info_from_param("LITECOIN", "TESTNET", "P2WPKH")
                    .expect("LITECOIN TESTNET P2WPKH coin_info");
                (
                    network_from_coin(&coin_info).expect("ltc-testnet"),
                    Payload::ScriptHash(hash160::Hash::from_slice(&data[1..]).unwrap()),
                )
            }
            111 => {
                let coin_info = coin_info_from_param("BITCOIN", "TESTNET", "NONE")
                    .expect("BITCOIN-TESTNET coin_info");
                (
                    network_from_coin(&coin_info).expect("btc-testnet"),
                    Payload::PubkeyHash(hash160::Hash::from_slice(&data[1..]).unwrap()),
                )
            }
            196 => {
                let coin_info = coin_info_from_param("BITCOIN", "TESTNET", "P2WPKH")
                    .expect("BITCOIN-TESTNET-P2WPKH coin_info");
                (
                    network_from_coin(&coin_info).expect("btc-testnet"),
                    Payload::ScriptHash(hash160::Hash::from_slice(&data[1..]).unwrap()),
                )
            }
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
    fn address_script_like(target_addr: &str, pub_key: &bitcoin::PublicKey) -> Result<Script> {
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
    use crate::address::BtcForkAddress;
    use crate::signer::ScriptPubKeyComponent;
    use crate::tcx_chain::Address;
    use tcx_constants::coin_info::coin_info_from_param;

    use std::str::FromStr;
    use tcx_constants::btc_fork_network::network_from_param;

    use tcx_constants::{CoinInfo, CurveType};
    use tcx_primitive::{
        Bip32DeterministicPrivateKey, Derive, DerivePath, DeterministicPrivateKey, Ss58Codec,
    };

    #[test]
    pub fn test_btc_fork_address() {
        let pub_key_str = "02506bc1dc099358e5137292f4efdd57e400f29ba5132aa5d12b18dac1c1f6aaba";
        let pub_key = hex::decode(pub_key_str).unwrap();
        let network = network_from_param("LITECOIN", "MAINNET", "NONE").unwrap();
        let addr = BtcForkAddress::p2shwpkh(&pub_key, &network)
            .unwrap()
            .to_string();
        assert_eq!(addr, "MR5Hu9zXPX3o9QuYNJGft1VMpRP418QDfW");

        let network = network_from_param("LITECOIN", "MAINNET", "SEGWIT").unwrap();
        let addr = BtcForkAddress::p2wpkh(&pub_key, &network)
            .unwrap()
            .to_string();
        assert_eq!(addr, "ltc1qum864wd9nwsc0u9ytkctz6wzrw6g7zdn08yddf");

        let network = network_from_param("BITCOIN", "MAINNET", "NONE").unwrap();
        let addr = BtcForkAddress::p2shwpkh(&pub_key, &network)
            .unwrap()
            .to_string();
        assert_eq!(addr, "3Js9bGaZSQCNLudeGRHL4NExVinc25RbuG");

        let network = network_from_param("BITCOIN", "MAINNET", "SEGWIT").unwrap();
        let addr = BtcForkAddress::p2wpkh(&pub_key, &network)
            .unwrap()
            .to_string();
        assert_eq!(addr, "bc1qum864wd9nwsc0u9ytkctz6wzrw6g7zdntm7f4e");
    }

    #[test]
    pub fn test_btc_fork_address_from_str() {
        let addr = BtcForkAddress::from_str("MR5Hu9zXPX3o9QuYNJGft1VMpRP418QDfW").unwrap();
        assert_eq!(addr.network.coin, "LITECOIN");
        assert_eq!(addr.network.seg_wit, "P2WPKH");
        assert_eq!(addr.network.network, "MAINNET");
        let addr = BtcForkAddress::from_str("ltc1qum864wd9nwsc0u9ytkctz6wzrw6g7zdn08yddf").unwrap();
        assert_eq!(addr.network.coin, "LITECOIN");
        assert_eq!(addr.network.seg_wit, "SEGWIT");
        assert_eq!(addr.network.network, "MAINNET");

        let addr = BtcForkAddress::from_str("3Js9bGaZSQCNLudeGRHL4NExVinc25RbuG").unwrap();
        assert_eq!(addr.network.coin, "BITCOIN");
        assert_eq!(addr.network.seg_wit, "P2WPKH");
        assert_eq!(addr.network.network, "MAINNET");
        let addr = BtcForkAddress::from_str("bc1qum864wd9nwsc0u9ytkctz6wzrw6g7zdntm7f4e").unwrap();
        assert_eq!(addr.network.coin, "BITCOIN");
        assert_eq!(addr.network.seg_wit, "SEGWIT");
        assert_eq!(addr.network.network, "MAINNET");
        let addr = BtcForkAddress::from_str("12z6UzsA3tjpaeuvA2Zr9jwx19Azz74D6g").unwrap();
        assert_eq!(addr.network.coin, "BITCOIN");
        assert_eq!(addr.network.seg_wit, "NONE");
        assert_eq!(addr.network.network, "MAINNET");

        let addr = BtcForkAddress::from_str("2MwN441dq8qudMvtM5eLVwC3u4zfKuGSQAB").unwrap();
        assert_eq!(addr.network.coin, "BITCOIN");
        assert_eq!(addr.network.seg_wit, "P2WPKH");
        assert_eq!(addr.network.network, "TESTNET");
    }

    #[test]
    pub fn test_address_like() {
        let addr = BtcForkAddress::from_str("MR5Hu9zXPX3o9QuYNJGft1VMpRP418QDfW").unwrap();
        let pub_key =
            hex::decode("02506bc1dc099358e5137292f4efdd57e400f29ba5132aa5d12b18dac1c1f6aaba")
                .unwrap();
        let liked_address = BtcForkAddress::address_like(&addr.to_string(), &pub_key)
            .ok()
            .unwrap();
        assert_eq!(
            "MR5Hu9zXPX3o9QuYNJGft1VMpRP418QDfW",
            liked_address.to_string()
        );

        let addr = BtcForkAddress::from_str("ltc1qum864wd9nwsc0u9ytkctz6wzrw6g7zdn08yddf").unwrap();
        let pub_key =
            hex::decode("02506bc1dc099358e5137292f4efdd57e400f29ba5132aa5d12b18dac1c1f6aaba")
                .unwrap();
        let liked_address = BtcForkAddress::address_like(&addr.to_string(), &pub_key)
            .ok()
            .unwrap();
        assert_eq!(
            "ltc1qum864wd9nwsc0u9ytkctz6wzrw6g7zdn08yddf",
            liked_address.to_string()
        );

        let addr = BtcForkAddress::from_str("3Js9bGaZSQCNLudeGRHL4NExVinc25RbuG").unwrap();
        let pub_key =
            hex::decode("02506bc1dc099358e5137292f4efdd57e400f29ba5132aa5d12b18dac1c1f6aaba")
                .unwrap();
        let liked_address = BtcForkAddress::address_like(&addr.to_string(), &pub_key)
            .ok()
            .unwrap();
        assert_eq!(
            "3Js9bGaZSQCNLudeGRHL4NExVinc25RbuG",
            liked_address.to_string()
        );

        let addr = BtcForkAddress::from_str("bc1qum864wd9nwsc0u9ytkctz6wzrw6g7zdntm7f4e").unwrap();
        let pub_key =
            hex::decode("02506bc1dc099358e5137292f4efdd57e400f29ba5132aa5d12b18dac1c1f6aaba")
                .unwrap();
        let liked_address = BtcForkAddress::address_like(&addr.to_string(), &pub_key)
            .ok()
            .unwrap();
        assert_eq!(
            "bc1qum864wd9nwsc0u9ytkctz6wzrw6g7zdntm7f4e",
            liked_address.to_string()
        );
    }

    #[test]
    pub fn extended_private_key_test() {
        let bitcoin_xprv_str = "xprv9yrdwPSRnvomqFK4u1y5uW2SaXS2Vnr3pAYTjJjbyRZR8p9BwoadRsCxtgUFdAKeRPbwvGRcCSYMV69nNK4N2kadevJ6L5iQVy1SwGKDTHQ";
        let anprv = Bip32DeterministicPrivateKey::from_ss58check(bitcoin_xprv_str).unwrap();
        let coin_info = CoinInfo {
            coin: "LITECOIN".to_string(),
            derivation_path: "m/44'/2'/0'/0/0".to_string(),
            curve: CurveType::SECP256k1,
            network: "MAINNET".to_string(),
            seg_wit: "NONE".to_string(),
        };
        let ltc_xprv_str = BtcForkAddress::extended_private_key(&anprv, &coin_info).unwrap();
        assert_eq!("xprv9yrdwPSRnvomqFK4u1y5uW2SaXS2Vnr3pAYTjJjbyRZR8p9BwoadRsCxtgUFdAKeRPbwvGRcCSYMV69nNK4N2kadevJ6L5iQVy1SwGKDTHQ", ltc_xprv_str);
    }

    #[test]
    pub fn extended_public_key_test() {
        let bitcoin_xprv_str = "xprv9yrdwPSRnvomqFK4u1y5uW2SaXS2Vnr3pAYTjJjbyRZR8p9BwoadRsCxtgUFdAKeRPbwvGRcCSYMV69nNK4N2kadevJ6L5iQVy1SwGKDTHQ";
        let anpub = Bip32DeterministicPrivateKey::from_ss58check(bitcoin_xprv_str)
            .unwrap()
            .derive(DerivePath::from_str("m/44'/2'/0'").unwrap().into_iter())
            .unwrap()
            .deterministic_public_key();
        let coin_info = CoinInfo {
            coin: "LITECOIN".to_string(),
            derivation_path: "m/44'/2'/0'/0/0".to_string(),
            curve: CurveType::SECP256k1,
            network: "MAINNET".to_string(),
            seg_wit: "NONE".to_string(),
        };
        let ltc_xprv_str = BtcForkAddress::extended_public_key(&anpub, &coin_info).unwrap();
        assert_eq!("xpub6JeaAjhtvtjCDnEo4Bjr7uEbGccaHnJtLY4aBnMaAYGjkBRB3fP9XvjcCbNjMiU1n5tt7dYKVgHPGzh3t3W6eLBxavxABTaoQ2jhbiQrfe4", ltc_xprv_str);
    }

    #[test]
    pub fn script_pub_key() {
        let addr = BtcForkAddress::from_str("MR5Hu9zXPX3o9QuYNJGft1VMpRP418QDfW").unwrap();
        let script = hex::encode(addr.script_pubkey().as_bytes());
        assert_eq!("a914bc64b2d79807cd3d72101c3298b89117d32097fb87", script);

        let addr = BtcForkAddress::from_str("ltc1qum864wd9nwsc0u9ytkctz6wzrw6g7zdn08yddf").unwrap();
        let script = hex::encode(addr.script_pubkey().as_bytes());
        assert_eq!("0014e6cfaab9a59ba187f0a45db0b169c21bb48f09b3", script);

        let addr = BtcForkAddress::from_str("Ldfdegx3hJygDuFDUA7Rkzjjx8gfFhP9DP").unwrap();
        let script = hex::encode(addr.script_pubkey().as_bytes());
        assert_eq!("76a914ca4d8acded69ce4f05d0925946d261f86c675fd888ac", script);

        let addr = BtcForkAddress::from_str("3Js9bGaZSQCNLudeGRHL4NExVinc25RbuG").unwrap();
        let script = hex::encode(addr.script_pubkey().as_bytes());
        assert_eq!("a914bc64b2d79807cd3d72101c3298b89117d32097fb87", script);
    }

    #[test]
    pub fn script_pub_key_component_address_like_test() {
        let _addr = BtcForkAddress::from_str("MR5Hu9zXPX3o9QuYNJGft1VMpRP418QDfW").unwrap();
        let pub_key = bitcoin::PublicKey::from_str(
            "02506bc1dc099358e5137292f4efdd57e400f29ba5132aa5d12b18dac1c1f6aaba",
        )
        .unwrap();
        let script =
            BtcForkAddress::address_script_like("MR5Hu9zXPX3o9QuYNJGft1VMpRP418QDfW", &pub_key)
                .unwrap();

        let script = hex::encode(script.as_bytes());
        assert_eq!("a914bc64b2d79807cd3d72101c3298b89117d32097fb87", script);

        let script = BtcForkAddress::address_script_like(
            "ltc1qum864wd9nwsc0u9ytkctz6wzrw6g7zdn08yddf",
            &pub_key,
        )
        .unwrap();

        let script = hex::encode(script.as_bytes());
        assert_eq!("0014e6cfaab9a59ba187f0a45db0b169c21bb48f09b3", script);

        let script =
            BtcForkAddress::address_script_like("Ldfdegx3hJygDuFDUA7Rkzjjx8gfFhP9DP", &pub_key)
                .unwrap();

        let script = hex::encode(script.as_bytes());
        assert_eq!("76a914e6cfaab9a59ba187f0a45db0b169c21bb48f09b388ac", script);

        let script =
            BtcForkAddress::address_script_like("3Js9bGaZSQCNLudeGRHL4NExVinc25RbuG", &pub_key)
                .unwrap();

        let script = hex::encode(script.as_bytes());
        assert_eq!("a914bc64b2d79807cd3d72101c3298b89117d32097fb87", script);
    }

    #[test]
    pub fn address_valid_test() {
        let coin = coin_info_from_param("BITCOIN", "MAINNET", "P2WPKH").unwrap();
        assert!(BtcForkAddress::is_valid(
            "3Js9bGaZSQCNLudeGRHL4NExVinc25RbuG",
            &coin
        ));
        let coin = coin_info_from_param("LITECOIN", "MAINNET", "NONE").unwrap();
        assert!(BtcForkAddress::is_valid(
            "Ldfdegx3hJygDuFDUA7Rkzjjx8gfFhP9DP",
            &coin
        ));
        let coin = coin_info_from_param("LITECOIN", "MAINNET", "P2WPKH").unwrap();
        assert!(BtcForkAddress::is_valid(
            "MR5Hu9zXPX3o9QuYNJGft1VMpRP418QDfW",
            &coin
        ));

        let coin = coin_info_from_param("LITECOIN", "MAINNET", "P2WPKH").unwrap();
        assert!(!BtcForkAddress::is_valid(
            "MR5Hu9zXPX3o9QuYNJGft1VMpRP418QDf",
            &coin
        ));

        let coin = coin_info_from_param("LITECOIN", "MAINNET", "P2WPKH").unwrap();
        assert!(!BtcForkAddress::is_valid("aaa", &coin));
    }
}
