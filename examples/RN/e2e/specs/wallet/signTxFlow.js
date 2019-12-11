/**
 * mnemonic flow test
 *
 * import -> sign -> export -> delete
 */

import signTx from './base/signTx'

import {
  SIGN_TX,
} from '../../constant'

const getChainParams = ({ mnemonic, password, chainType, network, segWit }) => {
  switch (chainType) {
    case 'TRON':
      return {
        mnemonic,
        chainType,
        password,
        network: '',
        segWit: '',
      }
    default:
      return {
        mnemonic,
        chainType,
        password,
        network,
        segWit,
      }
  }
}

export default function () {
  describe('⏳ signTx flow', () => {
    for (const txIndex in SIGN_TX) {
      const tx = SIGN_TX[txIndex]
      const { mnemonic, chainType, password, network, segWit, input, address, signature } = tx

      it(`should sign ${input} by ${chainType} wallet, and the expected signature is ${signature}`, async () => {
        const params = getChainParams({
          mnemonic,
          chainType,
          password,
          network,
          segWit,
        })
        await signTx({ ...params, input: JSON.stringify(input), address, signature })
      })
    }
  })
}
