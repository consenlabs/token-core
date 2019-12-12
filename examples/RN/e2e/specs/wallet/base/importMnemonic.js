import { id, toHaveText, label } from '../../../utils.js'

export default async function (params) {
  const { chainType, mnemonic, password, address, network, segWit } = params
  // go to Mnemonic screen
  await id('Mnemonic').tap()

  await id('input-mnemonic').tap()
  await id('input-mnemonic').replaceText(mnemonic)

  await id('input-password').tap()
  await id('input-password').replaceText(password)

  await id('input-chainType').tap()
  await id('input-chainType').replaceText(chainType)

  await id('input-network').tap()
  await id('input-network').replaceText(network)

  await id('input-segWit').tap()
  await id('input-segWit').replaceText(segWit)

  // dismiss keyboard
  await label('return').tap()

  await id('import').tap()
  await waitFor(id('import-address')).toExist().withTimeout(2000)
  await toHaveText('import-address', address)

  // export
  await id('export').tap()
  await waitFor(id('expected-mnemonic')).toExist().withTimeout(2000)
  await toHaveText('expected-mnemonic', mnemonic)

  // keystore
  await id('keystoreCommonVerify').tap()
  await waitFor(id('verifySuccess')).toExist().withTimeout(2000)

  await id('keystoreCommonExists').tap()
  await waitFor(id('isExists')).toExist().withTimeout(2000)

  await id('keystoreCommonAccounts').tap()
  await waitFor(id('accounts')).toExist().withTimeout(2000)

  // await id('keystoreCommonDelete').tap()
  // await waitFor(id('deleteSuccess')).toExist().withTimeout(2000)

  // go back
  await id('goBack').tap()
}
