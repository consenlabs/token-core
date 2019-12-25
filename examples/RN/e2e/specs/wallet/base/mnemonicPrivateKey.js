import { id, toHaveText, label, text } from '../../../utils.js'

export default async function (params) {
  const { chainType, privateKey, password, address, network, segWit, mnemonic, coverFlow } = params
    // go to MnemonicPrivateKey screen
    await id('MnemonicPrivateKey').tap()
    if (coverFlow === 'mnemonicCoverPrivateKey') {
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
      
      await id('importPrivateKey').tap()
      await waitFor(id('importPrivateKey-address')).toExist().withTimeout(2000)
      await toHaveText('importPrivateKey-address', address)
    
      // cover import mnemonic
      await id('input-mnemonic').tap()
      await id('input-mnemonic').replaceText(mnemonic)
    
      // dismiss keyboard
      await label('return').tap()

      await id('importMnemonic').tap()
      await waitFor(id('importMnemonic-address')).toExist().withTimeout(2000)
      await toHaveText('importMnemonic-address', address)
    
      await id('mnemonicDelete').tap()
      await waitFor(id('mnemonicDeleteSuccess')).toExist().withTimeout(2000)

      await id('privateKeyDelete').tap()
      await waitFor(id('privateKeyDeleteSuccess')).toExist().withTimeout(2000)
    
      // go back
      await id('goBack').tap()

    } else {

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

      await id('importMnemonic').tap()
      await waitFor(id('importMnemonic-address')).toExist().withTimeout(2000)
      await toHaveText('importMnemonic-address', address)

      // cover import privateKey
      await id('input-privateKey').tap()
      await id('input-privateKey').replaceText(privateKey)

      // dismiss keyboard
      await label('return').tap()

      await id('importPrivateKey').tap()
      await waitFor(id('importPrivateKey-address')).toExist().withTimeout(2000)
      await toHaveText('importPrivateKey-address', address)

      await id('privateKeyDelete').tap()
      await waitFor(id('privateKeyDeleteSuccess')).toExist().withTimeout(2000)
      
      await id('mnemonicDelete').tap()
      await waitFor(id('mnemonicDeleteSuccess')).toExist().withTimeout(2000)

      // go back
      await id('goBack').tap()
    }

}