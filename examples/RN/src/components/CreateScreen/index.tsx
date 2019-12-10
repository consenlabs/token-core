import React from 'react'
import { StyleSheet, View, Text, TextInput, Button, Alert } from 'react-native'
import walletAPI from '../../native'
import { getChainPath } from '../../constant/path'
import Loading from '../Loading'

interface Props {
}

interface State {
  password: string
  segWit: string
  id: string | null | undefined
  address: string | null | undefined
  expectedAddress: any
  chainType: __chainType
  network: __networkType
  isLoading: boolean
  mnemonic: any
}

class CMP extends React.Component<Props, State> {
  static navigationOptions = ({ navigation }: any) => {
    return {
      title: 'Create',
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
      password: '',
      id: '',
      address: '',
      expectedAddress: '',
      chainType: '' as __chainType,
      network: '' as __networkType,
      segWit: '',
      isLoading: false,
      mnemonic: '',
    }
  }
  render() {
    const { isLoading, password, chainType, network, segWit, address, mnemonic } = this.state
    const inputs = {
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
            testID="submit-btn"
            title="create"
            onPress={this.handleSubmit}
          />
          {!!address && <Text testID="expected-address">{address}</Text>}
        </View>
        <View>
          <Button
            testID="export-btn"
            title="export"
            onPress={this.handleExport}
          />
          {!!mnemonic && <Text testID="expected-mnemonic">{mnemonic}</Text>}
        </View>
        {this.renderImport()}
      </View>
    )
  }

  renderImport() {
    const { password, chainType, network, segWit, expectedAddress, mnemonic } = this.state
    const inputs = {
      mnemonic,
      password,
      chainType,
      network,
      segWit,
    }
    return (
      <View>
        {
          Object.keys(inputs).map((v) => {
            // @ts-ignore
            return <Text key={v}>{inputs[v]}</Text>
          })
        }
        <Button
          testID="import-btn"
          title="import"
          onPress={this.handleImport}
        />
        {!!expectedAddress && <Text testID="import-address">{expectedAddress}</Text>}
      </View>
    )
  }

  handleSubmit = async () => {
    const { password, chainType, network, segWit } = this.state
    const chainPath = getChainPath(chainType, network)
    try {
      const params = {
        password,
        name: '',
        passwordHint: ''
      }
      console.log('params', params)
      this.setState({ isLoading: true })
      const res = await walletAPI.hdStoreCreate(params)
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

  handleExport = async () => {
    const { id, password } = this.state
    try {
      this.setState({ isLoading: true })
      const res = await walletAPI.hdStoreExport({ id, password })
      this.setState({ mnemonic: res.value, isLoading: false })
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
      console.log('params', params)
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
      this.setState({ id: res.id, expectedAddress: address, isLoading: false })
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
