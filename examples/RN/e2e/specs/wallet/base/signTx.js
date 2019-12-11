import { id, toHaveText } from '../../../utils.js'

export default async function (params) {
  const { chainType, mnemonic, password, address, network, segWit, input, signature } = params
  await id('SignTx').tap()

  // 1. import wallet from mnemonic
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

  await id('import').tap()
  await waitFor(id('import-address')).toExist().withTimeout(2000)
  await toHaveText('import-address', address)

  // 2. sign tx
  await id('input-input').tap()
  await id('input-input').replaceText(input)
  await id('signTx').tap()

  // 3. verify signature
  await waitFor(id('signature')).toExist().withTimeout(2000)
  await toHaveText('signature', signature)

  // go back
  await id('goBack').tap()
}
