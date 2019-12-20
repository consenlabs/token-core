import { id, toHaveText, label, text } from '../../../utils.js'

export default async function (params) {
    const { chainType, mnemonic, password, network, segWit, address } = params
    // go to Mnemonic screen
    await id('Mnemonic').tap()
    await inputRightParams(chainType, mnemonic, password, network, segWit)

    // invalid mnemonic (empty)
    let invalidMnemonic = ''
    let errorMessage = 'invalid word in phrase'
    await inputInvalidMnemonic(invalidMnemonic, errorMessage)

    // invalid mnemonic (word misspelling)
    invalidMnemonic = 'inject kidney empty canal shadow pact comfort wife crush horse wife sketchs'
    errorMessage = 'invalid word in phrase'
    await inputInvalidMnemonic(invalidMnemonic, errorMessage)
    
    // invalid mnemonic (11 words)
    invalidMnemonic = 'inject kidney empty canal shadow pact comfort wife crush horse wife'
    errorMessage = 'invalid number of words in phrase: 11'
    await inputInvalidMnemonic(invalidMnemonic, errorMessage)

    // invalid mnemonic 
    invalidMnemonic = 'inject kidney empty canal shadow pact comfort wife crush horse wife sketch wife'
    errorMessage = 'invalid number of words in phrase: 13'
    await inputInvalidMnemonic(invalidMnemonic, errorMessage)

    // invalid mnemonic 
    invalidMnemonic = 'inject kidney empty canal shadow pact comfort wife crush horse wife wife'
    errorMessage = 'invalid checksum'
    await inputInvalidMnemonic(invalidMnemonic, errorMessage)

    await inputRightParams(chainType, mnemonic, password, network, segWit)
    // invalid password
    let invalidPassword = ''
    await inputInvalidPassword(invalidPassword, address)
    await deleteWallet('')

    await inputRightParams(chainType, mnemonic, password, network, segWit)
    // invalid chain type (empty)
    let invalidChainType = ''
    errorMessage = 'unsupported_chain'
    await inputInvalidChainType(invalidChainType, errorMessage)

    // invalid chain type (BITCOIN)
    invalidChainType = 'BITCOIN'
    errorMessage = 'unsupported_chain'
    await inputInvalidChainType(invalidChainType, errorMessage)

    if (chainType != 'TRON') {
        await inputRightParams(chainType, mnemonic, password, network, segWit)
        // invalid network (empty)
        let invalidNetwork = ''
        errorMessage = 'unsupported_chain'
        await inputInvalidNetwork(invalidNetwork, errorMessage)

        // invalid network (KOVAN)
        invalidNetwork = 'KOVAN'
        errorMessage = 'unsupported_chain'
        await inputInvalidNetwork(invalidNetwork, errorMessage)
        if (chainType != 'NERVOS') {
            await inputRightParams(chainType, mnemonic, password, network, segWit)
            // invalid Segwit (GENERAL)
            let invalidSegwit = 'GENERAL'
            errorMessage = 'unsupported_chain'
            await inputInvalidSegwit(invalidSegwit, errorMessage)
        }
    }

    await inputRightParams(chainType, mnemonic, password, network, segWit)
    // export mnemonic with invalid password
    invalidPassword = '123123123!@#$%^'
    errorMessage = 'password_incorrect'
    await exportInvalidPassword(address, invalidPassword, errorMessage)
    await deleteWallet(password)
    // go back
    await id('goBack').tap()
}


export async function inputRightParams (chainType, mnemonic, password, network, segWit) {
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
}

export async function inputInvalidMnemonic (invalidMnemonic, errorMessage) { 
    await id('input-mnemonic').tap()
    await id('input-mnemonic').replaceText(invalidMnemonic)
    await label('return').tap()
    await id('import').tap()
    await expect(text(errorMessage)).toExist()
    await text('OK').tap()
}

export async function inputInvalidPassword (invalidPassword, address) {
    await id('input-password').tap()
    await id('input-password').replaceText(invalidPassword)
    await label('return').tap()
    await id('import').tap()
    
    await waitFor(id('import-address')).toExist().withTimeout(2000)
    await toHaveText('import-address', address)
}

export async function inputInvalidChainType (invalidChainType, errorMessage) {
    await id('input-chainType').tap()
    await id('input-chainType').replaceText(invalidChainType)
    await label('return').tap()
    await id('import').tap()
    await expect(text(errorMessage)).toExist()
    await text('OK').tap()
}

export async function inputInvalidNetwork (invalidNetwork, errorMessage) {
    await id('input-network').tap()
    await id('input-network').replaceText(invalidNetwork)
    await label('return').tap()
    await id('import').tap()
    await expect(text(errorMessage)).toExist()
    await text('OK').tap()
}

export async function inputInvalidSegwit (invalidSegwit, errorMessage) {
    await id('input-segWit').tap()
    await id('input-segWit').replaceText(invalidSegwit)
    await label('return').tap()
    await id('import').tap()
    await expect(text(errorMessage)).toExist()
    await text('OK').tap()
}

export async function exportInvalidPassword (address, invalidPassword, errorMessage) {
    await label('return').tap()
    await id('clearOutput').tap()
    await id('import').tap()
    await waitFor(id('import-address')).toExist().withTimeout(2000)
    await toHaveText('import-address', address)

    await id('input-password').tap()
    await id('input-password').replaceText(invalidPassword)
    await label('return').tap()
    await id('export').tap()
    await expect(text(errorMessage)).toExist()
    await text('OK').tap()
}

export async function deleteWallet (password) {
    await id('input-password').tap()
    await id('input-password').replaceText(password)
    await label('return').tap()

    await id('keystoreCommonDelete').tap()
    await waitFor(id('deleteSuccess')).toExist().withTimeout(2000)
    await id('clearOutput').tap()
}