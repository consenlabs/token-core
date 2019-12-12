import { id, toHaveText, readTextValue } from '../../../utils.js'

export default async function (params) {
  const { chainType, password, address, network, segWit } = params
  await id('Create').tap()

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

  await id('create').tap()

  await waitFor(id('expected-address')).toExist().withTimeout(2000)
  const expectedAddress = await readTextValue('expected-address')

  // export
  await id('export').tap()
  await waitFor(id('expected-mnemonic')).toExist().withTimeout(2000)

  // import
  await id('import').tap()
  // verify
  await waitFor(id('import-address')).toExist().withTimeout(2000)
  await toHaveText('import-address', expectedAddress)

  // go back
  await id('goBack').tap()
}
