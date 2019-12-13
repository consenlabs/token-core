/**
 * sign message flow test
 *
 * import -> sign
 */

import tronSignMessage from './base/tronSignMessage'

import {
  PASSWORD,
  MNEMONIC_12,
  TRON_TESTNET_MNEMONIC_12_ADDRESS,
} from '../../constant'

const SIGN_MESSAGE = [
  {
    mnemonic: MNEMONIC_12,
    signature: "16417c6489da3a88ef980bf0a42551b9e76181d03e7334548ab3cb36e7622a484482722882a29e2fe4587b95c739a68624ebf9ada5f013a9340d883f03fcf9af1b",
    password: PASSWORD,
    chainType: "TRON",
    address: TRON_TESTNET_MNEMONIC_12_ADDRESS,
    input: {
      value: '645c0b7b58158babbfa6c6cd5a48aa7340a8749176b120e8516216787a13dc76',
      isHex: true,
      isTronHeader: true,
    }
  }
]

export default function () {
  describe('⏳ tronSignMessage flow', () => {
    for (const txIndex in SIGN_MESSAGE) {
      const tx = SIGN_MESSAGE[txIndex]
      const { mnemonic, chainType, password, input, address, signature } = tx
      const inputStr = JSON.stringify(input)

      it(`should sign ${inputStr} by ${chainType} wallet, and the expected signature is ${signature}`, async () => {
        const params = {
          mnemonic,
          chainType,
          password,
        }
        await tronSignMessage({ ...params, input: inputStr, address, signature })
      })
    }
  })
}
