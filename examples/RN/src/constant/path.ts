
export const ETHEREUM_DEFAULT_PATH = "m/44'/60'/0'/0/0"
export const ETHEREUM_LEDGER_PATH = "m/44'/60'/0'/0"
export const ETHEREUM_CUSTOM_PATH = "m/44'/60'/1'/0/0"

export const BITCOIN_MAINNET_PATH = "m/44'/0'/0'"
export const BITCOIN_TESTNET_PATH = "m/44'/1'/0'"

export const BITCOIN_SEGWIT_MAINNET_PATH = "m/49'/0'/0'"
export const BITCOIN_SEGWIT_TESTNET_PATH = "m/49'/1'/0'"

export const LITECOIN_MAINNET_PATH = "m/44'/2'/0'/0/0"
export const LITECOIN_TESTNET_PATH = "m/44'/1'/0'/0/0"

export const LITECOIN_SEGWIT_MAINNET_PATH = "m/49'/2'/0'/0/0"
export const LITECOIN_SEGWIT_TESTNET_PATH = "m/49'/1'/0'/0/0"

export const BITCOINCASH_MAINNET_PATH = "m/44'/145'/0'/0/0"
export const BITCOINCASH_TESTNET_PATH = "m/44'/1'/0'/0/0"

export const TRON_DEFAULT_PATH = "m/44'/195'/0'/0/0"

export const EOS_DEFAULT_PATH = "m/44'/194'/0'/0/0"

export const COSMOS_DEFAULT_PATH = "m/44'/118'/0'/0/0"

export const NERVOS_DEFAULT_PATH = "m/44'/309'/0'/0/0"

export const KUSAMA_DEFAULT_PATH = "//kusama//imToken/0"

export const POLKADOT_DEFAULT_PATH = "//polkadot//imToken/0"

// wallet path
export const PATH = {
  BITCOIN: {
    NONE: {
      0: BITCOIN_MAINNET_PATH,
      1: BITCOIN_TESTNET_PATH,
    },
    P2WPKH: {
      0: BITCOIN_SEGWIT_MAINNET_PATH,
      1: BITCOIN_SEGWIT_TESTNET_PATH,
    },
  },
  LITECOIN: {
    NONE: {
      2: LITECOIN_MAINNET_PATH,
      1: LITECOIN_TESTNET_PATH,
    },
    P2WPKH: {
      2: LITECOIN_SEGWIT_MAINNET_PATH,
      1: LITECOIN_SEGWIT_TESTNET_PATH,
    },
  },
  ETHEREUM: ETHEREUM_DEFAULT_PATH,
  EOS: EOS_DEFAULT_PATH,
  COSMOS: COSMOS_DEFAULT_PATH,
  TRON: TRON_DEFAULT_PATH,
  NERVOS: NERVOS_DEFAULT_PATH,
  KUSAMA: KUSAMA_DEFAULT_PATH,
  POLKADOT: POLKADOT_DEFAULT_PATH,
  BITCOINCASH: {
    145: BITCOINCASH_MAINNET_PATH,
    1: BITCOINCASH_TESTNET_PATH,
  }
}

export const getChainPath = (chainType: __chainType, network = 'TESTNET' as __networkType, segWit = 'P2WPKH' as __segWit) => {
  let chainId = network === 'MAINNET' ? 0 : 1
  switch (chainType) {
    case 'BITCOIN':
      chainId = network === 'MAINNET' ? 0 : 1
      return PATH[chainType][segWit][chainId as 0 | 1]

    case 'LITECOIN':
      chainId = network === 'MAINNET' ? 2 : 1
      return PATH[chainType]['NONE'][chainId as 1 | 2]

    case 'BITCOINCASH':
      chainId = network === 'MAINNET' ? 145 : 1
      return PATH[chainType][chainId as 1 | 145]

    case 'NERVOS':
      return PATH[chainType]

    case 'KUSAMA':
      return PATH[chainType]

    case 'POLKADOT':
      return PATH[chainType]

    default:
      return PATH[chainType]
  }
}
