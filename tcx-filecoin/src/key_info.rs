use serde::{Deserialize, Serialize};
use tcx_chain::Result;
use tcx_constants::CurveType;

use super::Error;

#[derive(Deserialize, Serialize, Debug)]
pub struct KeyInfo {
    #[serde(rename = "Type")]
    r#type: String,

    #[serde(rename = "PrivateKey")]
    private_key: String,
}

impl KeyInfo {
    pub fn from_lotus(bytes: &[u8]) -> Result<Self> {
        Ok(serde_json::from_slice::<KeyInfo>(bytes)?)
    }

    pub fn from_private_key(curve_type: CurveType, private_key: &[u8]) -> Result<Self> {
        match curve_type {
            CurveType::SECP256k1 => Ok(KeyInfo {
                r#type: "secp256k1".to_string(),
                private_key: base64::encode(private_key),
            }),
            CurveType::BLS => Ok(KeyInfo {
                r#type: "bls".to_string(),
                private_key: base64::encode(private_key),
            }),
            _ => Err(Error::InvalidCurveType.into()),
        }
    }

    pub fn to_json(&self) -> Result<Vec<u8>> {
        Ok(serde_json::to_vec(self)?)
    }

    pub fn decode_private_key(&self) -> Result<Vec<u8>> {
        Ok(base64::decode(&self.private_key)?.to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::KeyInfo;
    use tcx_constants::CurveType;

    #[test]
    fn test_import_and_export() {
        let key_info = KeyInfo::from_lotus(&hex::decode("7b2254797065223a22736563703235366b31222c22507269766174654b6579223a22437544586b6b4b46773549656d55685a545173514369534e6d6a327062545052495439514f736c587846733d227d").unwrap()).unwrap();
        assert_eq!(key_info.r#type, "secp256k1");
        assert_eq!(
            hex::encode(key_info.decode_private_key().unwrap()),
            "0ae0d7924285c3921e9948594d0b100a248d9a3da96d33d1213f503ac957c45b"
        );

        assert_eq!(hex::encode(key_info.to_json().unwrap()), "7b2254797065223a22736563703235366b31222c22507269766174654b6579223a22437544586b6b4b46773549656d55685a545173514369534e6d6a327062545052495439514f736c587846733d227d");
    }

    #[test]
    fn test_from_private_key() {
        let key_info = KeyInfo::from_private_key(
            CurveType::SECP256k1,
            &hex::decode("0ae0d7924285c3921e9948594d0b100a248d9a3da96d33d1213f503ac957c45b")
                .unwrap(),
        )
        .unwrap();

        assert_eq!(hex::encode(key_info.to_json().unwrap()), "7b2254797065223a22736563703235366b31222c22507269766174654b6579223a22437544586b6b4b46773549656d55685a545173514369534e6d6a327062545052495439514f736c587846733d227d");
    }
}
