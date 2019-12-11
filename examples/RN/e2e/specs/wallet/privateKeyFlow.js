/**
 * mnemonic flow test
 *
 * import -> sign -> export -> delete
 */

import importPrivateKey from './base/importPrivateKey'

import {
  PASSWORD,
  CHAINTYPES,
  BITCOINCASH_MAINNET_PRIVATEKEY,
  LITECOIN_MAINNET_PRIVATEKEY,
  TRON_MAINNET_PRIVATEKEY,
  BITCOINCASH_MAINNET_PRIVATEKEY_ADDRESS,
  LITECOIN_MAINNET_PRIVATEKEY_ADDRESS,
  TRON_MAINNET_PRIVATEKEY_ADDRESS
} from '../../constant'
import { formatPrivateKeyStoreParams } from '../../chain'

export const PRIVATEKEYS = {
  BITCOINCASH_MAINNET_PRIVATEKEY,
  LITECOIN_MAINNET_PRIVATEKEY,
  TRON_MAINNET_PRIVATEKEY
}
export const ADDRESSES = {
  BITCOINCASH_MAINNET_PRIVATEKEY_ADDRESS,
  LITECOIN_MAINNET_PRIVATEKEY_ADDRESS,
  TRON_MAINNET_PRIVATEKEY_ADDRESS
}

export default function () {
  describe('⏳ privateKey flow', () => {
    for (const chainIndex in CHAINTYPES) {
      let chainType = CHAINTYPES[chainIndex]
      let privateKey = PRIVATEKEYS[`${chainType}_MAINNET_PRIVATEKEY`]
      let address = ADDRESSES[chainType + '_MAINNET_PRIVATEKEY_ADDRESS']
      let network = 'MAINNET'
      it(`should import ${chainType} wallet, network is ${network}, privateKey is ${privateKey} and the expected is ${address}`, async () => {
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
