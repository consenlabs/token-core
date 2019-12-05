import { id, toHaveText, label } from '../../../utils.js'

export default async function (params) {
  const { chainType, privateKey, password, address, network } = params
  // go to Mnemonic screen
  await id('PrivateKey').tap()

  await id('privateKeyInput').tap()
  await id('privateKeyInput').replaceText(privateKey)
  await id('privateKeyInput').typeText(' ')

  await id('privateKeyPassword').tap()
  await id('privateKeyPassword').replaceText(password)

  await id('privateKeyChainType').tap()
  await id('privateKeyChainType').replaceText(chainType)

  await id('privateKeyNetwork').tap()
  await id('privateKeyNetwork').replaceText(network)

  // dismiss keyboard
  await label('return').tap()

  await id('privateKeySubmit').tap()

  await waitFor(id('privateKeyAddress')).toExist().withTimeout(2000)

  await toHaveText('privateKeyAddress', address)

  // go back
  await id('goBack').tap()
}
