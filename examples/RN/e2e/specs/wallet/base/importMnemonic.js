import { id, toHaveText, label } from '../../../utils.js'

export default async function (params) {
  const { chainType, mnemonic, password, address } = params
  await id('mnemonicInput').tap()
  await id('mnemonicInput').clearText()
  await id('mnemonicInput').typeText(mnemonic)

  await id('mnemonicPassword').tap()
  await id('mnemonicPassword').clearText()
  await id('mnemonicPassword').typeText(password)

  await id('mnemonicChainType').tap()
  await id('mnemonicChainType').clearText()
  await id('mnemonicChainType').typeText(chainType)

  // dismiss keyboard
  await label('return').tap()

  await id('mnemonicSubmit').tap()

  await waitFor(id('mnemonicAddress')).toExist().withTimeout(2000)

  await toHaveText('mnemonicAddress', address)
}
