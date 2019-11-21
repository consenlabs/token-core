pub mod btc_fork_network;
pub mod coin_info;
pub mod curve;

pub use btc_fork_network::{
    coin_from_xpub_prefix, network_form_hrp, network_from_coin, pub_version_from_prv_version,
    BtcForkNetwork,
};
pub use coin_info::{coin_info_from_symbol, coin_symbol_with_network, CoinInfo};
pub use curve::CurveType;

type Result<T> = std::result::Result<T, failure::Error>;

#[macro_use]
extern crate lazy_static;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
