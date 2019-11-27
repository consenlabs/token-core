/**
 * Wallet Native API
 */

import { NativeModules } from 'react-native'
import {
  // import wallet
  ImportWalletFromMnemonicParams,
  ImportWalletFromKeystoreParams,
  ImportWalletFromPrivateKeyParams,
  ImportWalletResult,

  // export wallet
  ExportParams,
  ExportBtcPrivateKeyParams,
  ExportKeystoreResult,
  ExportMnemonicResult,
  ExportPrivateKeyResult,
  ExistsMnemonicParams,
  ExistsKeystoreParams,
  ExistsPrivateKeyParams,
  RemoveWalletParams,
  RemoveWalletResult,

  // sign transaction
  EthtereumSignParams,
  BitcoinSignParams,
  EosSignParams,
  SignTxResult,

  // personal sign
  EosEcSignParams,
  SignParams,

  // password
  VerifyPasswordParams,
  VerifyPasswordResult,
} from './interface'

const WalletAPI = NativeModules.WalletAPI

export default {
  generateMnemonic(): Promise<string> {
    return WalletAPI.generateMnemonic({})
  },

  importWalletFromMnemonic(params: ImportWalletFromMnemonicParams): Promise<ImportWalletResult> {
    return WalletAPI.importWalletFromMnemonic(params)
  },

  importWalletFromKeystore(params: ImportWalletFromKeystoreParams): Promise<ImportWalletResult> {
    return WalletAPI.importWalletFromKeystore(params)
  },

  importWalletFromPrivateKey(params: ImportWalletFromPrivateKeyParams): Promise<ImportWalletResult> {
    return WalletAPI.importWalletFromPrivateKey(params)
  },

  existsMnemonic(params: ExistsMnemonicParams): Promise<ImportWalletResult> {
    return WalletAPI.findWalletByMnemonic(params)
  },

  existsKeystore(params: ExistsKeystoreParams): Promise<ImportWalletResult> {
    return WalletAPI.findWalletByKeystore(params)
  },

  existsPrivateKey(params: ExistsPrivateKeyParams): Promise<ImportWalletResult> {
    return WalletAPI.findWalletByPrivateKey(params)
  },

  exportPrivateKey(params: ExportParams | ExportBtcPrivateKeyParams): Promise<ExportPrivateKeyResult> {
    return WalletAPI.exportPrivateKey(params)
  },

  exportMnemonic(params: ExportParams): Promise<ExportMnemonicResult> {
    return WalletAPI.exportMnemonic(params)
  },

  exportKeystore(params: ExportParams): Promise<ExportKeystoreResult> {
    return WalletAPI.exportKeystore(params)
  },

  removeWallet(params: RemoveWalletParams): Promise<RemoveWalletResult> {
    return WalletAPI.removeWallet(params)
  },

  signTransaction(params: EthtereumSignParams | BitcoinSignParams | EosSignParams): Promise<SignTxResult> {
    return WalletAPI.signTransaction(params)
  },

  eosEcSign(params: EosEcSignParams): Promise<string /* signature */> {
    return WalletAPI.eosEcSign(params)
  },

  personalSign(params: SignParams): Promise<string /* signature */> {
    return WalletAPI.personalSign(params)
  },

  verifyPassword(params: VerifyPasswordParams): Promise<VerifyPasswordResult> {
    return WalletAPI.verifyPassword(params)
  },
}
