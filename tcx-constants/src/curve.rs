use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum CurveType {
    SECP256k1,          /* "secp256k1" */
    ED25519,            /* "ed25519" */
    ED25519Blake2bNano, /* "ed25519-blake2b-nano" */
    SubSr25519,
    Curve25519, /* "curve25519" */
    NIST256p1,
    BLS, /* "bls" */
}

impl CurveType {
    pub fn as_str(&self) -> &str {
        match self {
            CurveType::SECP256k1 => "SECP256k1",
            CurveType::ED25519 => "ED25519 ",
            CurveType::ED25519Blake2bNano => "ED25519Blake2bNano",
            CurveType::SubSr25519 => "SubSr25519",
            CurveType::Curve25519 => "Curve25519",
            CurveType::NIST256p1 => "NIST256p1",
            CurveType::BLS => "BLS",
        }
    }
}
