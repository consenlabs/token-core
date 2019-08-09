
use crate::Result;
use bitcoin_hashes::hex::ToHex;
use bitcoin::util::bip32::{ExtendedPubKey, Fingerprint, ChildNumber, ChainCode};
use bitcoin::PublicKey;
use bitcoin_hashes::ripemd160::Hash as Hash160;
use bitcoin_hashes::sha256::Hash as Sha256;
use bitcoin_hashes::hex::FromHex;
use bitcoin_hashes::Hash;
use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::Num;


pub struct HardWalletKeystore {}

pub struct Header {
    cla: u8,
    ins: u8,
    p1: u8,
    p2: u8,
    lc: u8,
}

impl Header {
    pub fn new(cla: u8, ins: u8, p1: u8, p2: u8, lc: u8) -> Header {
        Header {
            cla,
            ins,
            p1,
            p2,
            lc,
        }
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        vec![self.cla, self.ins, self.p1, self.p2, self.lc]
    }

    pub fn as_hex_str(&self) -> String {
        self.as_bytes().to_hex()
    }
}


const BTC_AID: &'static str = "695F627463";
const APDU_RSP_SUCCESS: &'static str = "9000";
const APDU_RSP_USER_NOT_CONFIRMED: &'static str = "6940";
const APDU_CONDITIONS_NOT_SATISFIED: &'static str = "6985";
const APDU_RSP_APPLET_NOT_EXIST: &'static str = "6A82";
const APDU_RSP_INCORRECT_P1P2: &'static str = "6A86";
const APDU_RSP_CLA_NOT_SUPPORTED: &'static str = "6E00";
const APDU_RSP_APPLET_WRONG_DATA: &'static str = "6A80";
const APDU_RSP_WRONG_LENGTH: &'static str = "6700";
const APDU_RSP_SIGNATURE_VERIFY_FAILED: &'static str = "6942";
const APDU_RSP_FUNCTION_NOT_SUPPORTED: &'static str = "6D00";

const APDU_RSP_WALLET_NOT_CREATED: &'static str = "F000";
const APDU_RSP_IN_MENU_PAGE: &'static str = "F080";
const APDU_RSP_PIN_NOT_VERIFIED: &'static str = "F081";
const APDU_BLUETOOTH_CHANNEL_ERROR: &'static str = "6F01";


fn valid_response(res: &str) -> Result<()> {
    if res.ends_with(APDU_RSP_SUCCESS) {
        return Ok(());
    }

    match res {
        APDU_RSP_USER_NOT_CONFIRMED => Err(format_err!("{}", "imkey_user_not_confirmed")),
        APDU_CONDITIONS_NOT_SATISFIED => Err(format_err!("{}", "imkey_conditions_not_satisfied")),
        APDU_RSP_APPLET_NOT_EXIST => Err(format_err!("{}", "imkey_applet_not_exist")),
        APDU_RSP_INCORRECT_P1P2 => Err(format_err!("{}", "imkey_command_format_error")),
        APDU_RSP_CLA_NOT_SUPPORTED => Err(format_err!("{}", "imkey_command_format_error")),
        APDU_RSP_APPLET_WRONG_DATA => Err(format_err!("{}", "imkey_command_data_error")),
        APDU_RSP_WRONG_LENGTH => Err(format_err!("{}", "imkey_apdu_wrong_length")),
        APDU_RSP_SIGNATURE_VERIFY_FAILED => Err(format_err!("{}", "imkey_signature_verify_fail")),
        APDU_RSP_FUNCTION_NOT_SUPPORTED => Err(format_err!("{}", "imkey_applet_function_not_supported")),
        APDU_RSP_WALLET_NOT_CREATED => Err(format_err!("{}", "imkey_wallet_not_created")),
        APDU_RSP_IN_MENU_PAGE => Err(format_err!("{}", "imkey_in_menu_page")),
        APDU_RSP_PIN_NOT_VERIFIED => Err(format_err!("{}", "imkey_pin_not_verified")),
        APDU_BLUETOOTH_CHANNEL_ERROR => Err(format_err!("{}", "imkey_bluetooth_channel_error")),
        _ => Err(format_err!("{}", "unknown_error")),
    }
}

// todo: apdu signature verify
//fn verify_apdu_sig(data: &[u8], signature: &[u8]) -> Result<()> {
//    let pubKey = "";
//}

fn get_parent_path(path: &str) -> String {
    let mut parent_path: Vec<&str> = path.split("/").collect();
    parent_path.pop();
    parent_path.join("/")
}

fn compress_pub_key(hex: &str) -> String {
    let x = &hex[2..66];
    let y = &hex[66..130];
//    let y_bytes = <[u8; 32]>::from_hex(y).unwrap();
    let b = BigInt::from_str_radix( y, 16).unwrap();
    if b.is_odd() {
        format!("{}{}", "03", x)
    } else {
        format!("{}{}", "02", x)
    }

}


impl HardWalletKeystore {
    pub fn select_applet() -> String {
        let aid_len = BTC_AID.len();
        let header = Header::new(0x00, 0xA4, 0x04, 0x00, aid_len as u8);
        format!("{}{}", header.as_hex_str(), BTC_AID.to_owned()).to_uppercase()
    }
    pub fn xpub_apdu(path: &str, verify_key: bool) -> Vec<String> {
        let parent_path = get_parent_path(path);
        [parent_path.as_bytes(), path.as_bytes()].iter().map(|bytes| {
            let _apdu_bytes: Vec<u8> = Vec::with_capacity(6 + bytes.len());
            let p1 = if verify_key { 0x01 } else { 0x00 };
            let header = Header::new(0x80, 0x43, p1, 0x00, bytes.len() as u8);
            [header.as_bytes(), bytes.to_vec(), vec![0x00]].concat().to_hex().to_uppercase()
        }).collect()
    }

    pub fn apdu_to_xpub(apdus: &[&str], _path: &str, _network: &str) -> Result<String> {
        apdus.iter().for_each(|apdu| {
            valid_response(apdu);
        });
        let xpub_hexs: Vec<&str> = apdus.iter().map(|apdu| {

            // 65 bytes pub + 32 bytes chaincode + signature
            let _signature = &apdu[97 * 2..apdu.len() - 4];
            let _data = &apdu[0..97 * 2];
            &apdu[0..194]
        }).collect();

        let parent_pub_hex = xpub_hexs[0];
        let compressed_parent_pub_key = compress_pub_key(parent_pub_hex);
//        let parent_pub_hex = format!("{}{}", prefix, &parent_pub_hex[2..130]);
        println!("{}", compressed_parent_pub_key);
        let parent_pub_bytes =  <[u8; 33]>::from_hex(&compressed_parent_pub_key)?;
        let parent_pub_key = PublicKey::from_slice(&parent_pub_bytes)?;
        let pub_key_hex = xpub_hexs[1];
        let compressed_pub_key = compress_pub_key(pub_key_hex);
        let compressed_pub_key_bytes = <[u8; 33]>::from_hex(&compressed_pub_key)?;
        let pub_key = PublicKey::from_slice(&compressed_pub_key_bytes)?;
        let chain_code:Vec<u8> = FromHex::from_hex(&xpub_hexs[1][130..194])?;

        let hash = Hash160::hash(&Sha256::hash(&parent_pub_key.to_bytes()).into_inner());


        // hash 160:6738daf77ddc61d3b79a2b11dd7ca67a567eb2db
        // fp: 1731779319
        // cn: -2147483648
        // cp 02A4AC7774E4DD4FC8AFCF39D4264CF217E408D23DA372CEC935C4F24189BB17B4
        // cc 407D0B0B0A67510BDCC345733162BA881E4420C8FD75D7DB93FAAA7CFA512E0F
        let fingerprint = &hash.into_inner()[0..4];

        let extend_pub_key = ExtendedPubKey {
            network: bitcoin::Network::Bitcoin,
            depth: 3,
            parent_fingerprint: Fingerprint::from(fingerprint),
            child_number: ChildNumber::from_hardened_idx(0)?,
            public_key: pub_key,
            chain_code: ChainCode::from(chain_code.as_slice()),
        };

        Ok(extend_pub_key.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;


    static BCH_PATH: &'static str = "m/44'/145'/0'";
    static BTC_PATH: &'static str = "m/49'/0'/0'";

    #[test]
    pub fn select_aid_apdu() {
        let apdu = HardWalletKeystore::select_applet();
        assert_eq!(apdu, "00A404000A695F627463")
    }

    #[test]
    pub fn xpub_apdu() {
        let apdu = HardWalletKeystore::xpub_apdu(BTC_PATH, true);
        let expected = vec!["80430100086D2F3439272F302700", "804301000B6D2F3439272F30272F302700"];
        assert_eq!(apdu, expected);

        let xpub_res = "04A4AC7774E4DD4FC8AFCF39D4264CF217E408D23DA372CEC935C4F24189BB17B4D40C6985A1E0E6458B9AFE3F548CC66A971917FF471B068102A6604C0353F4EC407D0B0B0A67510BDCC345733162BA881E4420C8FD75D7DB93FAAA7CFA512E0F3046022100B949A251CADF273C093004A00877F1B6CFAD7809972A3B34A5E5495791741CD9022100F6F3D0FBBDA536461149AEA1BEA578E2950C7308F5E8EBF5846A941D9120ED6E9000";
        let parent_xpub_res = "042C15224C69B254D8466FD70F03DF2EECD945373FCE247CED07DFA9DE420E4283FDFC98E28A99082C491EA3E2521F47805A79346DC16A72F292F21190D98AF872848713B1761239BC6615CD93BBA189E1DFFD65EFC703B84EE0BBB82B863EFCC43046022100B468DCB7C99D9E71EE0604168BDAF8C1B4D985DA08F185DB773AE4F88A6AA24D022100DC2A7C376C9587E7055015824E9065B64D09D73732CF7C1452DD5177B52EDA819000";
        let responses = vec![parent_xpub_res, xpub_res];
        let xpub = HardWalletKeystore::apdu_to_xpub(&responses, BTC_PATH, "MAINNET").unwrap();

        let expected = "xpub6CQmpQ34RDxVHAHBD86BjkTk6hQvDWQ3xCynZPu6H9uD5HHDx5pz5xT8EutdzHYHkWZqZW2xL4PHmUxTw2tYs3iHuRrmuMX4Sws2x2R3f3r";

        assert_eq!(xpub, expected);
        // m/49'/0'/0' 804301000B6D2F3439272F30272F302700
        // res 04A4AC7774E4DD4FC8AFCF39D4264CF217E408D23DA372CEC935C4F24189BB17B4D40C6985A1E0E6458B9AFE3F548CC66A971917FF471B068102A6604C0353F4EC407D0B0B0A67510BDCC345733162BA881E4420C8FD75D7DB93FAAA7CFA512E0F3046022100B949A251CADF273C093004A00877F1B6CFAD7809972A3B34A5E5495791741CD9022100F6F3D0FBBDA536461149AEA1BEA578E2950C7308F5E8EBF5846A941D9120ED6E9000

        // m/49'/0' 80430100086D2F3439272F302700
        // res 042C15224C69B254D8466FD70F03DF2EECD945373FCE247CED07DFA9DE420E4283FDFC98E28A99082C491EA3E2521F47805A79346DC16A72F292F21190D98AF872848713B1761239BC6615CD93BBA189E1DFFD65EFC703B84EE0BBB82B863EFCC43046022100B468DCB7C99D9E71EE0604168BDAF8C1B4D985DA08F185DB773AE4F88A6AA24D022100DC2A7C376C9587E7055015824E9065B64D09D73732CF7C1452DD5177B52EDA819000

        // xpub6CQmpQ34RDxVHAHBD86BjkTk6hQvDWQ3xCynZPu6H9uD5HHDx5pz5xT8EutdzHYHkWZqZW2xL4PHmUxTw2tYs3iHuRrmuMX4Sws2x2R3f3r
    }
}