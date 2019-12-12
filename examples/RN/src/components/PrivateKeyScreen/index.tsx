import React from 'react'
import { StyleSheet, View, Text, TextInput, Button, Alert } from 'react-native'
import walletAPI from '../../native'
import Loading from '../Loading'

interface Props {
}

interface State {
  chainType: __chainType
  network: __networkType
  password: string
  privateKey: string
  exportPrivateKey: any
  address: string
  segWit: string
  id: string
  isLoading: boolean
  verifySuccess: any
  isExists: any
  accounts: any
  deleteSuccess: any
  exportMnemonic: any
}

class CPK extends React.Component<Props, State> {
  static navigationOptions = ({ navigation }: any) => {
    return {
      title: 'PrivateKey',
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
      chainType: '' as __chainType,
      network: '' as __networkType,
      password: '',
      privateKey: '',
      exportPrivateKey: '',
      address: '',
      segWit: '',
      id: '',
      isLoading: false,
      verifySuccess: false,
      isExists: false,
      deleteSuccess: false,
      accounts: '',
      exportMnemonic: '',
    }
  }

  render() {
    const { privateKey, password, chainType, network, address, segWit, isLoading, exportPrivateKey } = this.state
    const inputs = {
      privateKey,
      password,
      chainType,
      network,
      segWit,
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
            testID="import"
            title="import"
            onPress={this.handleImport}
          />
          {!!address && <Text testID="import-address">{address}</Text>}
        </View>

        <View>
          <Button
            testID="export-btn"
            title="export"
            onPress={this.handleExport}
          />
          {!!exportPrivateKey && <Text testID="export-privateKey">{exportPrivateKey}</Text>}
        </View>
        {this.renderKeystore()}
      </View>
    )
  }

  renderKeystore() {
    const { verifySuccess, isExists, accounts, deleteSuccess } = this.state
    return (
      <View>
        <View>
          <Button
            testID="keystoreCommonVerify"
            title="keystoreCommonVerify"
            onPress={this.keystoreCommonVerify}
          />
          {!!verifySuccess && <Text testID="verifySuccess">{`verifySuccess`}</Text>}
        </View>

        <View>
          <Button
            testID="keystoreCommonExists"
            title="keystoreCommonExists"
            onPress={this.keystoreCommonExists}
          />
          {!!isExists && <Text testID="isExists">{`isExists`}</Text>}
        </View>

        <View>
          <Button
            testID="keystoreCommonAccounts"
            title="keystoreCommonAccounts"
            onPress={this.keystoreCommonAccounts}
          />
          {!!accounts && <Text testID="accounts">{accounts}</Text>}
        </View>

        <View>
          <Button
            testID="keystoreCommonDelete"
            title="keystoreCommonDelete"
            onPress={this.keystoreCommonDelete}
          />
          {!!deleteSuccess && <Text testID="deleteSuccess">{`deleteSuccess`}</Text>}
        </View>
      </View>
    )
  }

  keystoreCommonVerify = async () => {
    const { id, password } = this.state
    try {
      this.setState({ isLoading: true })
      const res = await walletAPI.keystoreCommonVerify({ id, password })
      this.setState({ verifySuccess: res.isSuccess, isLoading: false })
    } catch (err) {
      this.setState({ isLoading: false })
      Alert.alert('', err.message)
    }
  }

  keystoreCommonExists = async () => {
    const { privateKey } = this.state
    try {
      this.setState({ isLoading: true })
      // @ts-ignore
      const res = await walletAPI.keystoreCommonExists({ type: 'PRIVATE_KEY', value: privateKey })
      this.setState({ isExists: res.isExists, isLoading: false })
    } catch (err) {
      this.setState({ isLoading: false })
      Alert.alert('', err.message)
    }
  }

  keystoreCommonAccounts = async () => {
    const { id } = this.state
    try {
      this.setState({ isLoading: true })
      const res = await walletAPI.keystoreCommonAccounts({ id })
      const accounts = res.accounts
      // @ts-ignore
      this.setState({ accounts: accounts[0].address, isLoading: false })
    } catch (err) {
      this.setState({ isLoading: false })
      Alert.alert('', err.message)
    }
  }

  keystoreCommonDelete = async () => {
    const { id, password } = this.state
    try {
      this.setState({ isLoading: true })
      const res = await walletAPI.keystoreCommonDelete({ id, password })
      // @ts-ignore
      this.setState({ deleteSuccess: res.isSuccess, isLoading: false })
    } catch (err) {
      this.setState({ isLoading: false })
      Alert.alert('', err.message)
    }
  }

  handleImport = async () => {
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
      this.setState({ id: res.id, address: res.accounts[0].address, isLoading: false })
    } catch (err) {
      this.setState({ isLoading: false })
      Alert.alert('', err.message)
    }
  }

  handleExport = async () => {
    const { id, password, chainType, network } = this.state
    try {
      this.setState({ isLoading: true })
      const res = await walletAPI.privateKeyStoreExport({ id, password, chainType, network })
      this.setState({ exportPrivateKey: res.value, isLoading: false })
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

export default CPK
