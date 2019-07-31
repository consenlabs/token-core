use bip39::{Mnemonic, MnemonicType, Language};
use core::fmt;
use bitcoin::util::base58;
use bitcoin::util::bip32::{ExtendedPubKey, ExtendedPrivKey};

pub fn generate_mnemonic() -> String {
    Mnemonic::new(MnemonicType::Words12, Language::English).to_string()
}

pub struct DerivationInfo {
    depth: u8,
    parent_fingerprint: [u8;4],
    child_number: u32,
    chain_code: [u8;32],
    key: [u8;33]
}


impl DerivationInfo {
    pub fn encode_with_network(&self, network: [u8;4]) -> String {
        let mut ret = [0; 78];
        ret[0..4].copy_from_slice(&network);
        ret[4] = self.depth as u8;
        ret[5..9].copy_from_slice(&self.parent_fingerprint[..]);

        BigEndian::write_u32(&mut ret[9..13], u32::from(self.child_number));

        ret[13..45].copy_from_slice(&self.chain_code[..]);
        ret[45..78].copy_from_slice(&self.public_key.key.serialize()[..]);
        base58::check_encode_slice(&ret[..]).to_string()
    }
}

impl From<ExtendedPubKey> for DerivationInfo {
    fn from(epk : ExtendedPubKey) -> Self {
        DerivationInfo {
            depth: epk.depth,
            parent_fingerprint: epk.parent_fingerprint[..],
            child_number: u32::from(epk.child_number),
            chain_code: epk.chain_code[..],
            key: epk.public_key.key.serialize()
        }
    }
}

impl From<ExtendedPrivKey> for DerivationInfo {
    fn from(epk: ExtendedPrivKey) -> Self {
        let mut key = [0u8; 33];
        key[0] = 0u8;
        key[1..33].copy_from_slice(epk.private_key.key.serialize()[..]);
        DerivationInfo {
            depth: epk.depth,
            parent_fingerprint: epk.parent_fingerprint[..],
            child_number: u32::from(epk.child_number),
            chain_code: epk.chain_code[..],
            key
        }
    }
}
//
//impl fmt::Display for ExtendedPrivKey {
//    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
//        let mut ret = [0; 78];
//        ret[0..4].copy_from_slice(&match self.network {
//            Network::Bitcoin => [0x04, 0x88, 0xAD, 0xE4],
//            Network::Testnet | Network::Regtest => [0x04, 0x35, 0x83, 0x94],
//        }[..]);
//        ret[4] = self.depth as u8;
//        ret[5..9].copy_from_slice(&self.parent_fingerprint[..]);
//
//        BigEndian::write_u32(&mut ret[9..13], u32::from(self.child_number));
//
//        ret[13..45].copy_from_slice(&self.chain_code[..]);
//        ret[45] = 0;
//        ret[46..78].copy_from_slice(&self.private_key[..]);
//        fmt.write_str(&base58::check_encode_slice(&ret[..]))
//    }
//}