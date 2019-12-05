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
  address: string
  id: string
  isLoading: boolean
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
      address: '',
      id: '',
      isLoading: false,
    }
  }

  render() {
    const { privateKey, password, chainType, network, address, isLoading } = this.state
    return (
      <View style={styles.container}>
        <TextInput
          testID="privateKeyInput"
          value={privateKey}
          placeholder={'privateKey'}
          style={styles.input}
          onChangeText={(privateKey) => this.setState({ privateKey })}
          multiline
        />
        <TextInput
          testID="privateKeyPassword"
          value={password}
          placeholder={'password'}
          style={styles.input}
          onChangeText={(password) => this.setState({ password })}
        />
        <TextInput
          testID="privateKeyChainType"
          value={chainType}
          placeholder={'chainType'}
          style={styles.input}
          onChangeText={(chainType) => this.setState({ chainType: chainType as __chainType })}
        />
        <TextInput
          testID="privateKeyNetwork"
          value={network}
          placeholder={'network'}
          style={styles.input}
          onChangeText={(network) => this.setState({ network: network as __networkType })}
        />
        <Button
          testID="privateKeySubmit"
          title="submit"
          onPress={this.handleSubmit}
        />
        {!!address && <Text testID="privateKeyAddress">{address}</Text>}
        <Loading animating={isLoading} />
      </View>
    )
  }

  handleSubmit = async () => {
    const { privateKey, password, chainType, network } = this.state
    try {
      const params = {
        chainType,
        network,
        password,
        privateKey: privateKey.trim(),
        overwrite: true,
        passwordHint: ''
      }
      this.setState({ isLoading: true })
      const res = await walletAPI.importWalletFromPrivateKey(params)
      console.log('res', res)
      this.setState({ id: res.id, address: res.address, isLoading: false })
    } catch (err) {
      this.setState({ isLoading: true })
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
