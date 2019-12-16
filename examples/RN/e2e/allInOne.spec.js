import mnemonicFlow from './specs/wallet/mnemonicFlow'
import privateKeyFlow from './specs/wallet/privateKeyFlow'
import createFlow from './specs/wallet/createFlow'
import signTxFlow from './specs/wallet/signTxFlow'
import tronSignMessageFlow from './specs/wallet/tronSignMessageFlow'
import mnemonicPrivateKeyFlow from './specs/wallet/mnemonicPrivateKeyFlow'

const repeatImport = true
const runRobust = true

createFlow()
mnemonicFlow(repeatImport, runRobust)
privateKeyFlow(repeatImport, runRobust)
mnemonicPrivateKeyFlow()
signTxFlow()
tronSignMessageFlow()