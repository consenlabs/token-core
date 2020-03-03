declare module '*.json' {
  const value: any
  export default value
}
type fn = (...args: any[]) => any

type __chainType = 'ETHEREUM' | 'BITCOIN' | 'LITECOIN' | 'EOS' | 'COSMOS' | 'TRON' | 'BITCOINCASH' | 'NERVOS' | 'KUSAMA' | 'POLKADOT'
type __chainName = 'ETH' | 'BTC' | 'LITECOIN' | 'EOS' | 'COSMOS' | 'TRON' | 'BCH' | 'CKB' | 'KSM' | 'DOT'
type __walletSource = 'PRIVATE' | 'WIF' | 'MNEMONIC' | 'KEYSTORE'
type __networkType = 'TESTNET' | 'MAINNET'
type __externalAddress = {
  address: string,
  derivedPath: string,
  type: string,
  used?: boolean,
  balance?: string | number
}
type __publicKey = {
  path: string
  derivedMode: 'PATH_DIRECTLY' | 'HD_SHA256'
  publicKey: string
}
type __segWit = 'P2WPKH' | 'NONE'
type __BitcoinSignProtocol = 'BITCOIN' | 'OMNI'
type __OmniProperties = {
  propertyId: number
}
