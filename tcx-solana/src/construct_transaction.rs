use bincode::serialize;
use generic_array::{typenum::U64, GenericArray};
use serde::{Deserialize, Serialize};
use solana_program::short_vec;
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum SystemInstruction {
    /// Create a new account
    ///
    /// # Account references
    ///   0. `[WRITE, SIGNER]` Funding account
    ///   1. `[WRITE, SIGNER]` New account
    CreateAccount {
        /// Number of lamports to transfer to the new account
        lamports: u64,

        /// Number of bytes of memory to allocate
        space: u64,

        /// Address of program that will own the new account
        owner: Pubkey,
    },

    /// Assign account to a program
    ///
    /// # Account references
    ///   0. `[WRITE, SIGNER]` Assigned account public key
    Assign {
        /// Owner program account
        owner: Pubkey,
    },

    /// Transfer lamports
    ///
    /// # Account references
    ///   0. `[WRITE, SIGNER]` Funding account
    ///   1. `[WRITE]` Recipient account
    Transfer { lamports: u64 },

    /// Create a new account at an address derived from a base pubkey and a seed
    ///
    /// # Account references
    ///   0. `[WRITE, SIGNER]` Funding account
    ///   1. `[WRITE]` Created account
    ///   2. `[SIGNER]` (optional) Base account; the account matching the base Pubkey below must be
    ///                          provided as a signer, but may be the same as the funding account
    ///                          and provided as account 0
    CreateAccountWithSeed {
        /// Base public key
        base: Pubkey,

        /// String of ASCII chars, no longer than `Pubkey::MAX_SEED_LEN`
        seed: String,

        /// Number of lamports to transfer to the new account
        lamports: u64,

        /// Number of bytes of memory to allocate
        space: u64,

        /// Owner program account address
        owner: Pubkey,
    },

    /// Consumes a stored nonce, replacing it with a successor
    ///
    /// # Account references
    ///   0. `[WRITE]` Nonce account
    ///   1. `[]` RecentBlockhashes sysvar
    ///   2. `[SIGNER]` Nonce authority
    AdvanceNonceAccount,

    /// Withdraw funds from a nonce account
    ///
    /// # Account references
    ///   0. `[WRITE]` Nonce account
    ///   1. `[WRITE]` Recipient account
    ///   2. `[]` RecentBlockhashes sysvar
    ///   3. `[]` Rent sysvar
    ///   4. `[SIGNER]` Nonce authority
    ///
    /// The `u64` parameter is the lamports to withdraw, which must leave the
    /// account balance above the rent exempt reserve or at zero.
    WithdrawNonceAccount(u64),

    /// Drive state of Uninitialized nonce account to Initialized, setting the nonce value
    ///
    /// # Account references
    ///   0. `[WRITE]` Nonce account
    ///   1. `[]` RecentBlockhashes sysvar
    ///   2. `[]` Rent sysvar
    ///
    /// The `Pubkey` parameter specifies the entity authorized to execute nonce
    /// instruction on the account
    ///
    /// No signatures are required to execute this instruction, enabling derived
    /// nonce account addresses
    InitializeNonceAccount(Pubkey),

    /// Change the entity authorized to execute nonce instructions on the account
    ///
    /// # Account references
    ///   0. `[WRITE]` Nonce account
    ///   1. `[SIGNER]` Nonce authority
    ///
    /// The `Pubkey` parameter identifies the entity to authorize
    AuthorizeNonceAccount(Pubkey),

    /// Allocate space in a (possibly new) account without funding
    ///
    /// # Account references
    ///   0. `[WRITE, SIGNER]` New account
    Allocate {
        /// Number of bytes of memory to allocate
        space: u64,
    },

    /// Allocate space for and assign an account at an address
    ///    derived from a base public key and a seed
    ///
    /// # Account references
    ///   0. `[WRITE]` Allocated account
    ///   1. `[SIGNER]` Base account
    AllocateWithSeed {
        /// Base public key
        base: Pubkey,

        /// String of ASCII chars, no longer than `pubkey::MAX_SEED_LEN`
        seed: String,

        /// Number of bytes of memory to allocate
        space: u64,

        /// Owner program account
        owner: Pubkey,
    },

    /// Assign account to a program based on a seed
    ///
    /// # Account references
    ///   0. `[WRITE]` Assigned account
    ///   1. `[SIGNER]` Base account
    AssignWithSeed {
        /// Base public key
        base: Pubkey,

        /// String of ASCII chars, no longer than `pubkey::MAX_SEED_LEN`
        seed: String,

        /// Owner program account
        owner: Pubkey,
    },

    /// Transfer lamports from a derived address
    ///
    /// # Account references
    ///   0. `[WRITE]` Funding account
    ///   1. `[SIGNER]` Base for funding account
    ///   2. `[WRITE]` Recipient account
    TransferWithSeed {
        /// Amount to transfer
        lamports: u64,

        /// Seed to use to derive the funding account address
        from_seed: String,

        /// Owner to use to derive the funding account address
        from_owner: Pubkey,
    },
}
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct SolanaInstruction {
    pub program_id: Pubkey,
    pub accounts: Vec<AccountMeta>,
    pub data: Vec<u8>,
}
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct SolanaTransaction {
    #[serde(with = "short_vec")]
    pub signatures: Vec<Signature>,
    pub message: SolanaMessage,
}
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct SolanaMessage {
    pub header: MessageHeader,
    #[serde(with = "short_vec")]
    pub account_keys: Vec<Pubkey>,
    pub recent_blockhash: [u8; 32],
    #[serde(with = "short_vec")]
    pub instructions: Vec<CompiledInstruction>,
}
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct MessageHeader {
    pub num_required_signatures: u8,
    pub num_readonly_signed_accounts: u8,
    pub num_readonly_unsigned_accounts: u8,
}
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompiledInstruction {
    pub program_id_index: u8,
    #[serde(with = "short_vec")]
    pub accounts: Vec<u8>,
    #[serde(with = "short_vec")]
    pub data: Vec<u8>,
}
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AccountMeta {
    pub pubkey: Pubkey,
    pub is_signer: bool,
    pub is_writable: bool,
}
#[derive(Default)]
struct CompiledKeyMeta {
    is_signer: bool,
    is_writable: bool,
}
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, PartialOrd, Eq, Ord)]
pub struct Pubkey(pub [u8; 32]);

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Default)]
pub struct Signature(GenericArray<u8, U64>);
impl Signature {
    pub fn new(signature_slice: &[u8]) -> Self {
        Self(GenericArray::clone_from_slice(signature_slice))
    }
}
impl From<Signature> for [u8; 64] {
    fn from(signature: Signature) -> Self {
        signature.0.into()
    }
}

fn position(keys: &Vec<Pubkey>, key: &Pubkey) -> u8 {
    keys.iter().position(|k| k == key)? as u8
}

impl SolanaInstruction {
    pub fn new_with_bincode<T: Serialize>(
        program_id: Pubkey,
        data: &T,
        accounts: Vec<AccountMeta>,
    ) -> Self {
        let data = serialize(data)?;
        Self {
            program_id,
            accounts,
            data,
        }
    }
}

pub fn transfer_instruction(
    from_pubkey: &Pubkey,
    to_pubkey: &Pubkey,
    lamports: u64,
) -> SolanaInstruction {
    let account_metas = vec![
        AccountMeta {
            pubkey: from_pubkey.clone(),
            is_signer: true,
            is_writable: true,
        },
        AccountMeta {
            pubkey: to_pubkey.clone(),
            is_signer: false,
            is_writable: true,
        },
    ];
    SolanaInstruction::new_with_bincode(
        Pubkey([0u8; 32]),
        &SystemInstruction::Transfer { lamports },
        account_metas,
    )
}

pub fn transfer_many_instructions(
    from_pubkey: &Pubkey,
    to_lamports: &[(Pubkey, u64)],
) -> Vec<SolanaInstruction> {
    to_lamports
        .iter()
        .map(|(to_pubkey, lamports)| transfer_instruction(from_pubkey, to_pubkey, *lamports))
        .collect()
}

pub fn message_from_instructions(
    instructions: &[SolanaInstruction],
    payer: &Pubkey,
    blockhash: [u8; 32],
) -> SolanaMessage {
    let mut key_meta_map = BTreeMap::<&Pubkey, CompiledKeyMeta>::new();
    for ix in instructions {
        key_meta_map.entry(&ix.program_id).or_default();
        for account_meta in &ix.accounts {
            let meta = key_meta_map.entry(&account_meta.pubkey).or_default();
            meta.is_signer |= account_meta.is_signer;
            meta.is_writable |= account_meta.is_writable;
        }
    }
    key_meta_map.remove(&payer);
    let mut writable_signer_keys: Vec<Pubkey> = Vec::new();
    writable_signer_keys.push(payer.clone());
    writable_signer_keys.extend(
        key_meta_map
            .iter()
            .filter(|(key, meta)| meta.is_signer && meta.is_writable)
            .map(|(key, _)| (*key).clone())
            .collect::<Vec<Pubkey>>(),
    );
    let readonly_signer_keys = key_meta_map
        .iter()
        .filter(|(key, meta)| meta.is_signer && !meta.is_writable)
        .map(|(key, _)| (*key).clone())
        .collect::<Vec<Pubkey>>();
    let writable_non_signer_keys = key_meta_map
        .iter()
        .filter(|(key, meta)| !meta.is_signer && meta.is_writable)
        .map(|(key, _)| (*key).clone())
        .collect::<Vec<Pubkey>>();
    let readonly_non_signer_keys = key_meta_map
        .iter()
        .filter(|(key, meta)| !meta.is_signer && !meta.is_writable)
        .map(|(key, _)| (*key).clone())
        .collect::<Vec<Pubkey>>();
    let num_required_signatures: u8 =
        (writable_signer_keys.len() + readonly_signer_keys.len()) as u8;
    let num_readonly_signed_accounts: u8 = readonly_signer_keys.len() as u8;
    let num_readonly_unsigned_accounts: u8 = readonly_non_signer_keys.len() as u8;
    let static_account_keys = std::iter::empty()
        .chain(writable_signer_keys)
        .chain(readonly_signer_keys)
        .chain(writable_non_signer_keys)
        .chain(readonly_non_signer_keys)
        .collect();
    let mut compiled_instruction: Vec<CompiledInstruction> = Vec::new();
    for ix in instructions {
        let pid = position(&static_account_keys, &ix.program_id);
        let accounts: Vec<u8> = ix
            .accounts
            .iter()
            .map(|account_meta| position(&static_account_keys, &account_meta.pubkey))
            .collect();
        compiled_instruction.push(CompiledInstruction {
            program_id_index: pid,
            data: ix.data.clone(),
            accounts,
        });
    }
    SolanaMessage {
        header: MessageHeader {
            num_required_signatures,
            num_readonly_signed_accounts,
            num_readonly_unsigned_accounts,
        },
        account_keys: static_account_keys,
        recent_blockhash: blockhash,
        instructions: compiled_instruction,
    }
}

// pub fn generate_transaction(
//     from_keypairs: &[&Keypair],
//     message: SolanaMessage,
//     _recent_blockhash: [u8; 32],
// ) -> SolanaTransaction {
//     let serialized_message = bincode::serialize(&message).expect("Serialization failed.");
//     let mut pos = Vec::new();
//     let mut signatures = vec![Signature::default();from_keypairs.len()];
//     for keypair in from_keypairs{
//         let pubkey = Pubkey(keypair.public.to_bytes());
//         pos.push(position(&message.account_keys,&pubkey));
//     }
//     for i in 0..from_keypairs.len(){
//         let sig = from_keypairs[i]
//             .try_sign(&*serialized_message)
//             .expect("Sign failed.");
//         signatures[pos[i] as usize] = Signature::new(sig.to_bytes().as_slice());
//     }
//     SolanaTransaction {
//         signatures,
//         message,
//     }
// }
