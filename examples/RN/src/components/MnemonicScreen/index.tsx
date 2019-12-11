import React from 'react'
import { StyleSheet, View, Text, TextInput, Button, Alert } from 'react-native'
import walletAPI from '../../native'
import { getChainPath } from '../../constant/path'
import Loading from '../Loading'

interface Props {
}

interface State {
  mnemonic: string
  password: string
  segWit: string
  id: string | null | undefined
  address: string | null | undefined
  chainType: __chainType
  network: __networkType
  isLoading: boolean
  verifySuccess: any
  isExists: any
  accounts: any
  deleteSuccess: any
}

class CMP extends React.Component<Props, State> {
  static navigationOptions = ({ navigation }: any) => {
    return {
      title: 'Mnemonic',
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
      id: '',
      address: '',
      chainType: '' as __chainType,
      network: '' as __networkType,
      segWit: '',
      isLoading: false,
      verifySuccess: false,
      isExists: false,
      deleteSuccess: false,
      accounts: '',
    }
  }
  render() {
    const { mnemonic, password, chainType, network, segWit, address, isLoading } = this.state
    const inputs = {
      mnemonic,
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
    const { mnemonic } = this.state
    try {
      this.setState({ isLoading: true })
      // @ts-ignore
      const res = await walletAPI.keystoreCommonExists({ type: 'MNEMONIC', value: mnemonic })
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
    const { mnemonic, password, chainType, network, segWit } = this.state
    const chainPath = getChainPath(chainType, network)
    try {
      const params = {
        chainType,
        network,
        name: 'MNEMONIC-test',
        source: 'MNEMONIC' as __walletSource,
        path: chainPath,
        mnemonic: mnemonic.trim(),
        password,
        segWit,
        overwrite: true,
        passwordHint: ''
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
      const address = accountsRes.accounts[0].address
      this.setState({ id: res.id, address, isLoading: false })
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
