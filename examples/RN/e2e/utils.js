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


export async function getTextValue(testText) {
  try {
    await expect(text(testText)).toExist()
  } catch (e) {
    const start = `AX.label='${testText}`
    const end = "; AX.frame"
    const errorMessage = e.message.toString()
    const [, restMessage] = errorMessage.split(start)
    const [textValue] = restMessage.split(end)
    return textValue
  }
}

export async function toHaveText(testID, expectText) {
  await expect(id(testID)).toHaveText(expectText)
}

export const asyncForEach = async (array, callback) => {
  for (let index = 0; index < array.length; index++) {
    await callback(array[index], index, array)
  }
}
