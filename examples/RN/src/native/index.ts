/**
 * Wallet Native API
 */

// import protobuf from 'protobufjs'
import { NativeModules } from 'react-native'
import { Buffer } from 'buffer'
import protoRoot from './proto'
import { api, transaction } from './proto.d'

const TcxAPI = NativeModules.TcxApi

const getParamsAndResponseType = (method: any, params: any) => {
  const { chainType } = params
  switch (method) {
    case 'hd_store_create':
      return [
        protoRoot.api.HdStoreCreateParam,
        protoRoot.api.WalletResult,
      ]
    case 'hd_store_derive':
      return [
        protoRoot.api.HdStoreDeriveParam,
        protoRoot.api.AccountsResponse,
      ]
    case 'hd_store_import':
      return [
        protoRoot.api.HdStoreImportParam,
        protoRoot.api.WalletResult,
      ]
    case 'hd_store_export':
      return [
        protoRoot.api.WalletKeyParam,
        protoRoot.api.KeystoreCommonExportResult,
      ]
    case 'private_key_store_import':
      return [
        protoRoot.api.PrivateKeyStoreImportParam,
        protoRoot.api.WalletResult,
      ]
    case 'private_key_store_export':
      return [
        protoRoot.api.PrivateKeyStoreExportParam,
        protoRoot.api.KeystoreCommonExportResult,
      ]
    case 'keystore_common_exists':
      return [
        protoRoot.api.KeystoreCommonExistsParam,
        protoRoot.api.KeystoreCommonExistsResult,
      ]
    case 'keystore_common_verify':
      return [
        protoRoot.api.WalletKeyParam,
        protoRoot.api.Response,
      ]
    case 'keystore_common_delete':
      return [
        protoRoot.api.WalletKeyParam,
        protoRoot.api.Response,
      ]
    case 'keystore_common_accounts':
      return [
        protoRoot.api.KeystoreCommonAccountsParam,
        protoRoot.api.AccountsResponse,
      ]
    case 'sign_tx': {
      let ResponseType: any = protoRoot.transaction.BtcForkSignedTxOutput
      switch (chainType) {
        case 'TRON':
          ResponseType = protoRoot.transaction.TronTxOutput
          break
        default:
          break
      }
      return [
        protoRoot.api.SignParam,
        ResponseType,
      ]
    }
    case 'tron_sign_message':
      return [
        protoRoot.api.SignParam,
        protoRoot.transaction.TronMessageOutput,
      ]
    default:
      return []
  }
}

const tcxApi = async(method: any, params: any) => {
  try {
    const Any = protoRoot.google.protobuf.Any
    const TcxAction = protoRoot.api.TcxAction

    const [ParamType, ResType] = getParamsAndResponseType(method, params)

    const encodedParam = ParamType.create(params)
    const any = Any.create({
      type_url: method,
      value: ParamType.encode(encodedParam).finish(),
    })
    const message = TcxAction.create({
      method: method,
      param: any
    })

    console.log(`-------------- ${method} -------------------`)
    console.log(`${method} params`, JSON.stringify(params))
    const buffer = TcxAction.encode(message).finish()
    const hexStr = Buffer.from(buffer).toString('hex')
    console.log(`${method} hexStr: `, hexStr)
    const res = await TcxAPI.callTcxApi(hexStr)
    console.log(`${method} res: `, res)
    const retBuf = Buffer.from(res, 'hex')
    const result = ResType.decode(retBuf)
    console.log(`${method} decode res: `, JSON.stringify(result))
    return result
  } catch (error) {
    console.warn(`${method} error`, error)
    const errBuf = Buffer.from(error, 'hex')
    console.warn(`${method} errBuf`, errBuf)
    const ResponseType = protoRoot.api.Response
    const err = ResponseType.decode(errBuf)
    console.warn(`${method} decode err`, err)
    throw err
  }
}

export default {
  async hdStoreCreate(params: api.IHdStoreCreateParam): Promise<api.IWalletResult> {
    // @ts-ignore
    return tcxApi('hd_store_create', params)
  },

  async hdStoreImport(params: api.IHdStoreImportParam): Promise<api.IWalletResult> {
    // @ts-ignore
    return tcxApi('hd_store_import', params)
  },

  async hdStoreDerive(params: api.IHdStoreDeriveParam): Promise<api.IAccountsResponse> {
    // @ts-ignore
    return tcxApi('hd_store_derive', params)
  },

  async hdStoreExport(params: api.IWalletKeyParam): Promise<api.IKeystoreCommonExportResult> {
    // @ts-ignore
    return tcxApi('hd_store_export', params)
  },

  async privateKeyStoreImport(params: api.IPrivateKeyStoreImportParam): Promise<api.IWalletResult> {
    // @ts-ignore
    return tcxApi('private_key_store_import', params)
  },

  async privateKeyStoreExport(params: api.IPrivateKeyStoreExportParam): Promise<api.IKeystoreCommonExportResult> {
    // @ts-ignore
    return tcxApi('private_key_store_export', params)
  },

  async keystoreCommonVerify(params: api.IWalletKeyParam): Promise<api.IResponse> {
    // @ts-ignore
    return tcxApi('keystore_common_verify', params)
  },

  async keystoreCommonDelete(params: api.IWalletKeyParam): Promise<api.IResponse> {
    // @ts-ignore
    return tcxApi('keystore_common_delete', params)
  },

  async keystoreCommonExists(params: api.IKeystoreCommonExistsParam): Promise<api.IKeystoreCommonExistsResult> {
    // @ts-ignore
    return tcxApi('keystore_common_exists', params)
  },

  async keystoreCommonAccounts(params: api.IKeystoreCommonAccountsParam): Promise<api.IAccountsResponse> {
    // @ts-ignore
    return tcxApi('keystore_common_accounts', params)
  },

  async signTx(params: api.ISignParam): Promise<transaction.IBtcForkSignedTxOutput | transaction.ITronTxOutput> {
    // @ts-ignore
    return tcxApi('sign_tx', params)
  },

  async tronSignMessage(params: api.ISignParam): Promise<transaction.ITronMessageOutput> {
    // @ts-ignore
    return tcxApi('tron_sign_message', params)
  },
}
