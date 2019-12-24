export const formatHdStoreParams = ({ mnemonic, password, chainType, network, segWit }) => {
  switch (chainType) {
    case 'TRON':
      return {
        chainType,
        mnemonic,
        password,
        network: '',
        segWit: '',
      }
    case 'NERVOS':
      return {
        chainType,
        mnemonic,
        password,
        network,
        segWit: '',
      }
    default:
      return {
        chainType,
        mnemonic,
        password,
        network,
        segWit,
      }
  }
}

export const formatPrivateKeyStoreParams = ({ privateKey, password, chainType, network, segWit }) => {
  switch (chainType) {
    case 'TRON':
      return {
        chainType,
        privateKey,
        password,
        network: '',
        segWit: '',
      }
    case 'TRON':
      return {
        chainType,
        privateKey,
        password,
        network,
        segWit: '',
      }
    default:
      return {
        chainType,
        privateKey,
        password,
        network,
        segWit,
      }
  }
}
