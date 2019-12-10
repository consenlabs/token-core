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

const getChainParams = ({ privateKey, password, chainType, network, segWit }) => {
  switch (chainType) {
    case 'TRON':
      return {
        chainType,
        privateKey,
        password,
        network: '',
        segWit: '',
      }
    default:
      return {
        chainType,
        privateKey,
        password,
        network,
        segWit,
      }
  }
}

export default function () {
  describe('⏳ privateKey flow', () => {
    for (const chainIndex in CHAINTYPES) {
      let chainType = CHAINTYPES[chainIndex]
      let privateKey = PRIVATEKEYS[`${chainType}_MAINNET_PRIVATEKEY`]
      let address = ADDRESSES[chainType + '_MAINNET_PRIVATEKEY_ADDRESS']
      let network = 'MAINNET'
      console.log(network)
      it(`should import ${chainType} wallet, network is ${network}, privateKey is ${privateKey} and address is ${address}`, async () => {
        // const { chainType, mnemonic, password, address, network } = params
        const params = getChainParams({
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
