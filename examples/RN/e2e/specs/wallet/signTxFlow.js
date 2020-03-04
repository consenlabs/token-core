/**
 * sign tx flow test
 *
 * import -> sign
 */

import signTx from './base/signTx'

import {
  MNEMONIC_12,
  PASSWORD,
  BITCOINCASH_MAINNET_MNEMONIC_12_ADDRESS,
  TRON_MAINNET_MNEMONIC_12_ADDRESS,
  LITECOIN_TESTNET_MNEMONIC_12_ADDRESS,
  KUSAMA_MAINNET_MNEMONIC_12_ADDRESS,
  POLKADOT_MAINNET_MNEMONIC_12_ADDRESS,
} from '../../constant'
import { formatHdStoreParams } from '../../chain'

// sign tx
export const SIGN_TX = [
  {
    chainType: "LITECOIN",
    mnemonic: MNEMONIC_12,
    network: "TESTNET",
    segWit: "NONE",
    password: PASSWORD,
    address: LITECOIN_TESTNET_MNEMONIC_12_ADDRESS,
    input: {
      to: "mrU9pEmAx26HcbKVrABvgL7AwA5fjNFoDc",
      amount: 500000,
      unspents: [{
        txHash: "a477af6b2667c29670467e4e0728b685ee07b240235771862318e29ddbe58458",
        vout: 0,
        amount: 1000000,
        address: LITECOIN_TESTNET_MNEMONIC_12_ADDRESS,
        scriptPubKey: "76a91488d9931ea73d60eaf7e5671efc0552b912911f2a88ac",
        derivedPath: "0/0",
        sequence: 0
      }],
      fee: 100000,
      changeAddressIndex: 1,
      changeAddress: "",
      network: "TESTNET",
      segWit: "NONE"
    },
    signature: "01000000015884e5db9de218238671572340b207ee85b628074e7e467096c267266baf77a4000000006a47304402206dd1b8e728cbb40390d34abca37d5b47caf18305b3279486e6dcedf78a83135e02201f9aae50abc75eacdf8d79790d08466c74cffbf10c3e61380ba1c4921d6a91620121033d710ab45bb54ac99618ad23b3c1da661631aa25f23bfe9d22b41876f1d46e4effffffff0220a10700000000001976a9147821c0a3768aa9d1a37e16cf76002aef5373f1a888ac801a0600000000001976a9145ec105a23edc97d73a8de1e49d498684c40aa84988ac00000000",
  },
  {
    chainType: "BITCOINCASH",
    mnemonic: MNEMONIC_12,
    network: "MAINNET",
    segWit: "NONE",
    password: PASSWORD,
    address: BITCOINCASH_MAINNET_MNEMONIC_12_ADDRESS,
    input: {
      to: "qq40fskqshxem2gvz0xkf34ww3h6zwv4dcr7pm0z6s",
      amount: 93454,
      unspents: [{
        txHash: "09c3a49c1d01f6341c43ea43dd0de571664a45b4e7d9211945cb3046006a98e2",
        vout: 0,
        amount: 100000,
        address: BITCOINCASH_MAINNET_MNEMONIC_12_ADDRESS,
        scriptPubKey: "76a91488d9931ea73d60eaf7e5671efc0552b912911f2a88ac",
        derivedPath: "1/0",
        sequence: 0
      }],
      fee: 6000,
      changeIdx: 1,
      changeAddress: "",
      network: "MAINNET",
      segWit: "NONE"
    },
    signature: "0100000001e2986a004630cb451921d9e7b4454a6671e50ddd43ea431c34f6011d9ca4c309000000006b48304502210088ed3a37e78b75b035bb659b8193a6d4f107d910912b46fda970dea14180fdc602207055129e7b128c97dc8a42cc0f84ba84dc79450969f2ae5e90e9027c5d1f9ac14121032883f8e6ef294bd499274ae62c2e3353d48e170f4e7efa88059be64302a99463ffffffff020e6d0100000000001976a9142af4c2c085cd9da90c13cd64c6ae746fa139956e88ac22020000000000001976a91408bec760167a6c9ee0c3608092b01b9b54bd387a88ac00000000",
  },
  {
    chainType: "TRON",
    mnemonic: MNEMONIC_12,
    network: '',
    segWit: '',
    password: PASSWORD,
    address: TRON_MAINNET_MNEMONIC_12_ADDRESS,
    input: {
      rawData: '0a0202a22208e216e254e43ee10840c8cbe4e3df2d5a67080112630a2d747970652e676f6f676c65617069732e636f6d2f70726f746f636f6c2e5472616e73666572436f6e747261637412320a15415c68cc82c87446f602f019e5fd797437f5b79cc212154156a6076cd1537fa317c2606e4edfa4acd3e8e92e18a08d06709084e1e3df2d',
    },
    signature: 'bbf5ce0549490613a26c3ac4fc8574e748eabda05662b2e49cea818216b9da18691e78cd6379000e9c8a35c13dfbf620f269be90a078b58799b56dc20da3bdf200',
  },
  {
    chainType: "KUSAMA",
    mnemonic: MNEMONIC_12,
    network: '',
    segWit: '',
    password: PASSWORD,
    address: KUSAMA_MAINNET_MNEMONIC_12_ADDRESS,
    input: {
      rawData: '0x496943de3c8b1458635ee30c15bb26cede6957abdd28f0a860cdaf967116c747',
    },
    signature: '0x01',
  },
  {
    chainType: "POLKADOT",
    mnemonic: MNEMONIC_12,
    network: '',
    segWit: '',
    password: PASSWORD,
    address: POLKADOT_MAINNET_MNEMONIC_12_ADDRESS,
    input: {
      rawData: '0x496943de3c8b1458635ee30c15bb26cede6957abdd28f0a860cdaf967116c747',
    },
    signature: '0x01',
  }

]

export default function () {
  describe('⏳ signTx flow', () => {
    for (const txIndex in SIGN_TX) {
      const tx = SIGN_TX[txIndex]
      const { mnemonic, chainType, password, network, segWit, input, address, signature } = tx
      const inputStr = JSON.stringify(input)

      it(`should sign ${inputStr} by ${chainType} wallet, and the expected signature is ${signature}`, async () => {
        const params = formatHdStoreParams({
          mnemonic,
          chainType,
          password,
          network,
          segWit,
        })
        await signTx({ ...params, input: inputStr, address, signature })
      })
    }
  })
}
