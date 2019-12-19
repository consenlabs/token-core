import { id, toHaveText, label, text, getTextValue } from '../../../utils.js'

export default async function (params) {
    const { chainType, privateKey, password, address, network, segWit, repeatImport, runRobust, REPEAT_PASSWORD } = params
    // go to Mnemonic screen
    await id('PrivateKey').tap()
    
    await inputRightParams (chainType, privateKey, password, network, segWit)
    // invalid privateKey (empty)
    let invalidPrivateKey = ''
    let errorMessage = ''
    
    errorMessage = (chainType === 'TRON') ? 'invalid_private_key' : 'base58ck data not even long enough for a checksum'
    await inputInvalidPrivateKey(invalidPrivateKey, errorMessage)

    // invalid privateKey (error)
    invalidPrivateKey = privateKey.substr(0, privateKey.length-7) + 'imToken'
    errorMessage = (chainType === 'TRON') ? "hex can't decode: InvalidHexCharacter { c: 'i', index: 57 }" : 'base58ck checksum'
    await inputInvalidPrivateKey(invalidPrivateKey, errorMessage)

    // invalid privateKey length-1
    invalidPrivateKey = privateKey.substr(0, privateKey.length-1)
    errorMessage = (chainType === 'TRON') ? "hex can't decode: OddLength" : 'base58ck checksum'
    await inputInvalidPrivateKey(invalidPrivateKey, errorMessage)
    
    // invalid privateKey length+1
    invalidPrivateKey = privateKey + 'i'
    errorMessage = (chainType === 'TRON') ? "hex can't decode: OddLength" : 'base58ck checksum'
    await inputInvalidPrivateKey(invalidPrivateKey, errorMessage)
  
    await inputRightParams (chainType, privateKey, password, network, segWit)
    // invalid password
    let invalidPassword = ''
    await inputInvalidPassword(invalidPassword, address)
    await deleteWallet('')

    await inputRightParams (chainType, privateKey, password, network, segWit)
    // invalid ChainType 
    let invalidChainType = ''
    errorMessage = (chainType === 'TRON') ? 'unsupported_chain' : 'invalid_private_key'
    invalidChainType = (chainType === 'BITCOINCASH') ? 'LITECOIN' : 'BITCOINCASH'
    await inputInvalidChainType (invalidChainType, errorMessage)

    if (chainType != 'TRON') {
        await inputRightParams (chainType, privateKey, password, network, segWit)
        // invalid network (TESTNET)
        let invalidNetwork = 'TESTNET'
        errorMessage = 'invalid_private_key'
        await inputInvalidNetwork(invalidNetwork, errorMessage)

        // invalid network (empty)
        invalidNetwork = ''
        errorMessage = 'unsupported_chain'
        await inputInvalidNetwork(invalidNetwork, errorMessage)

        // invalid Segwit (GENERAL)
        let invalidSegwit = 'GENERAL'
        errorMessage = 'unsupported_chain'
        await inputInvalidSegwit (invalidSegwit, errorMessage)
    }

    await inputRightParams (chainType, privateKey, password, network, segWit)
    // export private key with invalid password
    invalidPassword = '123123123!@#$%^'
    errorMessage = 'password_incorrect'
    await exportInvalidPassword (address, invalidPassword, errorMessage)

    await deleteWallet(password)

    // go back
    await id('goBack').tap()
    
}


export async function inputRightParams (chainType, privateKey, password, network, segWit) {
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
}

export async function inputInvalidPrivateKey (invalidPrivateKey, errorMessage) { 
    await id('input-privateKey').tap()
    await id('input-privateKey').replaceText(invalidPrivateKey)
    await label('return').tap()
    await id('import').tap()

    try {
        await expect(text(errorMessage)).toExist()
        await text('OK').tap()
    } catch (error) {
        let fullErrorMessage = await getTextValue(errorMessage)
        if (fullErrorMessage.indexOf('does not match expected') > -1) {
            await text('OK').tap()
        }
    }
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