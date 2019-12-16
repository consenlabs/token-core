import React from 'react'
import { StyleSheet, View, Text, TextInput, Button, Alert } from 'react-native'
import walletAPI from '../../native'
import { getChainPath } from '../../constant/path'
import Loading from '../Loading'

interface Props {
}

interface State {
  mnemonic: any
  password: string
  segWit: string
  idFromMnemonic: string | null | undefined
  addressFromMnemonic: string | null | undefined
  chainType: __chainType
  network: __networkType
  isLoading: boolean
  verifySuccess: any
  isExists: any
  accounts: any
  privateKeyDeleteSuccess: any
  mnemonicDeleteSuccess: any
  exportMnemonic: any
  privateKey: string
  exportPrivateKey: any
  addressFromPrivateKey: string
  idFromPrivateKey: string | null | undefined
}

class CMP extends React.Component<Props, State> {
  static navigationOptions = ({ navigation }: any) => {
    return {
      title: 'Mnemonic & PrivateKey',
      headerLeft: () => (
        <Button
          testID="goBack"
          onPress={() => navigation.goBack()}
          title="goBack"
          color="#333"
        />
      ),
    };
  }

  constructor(props: Props) {
    super(props)
    this.state = {
      mnemonic: '',
      password: '',
      idFromMnemonic: '',
      addressFromMnemonic: '',
      chainType: '' as __chainType,
      network: '' as __networkType,
      segWit: '',
      isLoading: false,
      verifySuccess: false,
      isExists: false,
      privateKeyDeleteSuccess: false,
      mnemonicDeleteSuccess: false,
      accounts: '',
      exportMnemonic: '',
      privateKey: '',
      exportPrivateKey: '',
      addressFromPrivateKey: '',
      idFromPrivateKey: ''
    }
  }

  render() {
    const { mnemonic, password, chainType, network, segWit, addressFromMnemonic, isLoading, privateKey, addressFromPrivateKey } = this.state
    const inputs = {
      mnemonic,
      password,
      chainType,
      network,
      segWit,
      privateKey,
    }
    return (
      <View style={styles.container}>
        <Loading animating={isLoading} />
        <View>
          {
            Object.keys(inputs).map((v) => {
              return <TextInput
                key={v}
                testID={`input-${v}`}
                // @ts-ignore
                value={inputs[v]}
                placeholder={v}
                style={styles.input}
                onChangeText={(text) => {
                  // @ts-ignore
                  this.setState({ [v]: text })
                }}
              />
            })
          }
          <Button
            testID="importMnemonic"
            title="importMnemonic"
            onPress={this.mnemonicImport}
          />
          {!!addressFromMnemonic && <Text testID="importMnemonic-address">{addressFromMnemonic}</Text>}
        </View>
        <View>
          <Button
              testID="importPrivateKey"
              title="importPrivateKey"
              onPress={this.privateKeyImport}
            />
            {!!addressFromPrivateKey && <Text testID="importPrivateKey-address">{addressFromPrivateKey}</Text>}
        </View>
        {this.renderKeystore()}
      </View>
    )
  }

  renderKeystore() {
    const { privateKeyDeleteSuccess, mnemonicDeleteSuccess } = this.state
    return (
      <View>
        <View>
          <Button
            testID="mnemonicDelete"
            title="mnemonicDelete"
            onPress={this.mnemonicDelete}
          />
          {!!mnemonicDeleteSuccess && <Text testID="mnemonicDeleteSuccess">{`mnemonicDeleteSuccess`}</Text>}
        </View>

        <View>
          <Button
            testID="privateKeyDelete"
            title="privateKeyDelete"
            onPress={this.privateKeyDelete}
          />
          {!!privateKeyDeleteSuccess && <Text testID="privateKeyDeleteSuccess">{`privateKeyDeleteSuccess`}</Text>}
        </View>

      </View>
    )
  }

  mnemonicDelete = async () => {
    const { idFromMnemonic, password } = this.state
    try {
      this.setState({ isLoading: true })
      const res = await walletAPI.keystoreCommonDelete({ id: idFromMnemonic, password })
      // @ts-ignore
      this.setState({ mnemonicDeleteSuccess: res.isSuccess, isLoading: false })
    } catch (err) {
      this.setState({ isLoading: false })
      Alert.alert('', err.message)
    }
  }

  privateKeyDelete = async () => {
    const { idFromPrivateKey, password } = this.state
    try {
      this.setState({ isLoading: true })
      const res = await walletAPI.keystoreCommonDelete({ id: idFromPrivateKey, password })
      // @ts-ignore
      this.setState({ privateKeyDeleteSuccess: res.isSuccess, isLoading: false })
    } catch (err) {
      this.setState({ isLoading: false })
      Alert.alert('', err.message)
    }
  }

  mnemonicImport = async () => {
    const { mnemonic, password, chainType, network, segWit } = this.state
    const chainPath = getChainPath(chainType, network)
    try {
      const params = {
        mnemonic: mnemonic.trim(),
        password,
        source: 'MNEMONIC' as __walletSource,
        name: 'MNEMONIC-test',
        passwordHint: '',
        overwrite: true,
      }
      this.setState({ isLoading: true })
      const res = await walletAPI.hdStoreImport(params)
      const deriveParams = {
        chainType,
        path: chainPath,
        network,
        segWit,
      }
      const accountsRes = await walletAPI.hdStoreDerive({
        id: res.id,
        password,
        derivations: [deriveParams]
      })
      // @ts-ignore
      const addressFromMnemonic = accountsRes.accounts[0].address
      this.setState({ idFromMnemonic: res.id, addressFromMnemonic, isLoading: false })
    } catch (err) {
      this.setState({ isLoading: false })
      Alert.alert('', err.message)
    }
  }

  privateKeyImport = async () => {
    const { privateKey, password, chainType, network, segWit } = this.state
    try {
      const params = {
        privateKey: privateKey.trim(),
        password,
        chainType,
        network,
        segWit,
        overwrite: true,
      }
      this.setState({ isLoading: true })
      const res = await walletAPI.privateKeyStoreImport(params)
      // @ts-ignore
      this.setState({ idFromPrivateKey: res.id, addressFromPrivateKey: res.accounts[0].address, isLoading: false })
    } catch (err) {
      this.setState({ isLoading: false })
      Alert.alert('', err.message)
    }
  }
}

const styles = StyleSheet.create({
  container: {
    margin: 10,
  },
  input: {
    height: 50,
  },
  text: {
    color: '#333',
    fontSize: 14,
  },
})

export default CMP
