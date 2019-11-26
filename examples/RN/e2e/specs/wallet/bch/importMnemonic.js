import importMnemonic from '../base/importMnemonic'
import {
  MNEMONIC,
  ADDRESS_BCH,
  PASSWORD,
} from '../../../constant'

export default function () {
  describe('⏳ Import BCH Wallet by mnemonic flow ', () => {
    it('should import wallet by mnemonic', async () => {
      await importMnemonic({
        chainType: 'BITCOINCASH',
        mnemonic: MNEMONIC,
        password: PASSWORD,
        address: ADDRESS_BCH,
      })
    })
  })
}
