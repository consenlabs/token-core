use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum CurveType {
    SECP256k1,          /* "secp256k1" */
    ED25519,            /* "ed25519" */
    ED25519Blake2bNano, /* "ed25519-blake2b-nano" */
    SR25519,
    Curve25519, /* "curve25519" */
    NIST256p1,
}
