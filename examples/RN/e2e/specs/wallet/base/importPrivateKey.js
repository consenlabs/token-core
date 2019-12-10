import { id, toHaveText, label } from '../../../utils.js'

export default async function (params) {
  const { chainType, privateKey, password, address, network, segWit } = params
  // go to Mnemonic screen
  await id('PrivateKey').tap()

  await id('input-privateKey').tap()
  await id('input-privateKey').replaceText(privateKey)

  await id('input-password').tap()
  await id('input-password').replaceText(password)

  await id('input-chainType').tap()
  await id('input-chainType').replaceText(chainType)

  await id('input-network').tap()
  await id('input-network').replaceText(network)

  await id('input-segWit').tap()
  await id('input-segWit').replaceText(segWit)

  // dismiss keyboard
  // await label('return').tap()
  await id('import-btn').tap()
  await waitFor(id('import-address')).toExist().withTimeout(2000)
  await toHaveText('import-address', address)

  // export
  await id('export-btn').tap()
  await waitFor(id('export-privateKey')).toExist().withTimeout(2000)
  await toHaveText('export-privateKey', privateKey)

  // go back
  await id('goBack').tap()
}
