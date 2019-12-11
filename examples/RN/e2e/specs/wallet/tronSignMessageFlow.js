
import tronSignMessage from './base/tronSignMessage'

import {
  PASSWORD,
  MNEMONIC_12,
  TRON_TESTNET_MNEMONIC_12_ADDRESS,
} from '../../constant'

const SIGN_MESSAGE = [
  {
    mnemonic: MNEMONIC_12,
    signature: "7209610445e867cf2a36ea301bb5d1fbc3da597fd2ce4bb7fa64796fbf0620a4175e9f841cbf60d12c26737797217c0082fdb3caa8e44079e04ec3f93e86bbea1c",
    password: PASSWORD,
    chainType: "TRON",
    address: TRON_TESTNET_MNEMONIC_12_ADDRESS,
    input: {
      value: '0x645c0b7b58158babbfa6c6cd5a48aa7340a8749176b120e8516216787a13dc76',
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
