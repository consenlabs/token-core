/**
 * mnemonic flow test
 *
 * import -> sign -> export -> delete
 */

import importMnemonic from './base/importMnemonic'
import { CHAINS_DATA } from '../../constant'

export default function () {
  describe('⏳ mnemonic flow', () => {
    CHAINS_DATA.forEach(async (chain) => {
      it(`should import ${chain.chainType} wallet and the address is ${chain.address}`, async () => {
        await importMnemonic(chain)
      })
    })
  })
}
