import { id, text } from '../../../utils.js'

export default async function (params) {
  const { chainType, mnemonic, password, address } = params
  await id('mnemonicInput').tap()
  await id('mnemonicInput').typeText(mnemonic)

  await id('mnemonicPassword').tap()
  await id('mnemonicPassword').typeText(password)

  await id('mnemonicChainType').tap()
  await id('mnemonicChainType').typeText(chainType)

  await id('mnemonicSubmit').tap()

  await expect(text(address)).toExist()
}
