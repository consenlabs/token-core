import { id, toHaveText, label, text } from '../../../utils.js'

export default async function (params) {
    const { chainType, mnemonic, password, network, segWit, address } = params
    // go to Mnemonic screen
    await id('Mnemonic').tap()
    await inputRightParams(chainType, mnemonic, password, network, segWit)

    // invalid mnemonic (empty)
    let invalidMnemonic = ''
    let errorMessage = 'called `Result::unwrap()` on an `Err` value: InvalidWord'
    await inputInvalidMnemonic(invalidMnemonic, errorMessage)

    // invalid mnemonic (word misspelling)
    invalidMnemonic = 'inject kidney empty canal shadow pact comfort wife crush horse wife sketchs'
    errorMessage = 'called `Result::unwrap()` on an `Err` value: InvalidWord'
    await inputInvalidMnemonic(invalidMnemonic, errorMessage)
    
    // invalid mnemonic (11 words)
    invalidMnemonic = 'inject kidney empty canal shadow pact comfort wife crush horse wife'
    errorMessage = 'called `Result::unwrap()` on an `Err` value: InvalidWordLength(11)'
    await inputInvalidMnemonic(invalidMnemonic, errorMessage)

    // invalid mnemonic 
    invalidMnemonic = 'inject kidney empty canal shadow pact comfort wife crush horse wife sketch wife'
    errorMessage = 'called `Result::unwrap()` on an `Err` value: InvalidWordLength(13)'
    await inputInvalidMnemonic(invalidMnemonic, errorMessage)

    // invalid mnemonic 
    invalidMnemonic = 'inject kidney empty canal shadow pact comfort wife crush horse wife wife'
    errorMessage = 'called `Result::unwrap()` on an `Err` value: InvalidChecksum'
    await inputInvalidMnemonic(invalidMnemonic, errorMessage)

    await inputRightParams(chainType, mnemonic, password, network, segWit)
    // invalid password
    let invalidPassword = ''
    await importInvalidPassword(invalidPassword, address)

    await inputRightParams(chainType, mnemonic, password, network, segWit)
    // invalid chain type (empty)
    let invalidChainType = ''
    errorMessage = 'unsupported_chain'
    await importInvalidChainType(invalidChainType, errorMessage)

    // invalid chain type (BITCOIN)
    invalidChainType = 'BITCOIN'
    errorMessage = 'unsupported_chain'
    await importInvalidChainType(invalidChainType, errorMessage)

    if (chainType != 'TRON') {
        await inputRightParams(chainType, mnemonic, password, network, segWit)
        // invalid network (empty)
        let invalidNetwork = ''
        errorMessage = 'unsupported_chain'
        await importInvalidNetwork(invalidNetwork, errorMessage)

        // invalid network (KOVAN)
        invalidNetwork = 'KOVAN'
        errorMessage = 'unsupported_chain'
        await importInvalidNetwork(invalidNetwork, errorMessage)

        await inputRightParams(chainType, mnemonic, password, network, segWit)
        // invalid Segwit (GENERAL)
        let invalidSegwit = 'GENERAL'
        errorMessage = 'unsupported_chain'
        await importInvalidSegwit(invalidSegwit, errorMessage)
    }

    await inputRightParams(chainType, mnemonic, password, network, segWit)
    // export mnemonic with invalid password
    invalidPassword = '123123123!@#$%^'
    errorMessage = 'password_incorrect'
    await exportInvalidPassword(address, invalidPassword, errorMessage)

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

export async function importInvalidPassword (invalidPassword, address) {
    await id('input-password').tap()
    await id('input-password').replaceText(invalidPassword)
    await label('return').tap()
    await id('import').tap()
    
    await waitFor(id('import-address')).toExist().withTimeout(2000)
    await toHaveText('import-address', address)
}

export async function importInvalidChainType (invalidChainType, errorMessage) {
    await id('input-chainType').tap()
    await id('input-chainType').replaceText(invalidChainType)
    await label('return').tap()
    await id('import').tap()
    await expect(text(errorMessage)).toExist()
    await text('OK').tap()
}

export async function importInvalidNetwork (invalidNetwork, errorMessage) {
    await id('input-network').tap()
    await id('input-network').replaceText(invalidNetwork)
    await label('return').tap()
    await id('import').tap()
    await expect(text(errorMessage)).toExist()
    await text('OK').tap()
}

export async function importInvalidSegwit (invalidSegwit, errorMessage) {
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