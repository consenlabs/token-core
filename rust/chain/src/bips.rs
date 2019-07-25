use bip39::{Mnemonic, MnemonicType, Language};

pub fn generate_mnemonic() -> String {
    Mnemonic::new(MnemonicType::Words12, Language::English).to_string()
}

