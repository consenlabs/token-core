/**
 * menmonic cover privateKey flow test
 *
 * 1.import privateKey -> import mnemonic -> verify keystore -> detele
 * 2.import mnemonic -> import privateKey -> verify keystore -> detele
 */

import mnemonicPrivateKey from './base/mnemonicPrivateKey'

import {
  PASSWORD,
  REPEAT_PASSWORD,
  CHAINTYPES,
  MNEMONIC_12,
  BITCOINCASH_MAINNET_MNEMONIC_12_PRIVATEKEY,
  LITECOIN_MAINNET_MNEMONIC_12_PRIVATEKEY,
  TRON_MAINNET_MNEMONIC_12_PRIVATEKEY,
  BITCOINCASH_MAINNET_MNEMONIC_12_ADDRESS,
  LITECOIN_MAINNET_MNEMONIC_12_ADDRESS,
  TRON_MAINNET_MNEMONIC_12_ADDRESS
} from '../../constant'

import { formatPrivateKeyStoreParams } from '../../chain'

export const PRIVATEKEYS = {
  BITCOINCASH_MAINNET_MNEMONIC_12_PRIVATEKEY,
  LITECOIN_MAINNET_MNEMONIC_12_PRIVATEKEY,
  TRON_MAINNET_MNEMONIC_12_PRIVATEKEY
}

export const ADDRESSES = {
  BITCOINCASH_MAINNET_MNEMONIC_12_ADDRESS,
  LITECOIN_MAINNET_MNEMONIC_12_ADDRESS,
  TRON_MAINNET_MNEMONIC_12_ADDRESS
}

export default function () {
  describe('⏳ mnemonic cover privateKey flow', () => {
    for (const chainIndex in CHAINTYPES) {
      let chainType = CHAINTYPES[chainIndex]
      let privateKey = PRIVATEKEYS[`${chainType}_MAINNET_MNEMONIC_12_PRIVATEKEY`]
      let address = ADDRESSES[chainType + '_MAINNET_MNEMONIC_12_ADDRESS']
      let network = 'MAINNET'
      let mnemonic = MNEMONIC_12
      
      it(`should import ${chainType} wallet, network is ${network}`, async () => {
        const params = formatPrivateKeyStoreParams({
          privateKey,
          chainType,
          password: PASSWORD,
          network,
          segWit: 'NONE',
        })
        let coverFlow = 'mnemonicCoverPrivateKey'
        await mnemonicPrivateKey({ ...params, address, mnemonic, REPEAT_PASSWORD, coverFlow})
        coverFlow = 'privateKeyCoverMnemonic'
        await mnemonicPrivateKey({ ...params, address, mnemonic, REPEAT_PASSWORD, coverFlow})
      })
    }
  })
}
