/**
 * privateKey flow test
 *
 * import -> export
 */

import importPrivateKey from './base/importPrivateKey'

import {
  PASSWORD,
  CHAINTYPES,
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
  describe('⏳ privateKey flow', () => {
    for (const chainIndex in CHAINTYPES) {
      let chainType = CHAINTYPES[chainIndex]
      let privateKey = PRIVATEKEYS[`${chainType}_MAINNET_MNEMONIC_12_PRIVATEKEY`]
      let address = ADDRESSES[chainType + '_MAINNET_MNEMONIC_12_ADDRESS']
      let network = 'MAINNET'
      it(`should import ${chainType} wallet, network is ${network}, privateKey is ${privateKey} and the expected address is ${address}`, async () => {
        const params = formatPrivateKeyStoreParams({
          privateKey,
          chainType,
          password: PASSWORD,
          network,
          segWit: 'NONE',
        })
        await importPrivateKey({ ...params, address })
      })
    }
  })
}
