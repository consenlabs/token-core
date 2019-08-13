use super::Error;

pub type B256 = [u8; 32];

pub type B512 = [u8; 64];

pub type B128 = [u8; 16];

pub type B160 = [u8; 20];

pub type Result<T> = core::result::Result<T, Error>;

