mod address;
mod singer;
mod transaction;
mod util;
#[macro_use]
extern crate failure;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn eth_address() {}
}
