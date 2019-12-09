/**
 * Wallet Native API
 */

// import protobuf from 'protobufjs'
import { NativeModules } from 'react-native'
import { Buffer } from 'buffer'
import protoRoot from './proto'
import { api } from './proto.d'

const TcxAPI = NativeModules.TcxApi

export default {
  async importWalletFromMnemonic(params: api.IHdStoreImportParam): Promise<api.IWalletResult> {
    try {
      const Any = protoRoot.google.protobuf.Any
      const TcxAction = protoRoot.api.TcxAction
      const ParamType = protoRoot.api.HdStoreImportParam
      const encodedParam = ParamType.create(params)
      const any = Any.create({
        type_url: 'hd_store_import',
        value: ParamType.encode(encodedParam).finish(),
      })
      const message = TcxAction.create({
        method: 'hd_store_import',
        param: any
      })

      const buffer = TcxAction.encode(message).finish()
      const hexStr = Buffer.from(buffer).toString('hex')
      console.log('hexStr', hexStr)
      const res = await TcxAPI.callTcxApi(hexStr)
      console.log('res', res)
      const retBuf = Buffer.from(res, 'hex')
      const WalletType = protoRoot.api.WalletResult
      const wallet = WalletType.decode(retBuf)
      console.log('wallet: ', wallet)
      return wallet
    } catch (error) {
      console.log('error', error)
      const errBuf = Buffer.from(error, 'hex')
      console.log('errBuf', errBuf)
      const ResponseType = protoRoot.api.Response
      const err = ResponseType.decode(errBuf)
      console.log('err', err)
      throw err
    }
  },
}
