import { id, toHaveText, label } from '../../../utils.js'

export default async function (params) {
  const { chainType, mnemonic, password, address, network } = params
  // go to Mnemonic screen
  await id('Mnemonic').tap()

  await id('mnemonicInput').tap()
  await id('mnemonicInput').replaceText(mnemonic)
  await id('mnemonicInput').typeText(' ')

  await id('mnemonicPassword').tap()
  await id('mnemonicPassword').replaceText(password)

  await id('mnemonicChainType').tap()
  await id('mnemonicChainType').replaceText(chainType)

  await id('mnemonicNetwork').tap()
  await id('mnemonicNetwork').replaceText(network)

  // dismiss keyboard
  await label('return').tap()

  await id('mnemonicSubmit').tap()

  await waitFor(id('mnemonicAddress')).toExist().withTimeout(2000)

  await toHaveText('mnemonicAddress', address)

  // go back
  await id('goBack').tap()
}
