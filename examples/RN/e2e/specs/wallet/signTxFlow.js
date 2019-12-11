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
        amount: 100000,
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
    signature: "01000000015884e5db9de218238671572340b207ee85b628074e7e467096c267266baf77a4000000006a473044022029063983b2537e4aa15ee838874269a6ba6f5280297f92deb5cd56d2b2db7e8202207e1581f73024a48fce1100ed36a1a48f6783026736de39a4dd40a1ccc75f651101210223078d2942df62c45621d209fab84ea9a7a23346201b7727b9b45a29c4e76f5effffffff0220a10700000000001976a9147821c0a3768aa9d1a37e16cf76002aef5373f1a888ac801a0600000000001976a914073b7eae2823efa349e3b9155b8a735526463a0f88ac00000000",
  },
  {
    chainType: "TRON",
    mnemonic: MNEMONIC_12,
    network: '',
    segWit: '',
    password: PASSWORD,
    address: TRON_MAINNET_MNEMONIC_12_ADDRESS,
    input: {
      raw_data: '0a0202a22208e216e254e43ee10840c8cbe4e3df2d5a67080112630a2d747970652e676f6f676c65617069732e636f6d2f70726f746f636f6c2e5472616e73666572436f6e747261637412320a15415c68cc82c87446f602f019e5fd797437f5b79cc212154156a6076cd1537fa317c2606e4edfa4acd3e8e92e18a08d06709084e1e3df2d',
    },
    signature: "a9605b8766796299aeb40279c78e6818519d2330c4d20fb99583c85249920e8004b8d003ed9981016d87222c58e442de5b20b1996a322480562785f91d95211801",
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
    signature: "0100000001e2986a004630cb451921d9e7b4454a6671e50ddd43ea431c34f6011d9ca4c309000000006a473044022064fb81c11181e6604aa56b29ed65e31680fc1203f5afb6f67c5437f2d68192d9022022282d6c3c35ffdf64a427df5e134aa0edb8528efb6151cb1c3b21422fdfd6e041210251492dfb299f21e426307180b577f927696b6df0b61883215f88eb9685d3d449ffffffff020e6d0100000000001976a9142af4c2c085cd9da90c13cd64c6ae746fa139956e88ac22020000000000001976a914bedf37acf35504c9bfd18b09d989d0fb23fd269688ac00000000",
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
