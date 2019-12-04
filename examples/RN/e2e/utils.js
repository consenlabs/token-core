export function id(str) {
  return element(by.id(str))
}

export function label(str) {
  return element(by.label(str))
}

export function text(str) {
  return element(by.text(str))
}

export function type(str) {
  return element(by.type(str))
}

export async function readTextValue(testID) {
  try {
    await expect(id(testID)).toNotExist()
  } catch (e) {
    const start = `AX.id='${testID}';`
    const end = "; AX.frame"
    const errorMessage = e.message.toString()
    const [, restMessage] = errorMessage.split(start)
    const [label] = restMessage.split(end)
    const [, value] = label.split("=")
    const textValue = value.slice(1, value.length - 1)
    return textValue
  }
}

export async function readAddressValue(walletName) {
  try {
    await expect(text(walletName)).toNotExist()
  } catch (e) {
    const start = "AX.id='"
    const end = `'; AX.label='${walletName} `
    const errorMessage = e.message.toString()
    const [, restMessage] = errorMessage.split(start)
    const [address] = restMessage.split(end)
    return address
  }
}

export async function readMnemonicValue() {
  let mnemonic = ''
  for (let index = 0; index < 12; index++) {
    mnemonic += await readTextValue(`mnemonic-${index}`)
    if (index < 11) {
      mnemonic += ' '
    }
  }
  return mnemonic
}

export async function toHaveText(testID, expectText) {
  await expect(id(testID)).toHaveText(expectText)
}
