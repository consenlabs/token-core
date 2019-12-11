import React from 'react'
import { StyleSheet, View, Text, TextInput, Button, Alert } from 'react-native'
import walletAPI from '../../native'
import { getChainPath } from '../../constant/path'
import Loading from '../Loading'

interface Props { }

interface State {
  id: any
  password: string
  chainType: __chainType
  address: any
  input: string

  mnemonic: string
  isLoading: boolean
  signature: any
}

class CPK extends React.Component<Props, State> {
  static navigationOptions = ({ navigation }: any) => {
    return {
      title: 'TronSignMessage',
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
      id: '',
      password: '',
      chainType: '' as __chainType,
      address: '',
      mnemonic: '',
      isLoading: false,
      signature: '',
      input: '',
    }
  }

  render() {
    const { isLoading } = this.state
    return (
      <View style={styles.container}>
        <Loading animating={isLoading} />
        {this.renderImport()}
        {this.renderSign()}
      </View>
    )
  }

  renderSign() {
    const { signature, input } = this.state
    const inputs = {
      input,
    }
    return (
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
          testID="sign"
          title="sign"
          onPress={this.sign}
        />
        {!!signature && <Text testID="signature">{signature}</Text>}
      </View>
    )
  }

  renderImport() {
    const { mnemonic, password, chainType, address, isLoading } = this.state
    const inputs = {
      mnemonic,
      password,
      chainType,
    }
    return (
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
    )
  }

  sign = async () => {
    const { id, password, chainType, address, input } = this.state
    const params = {
      id,
      password,
      chainType,
      address,
      input: JSON.parse(input),
    }
    this.setState({ isLoading: true })

    try {
      const res = await walletAPI.tronSignMessage(params)
      this.setState({ signature: res.signature, isLoading: false })
    } catch (err) {
      this.setState({ isLoading: false })
      Alert.alert('', err.message)
    }
  }

  handleImport = async () => {
    const { mnemonic, password, chainType } = this.state
    const chainPath = getChainPath(chainType)
    try {
      const params = {
        chainType,
        name: 'MNEMONIC-test',
        source: 'MNEMONIC' as __walletSource,
        path: chainPath,
        mnemonic: mnemonic.trim(),
        password,
        overwrite: true,
        segWit: '',
        network: '',
        passwordHint: ''
      }
      this.setState({ isLoading: true })
      const res = await walletAPI.hdStoreImport(params)
      const deriveParams = {
        chainType,
        path: chainPath,
        network: '',
        segWit: '',
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

export default CPK
