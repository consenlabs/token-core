/**
 * mnemonic flow test
 *
 * import -> export -> repeat import -> verify keystore -> delete
 */

import importMnemonic from './base/importMnemonic'
import robustImportMnemonic from './base/robustImportMnemonic'

import {
  PASSWORD,
  REPEAT_PASSWORD,
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

export default function (repeatImport, runRobust) {
  describe('⏳ mnemonic flow', () => {
    for (const chainIndex in CHAINTYPES) {
      for (const networkIndex in NETWORKS) {
        for (const mnemonicIndex in MNEMONICS) {

          let chainType = CHAINTYPES[chainIndex]
          let network = NETWORKS[networkIndex]
          let mnemonic = MNEMONICS[mnemonicIndex]
          let address = ADDRESSES[chainType + '_' + network + '_' + mnemonicIndex + '_ADDRESS']

          it(`should import ${chainType} wallet, network is ${network}, mnemonic is ${mnemonic} and the expected address is ${address}`, async () => {
            const params = formatHdStoreParams({
              mnemonic,
              chainType,
              password: PASSWORD,
              network,
              segWit: 'NONE',
            })
            await importMnemonic({ ...params, address, repeatImport, runRobust, REPEAT_PASSWORD})
          })
        }
      }
    }

    if (runRobust) {
      for (const chainIndex in CHAINTYPES) {

        let chainType = CHAINTYPES[chainIndex]
        let network = 'MAINNET'
        let mnemonic = MNEMONIC_12
        let address = ADDRESSES[chainType + '_' + network + '_MNEMONIC_12_ADDRESS']
  
        it(`should roubust import ${chainType} wallet, network is ${network}, mnemonic is ${mnemonic} and the expected address is ${address}`, async () => {
          const params = formatHdStoreParams({
            mnemonic,
            chainType,
            password: PASSWORD,
            network,
            segWit: 'NONE',
          })
          
            await robustImportMnemonic({ ...params, address })
        
        })
      }
    }
  })
}
