/**
 * mnemonic flow test
 *
 * import -> sign -> export -> delete
 */

import create from './base/create'
import { PASSWORD } from '../../constant'

export const CHAINTYPES = ['BITCOINCASH', 'LITECOIN', 'TRON']
export const NETWORKS = ['MAINNET', 'TESTNET']

const getChainParams = ({ password, chainType, network, segWit }) => {
  switch (chainType) {
    case 'BITCOINCASH':
    case 'LITECOIN':
      return {
        chainType,
        password,
        network,
        segWit,
      }
    case 'TRON':
      return {
        chainType,
        password,
        network: '',
        segWit: '',
      }
    default:
      return {
        chainType,
        password,
        network,
        segWit,
      }
  }
}

export default function () {
  describe('⏳ create flow', () => {
    for (const chainIndex in CHAINTYPES) {
      for (const networkIndex in NETWORKS) {
        let chainType = CHAINTYPES[chainIndex]
        let network = NETWORKS[networkIndex]

        it(`should create ${chainType} wallet, network is ${network}`, async () => {
          // const { chainType, mnemonic, password, address, network } = params
          const params = getChainParams({
            chainType,
            password: PASSWORD,
            network,
            segWit: 'NONE',
          })
          await create({ ...params })
        })
      }
    }
  })
}
