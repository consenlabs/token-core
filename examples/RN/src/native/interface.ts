export interface ImportWalletResult {
  id: string
  chainType: __chainType
  address: string // public address BITCOIN is Base58 and ETHEREUM is 0x....
  createdAt: number
  source: __walletSource
  externalAddress?: __externalAddress // Bitcoin external address
  publicKeys?: (string | __publicKey)[] // EOS public keys. index 0 as owner key | index 1 as active key
}

export interface ImportWalletFromMnemonicParams {
  chainType: __chainType
  network: __networkType
  name: string
  mnemonic: string
  password: string
  source: __walletSource
  path: string
  overwrite?: boolean
  passwordHint?: string
}

export interface ImportWalletFromKeystoreParams {
  chainType: __chainType
  name: string
  password: string
  keystore: string
  passwordHint?: string
  overwrite?: boolean
}

export interface ImportWalletFromPrivateKeyParams {
  chainType: __chainType
  network: __networkType
  password: string
  privateKey?: string
  overwrite?: boolean
  passwordHint?: string
  permissions?: { permission: string, publicKey: string }[]
  privateKeys?: string[]
}

export interface ExistAddress {
  chainType: __chainType
  address: string
}

export interface ExistsResult {
  exists: boolean
}

export interface ExistsMnemonicParams {
  chainType: __chainType
  network: __networkType
  mnemonic: string
  path: string
}

export interface ExistsKeystoreParams {
  chainType: __chainType
  password: string
  keystore: string
}

export interface ExistsPrivateKeyParams {
  chainType: __chainType
  network: __networkType
  privateKey: string
}

export interface ExportParams {
  id: string
  password: string
}

export interface ExportBtcPrivateKeyParams {
  id: string
  password: string
  address: string
  derivedPath: string
}

export interface ExportPrivateKeyResult {
  privateKey: string
}

export interface ExportMnemonicResult {
  mnemonic: string
  path: string
  address: string
}

export interface ExportKeystoreResult {
  json: string
}

export interface RemoveWalletParams {
  id: string // file id
  password: string
}

export interface RemoveWalletResult {
  id: string
}

interface EthtereumSign {
  id: string // file id
  password?: string
  nonce?: string
  gasPrice?: string
  gasLimit?: string
  value: string
  to: string // receiver address
  data?: string // hex string
  chainId: string // kovan/mainnet
  chainType: __chainType
}

export interface EthtereumSignParams extends EthtereumSign {
  password: string
}

export interface SignTxResult {
  sign: string
  hash: string
}

export interface BitcoinSign {
  id: string // file id
  chainId: string // TESTNET | MAINNET
  to: string
  amount: string
  password: string
  outputs: object[]
  fee: string
  internalUsed: number
  chainType: __chainType
  segWit: __segWit
  protocol: __BitcoinSignProtocol
  extra: __OmniProperties
}

export interface BitcoinSignParams extends BitcoinSign {
  password: string
}

export interface EosSign {
  id: string // file id
  amount?: string
  symbol: string
  from: string
  to: string
  chainType: __chainType
  password: string
  chainId: string
  txHex?: string // TODO
  tr?: any // TODO
  transactions: { txHex: string, publicKeys: (string | __publicKey)[] }[]
}

export interface EosSignParams extends EosSign {
  password: string
}

export interface EosEcSignParams {
  id: string // file id
  data: string
  password: string
  isHex: boolean
  publicKey: string
}

export interface SignParams {
  id: string // file id
  password: string
  data: string // normal string
}

export interface VerifyPasswordParams {
  id: string // file id
  password: string
}

export interface VerifyPasswordResult {
  id: string // file id
}

export interface GenerateMnemonicResult {
  mnemonic: string
}
