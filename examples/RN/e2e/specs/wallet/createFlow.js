/**
 * create flow test
 *
 * create -> export -> import
 */

import create from './base/create'
import { PASSWORD, CHAINTYPES, NETWORKS } from '../../constant'
import { formatHdStoreParams } from '../../chain'

export default function () {
  describe('⏳ create flow', () => {
    for (const chainIndex in CHAINTYPES) {
      for (const networkIndex in NETWORKS) {
        let chainType = CHAINTYPES[chainIndex]
        let network = NETWORKS[networkIndex]

        it(`should create ${chainType} wallet, network is ${network}`, async () => {
          const params = formatHdStoreParams({
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
