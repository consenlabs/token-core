/**
 * mnemonic flow test
 *
 * import -> sign -> export -> delete
 */

import importMnemonic from './base/importMnemonic'

import {
  PASSWORD,
  MNEMONIC_12,
  MNEMONIC_24,
  BITCOINCASH_TESTNET_MNEMONIC_12_ADDRESS,
  LITECOIN_TESTNET_MNEMONIC_12_ADDRESS,
  TRON_TESTNET_MNEMONIC_12_ADDRESS,
  BITCOINCASH_MAINNET_MNEMONIC_12_ADDRESS,
  LITECOIN_MAINNET_MNEMONIC_12_ADDRESS,
  TRON_MAINNET_MNEMONIC_12_ADDRESS,
  BITCOINCASH_TESTNET_MNEMONIC_24_ADDRESS,
  LITECOIN_TESTNET_MNEMONIC_24_ADDRESS,
  TRON_TESTNET_MNEMONIC_24_ADDRESS,
  BITCOINCASH_MAINNET_MNEMONIC_24_ADDRESS,
  LITECOIN_MAINNET_MNEMONIC_24_ADDRESS,
  TRON_MAINNET_MNEMONIC_24_ADDRESS,
} from '../../constant'

export const CHAINTYPES = ['BITCOINCASH', 'LITECOIN', 'TRON']
export const NETWORKS = ['MAINNET', 'TESTNET']
export const MNEMONICS = {
  MNEMONIC_12,
  MNEMONIC_24
}
export const ADDRESSES = {
  BITCOINCASH_MAINNET_MNEMONIC_12_ADDRESS,
  LITECOIN_MAINNET_MNEMONIC_12_ADDRESS,
  TRON_MAINNET_MNEMONIC_12_ADDRESS,
  BITCOINCASH_TESTNET_MNEMONIC_12_ADDRESS,
  LITECOIN_TESTNET_MNEMONIC_12_ADDRESS,
  TRON_TESTNET_MNEMONIC_12_ADDRESS,
  BITCOINCASH_MAINNET_MNEMONIC_24_ADDRESS,
  LITECOIN_MAINNET_MNEMONIC_24_ADDRESS,
  TRON_MAINNET_MNEMONIC_24_ADDRESS,
  BITCOINCASH_TESTNET_MNEMONIC_24_ADDRESS,
  LITECOIN_TESTNET_MNEMONIC_24_ADDRESS,
  TRON_TESTNET_MNEMONIC_24_ADDRESS,
}


const getChainParams = ({ mnemonic, password, chainType, network, segWit }) => {
  switch (chainType) {
    case 'BITCOINCASH':
    case 'LITECOIN':
      return {
        chainType,
        mnemonic,
        password,
        network,
        segWit,
      }
    case 'TRON':
      return {
        chainType,
        mnemonic,
        password,
        network: '',
        segWit: '',
      }
    default:
      return {
        chainType,
        mnemonic,
        password,
        network,
        segWit,
      }
  }
}

export default function () {
  describe('⏳ mnemonic flow', () => {
    for (const chainIndex in CHAINTYPES) {
      for (const networkIndex in NETWORKS) {
        for (const mnemonicIndex in MNEMONICS) {

          let chainType = CHAINTYPES[chainIndex]
          let network = NETWORKS[networkIndex]
          let mnemonic = MNEMONICS[mnemonicIndex]
          let address = ADDRESSES[chainType + '_' + network + '_' + mnemonicIndex + '_ADDRESS']

          it(`should import ${chainType} wallet, network is ${network}, mnemonic is ${mnemonicIndex} and address is ${address}`, async () => {
            // const { chainType, mnemonic, password, address, network } = params
            const params = getChainParams({
              mnemonic,
              chainType,
              password: PASSWORD,
              network,
              segWit: 'NONE',
            })
            await importMnemonic({ ...params, address })
          })
        }
      }
    }
  })
}
