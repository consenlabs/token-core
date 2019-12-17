use bip39::{Language, Mnemonic, MnemonicType};

pub fn generate_mnemonic() -> String {
    Mnemonic::new(MnemonicType::Words12, Language::English).to_string()
}
