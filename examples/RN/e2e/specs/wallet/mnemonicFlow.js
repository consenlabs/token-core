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
  CHAINTYPES,
  NETWORKS,
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
import { formatHdStoreParams } from '../../chain'

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

export default function () {
  describe('⏳ mnemonic flow', () => {
    for (const chainIndex in CHAINTYPES) {
      for (const networkIndex in NETWORKS) {
        for (const mnemonicIndex in MNEMONICS) {

          let chainType = CHAINTYPES[chainIndex]
          let network = NETWORKS[networkIndex]
          let mnemonic = MNEMONICS[mnemonicIndex]
          let address = ADDRESSES[chainType + '_' + network + '_' + mnemonicIndex + '_ADDRESS']

          it(`should import ${chainType} wallet, network is ${network}, mnemonic is ${mnemonicIndex} and the expected address is ${address}`, async () => {
            // const { chainType, mnemonic, password, address, network } = params
            const params = formatHdStoreParams({
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
