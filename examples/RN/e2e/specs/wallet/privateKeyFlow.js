/**
 * mnemonic flow test
 *
 * import -> sign -> export -> delete
 */

import importPrivateKey from './base/importPrivateKey'

import {
  PASSWORD,
  BITCOINCASH_MAINNET_PRIVATEKEY,
  LITECOIN_MAINNET_PRIVATEKEY,
  TRON_MAINNET_PRIVATEKEY,
  BITCOINCASH_MAINNET_PRIVATEKEY_ADDRESS,
  LITECOIN_MAINNET_PRIVATEKEY_ADDRESS,
  TRON_MAINNET_PRIVATEKEY_ADDRESS
} from '../../constant'

export const CHAINTYPES = ['BITCOINCASH', 'LITECOIN', 'TRON']
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
        for (const ptivatekeyIndex in PRIVATEKEYS) {
          
          let chainType = CHAINTYPES[chainIndex]
          let privateKey = PRIVATEKEYS[ptivatekeyIndex]
          let address = ADDRESSES[ptivatekeyIndex + '_ADDRESS']
          let network = ptivatekeyIndex.indexOf('MAINNET') ? 'MAINNET' : 'TESTNET'
          console.log(network)
          it(`should import ${chainType} wallet, network is ${network}, privateKey is ${ptivatekeyIndex} and address is ${address}`, async () => {
            // const { chainType, mnemonic, password, address, network } = params
            await importPrivateKey({
              chainType: chainType,
              privateKey: privateKey,
              password: PASSWORD,
              address: address,
              network: network
            })
          })
      }
    }
  })
}
