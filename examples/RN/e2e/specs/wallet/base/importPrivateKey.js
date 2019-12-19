import { id, toHaveText, label, text } from '../../../utils.js'

export default async function (params) {
  const { chainType, privateKey, password, address, network, segWit, repeatImport, runRobust, REPEAT_PASSWORD } = params
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
  await label('return').tap()
  
  await id('import').tap()
  await waitFor(id('import-address')).toExist().withTimeout(2000)
  await toHaveText('import-address', address)

  // repeat import
  if (repeatImport) {
    await id('clearOutput').tap()

    await id('input-password').tap()
    await id('input-password').replaceText(REPEAT_PASSWORD)

    await id('import').tap()
    await waitFor(id('import-address')).toExist().withTimeout(2000)
    await toHaveText('import-address', address)

    // dismiss keyboard
    await label('return').tap()
  }

  // export
  await id('export').tap()
  await waitFor(id('export-privateKey')).toExist().withTimeout(2000)
  await toHaveText('export-privateKey', privateKey)

  // keystore
  await id('keystoreCommonVerify').tap()
  await waitFor(id('verifySuccess')).toExist().withTimeout(2000)

  // private key verify failed, need xyz fix
  /*
  await id('keystoreCommonExists').tap()
  await waitFor(id('isExists')).toExist().withTimeout(2000)

  await id('keystoreCommonAccounts').tap()
  await waitFor(id('accounts')).toExist().withTimeout(2000)
  */
  await id('keystoreCommonDelete').tap()
  await waitFor(id('deleteSuccess')).toExist().withTimeout(2000)

  await id('clearOutput').tap()

  // wallet not found
  await id('export').tap()
  await expect(text('wallet_not_found')).toExist()
  await text('OK').tap()
  // only e2e:debug 
  // await text('Dismiss All').tap() 

  await id('keystoreCommonVerify').tap()
  await expect(text('wallet_not_found')).toExist()
  await text('OK').tap()
    // only e2e:debug 
  // await text('Dismiss All').tap()

  /*
  await id('keystoreCommonExists').tap()
  await waitFor(id('isExists')).toNotExist().withTimeout(2000)

  await id('keystoreCommonAccounts').tap()
  await expect(text('wallet_not_found')).toExist()
  await text('OK').tap()
    // only e2e:debug 
  // await text('Dismiss All').tap()
*/
  await id('keystoreCommonDelete').tap()
  await expect(text('wallet_not_found')).toExist()
  await text('OK').tap()
    // only e2e:debug 
  // await text('Dismiss All').tap()

  // go back
  await id('goBack').tap()
}