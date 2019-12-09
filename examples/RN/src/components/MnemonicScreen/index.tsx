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
  id: string
  address: string
  chainType: __chainType
  network: __networkType
  isLoading: boolean
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
      mnemonic: 'inject kidney empty canal shadow pact comfort wife crush horse wife sketch',
      password: '1234',
      id: '',
      address: '',
      chainType: 'BITCOINCASH' as __chainType,
      network: 'MAINNET' as __networkType,
      isLoading: false,
    }
  }
  render() {
    const { mnemonic, password, chainType, network, id, address, isLoading } = this.state
    return (
      <View style={styles.container}>
        <TextInput
          testID="mnemonicInput"
          value={mnemonic}
          placeholder={'mnemonic'}
          style={styles.input}
          onChangeText={(mnemonic) => this.setState({ mnemonic })}
          multiline
        />
        <TextInput
          testID="mnemonicPassword"
          value={password}
          placeholder={'password'}
          style={styles.input}
          onChangeText={(password) => this.setState({ password })}
        />
        <TextInput
          testID="mnemonicChainType"
          value={chainType}
          placeholder={'chainType'}
          style={styles.input}
          onChangeText={(chainType) => this.setState({ chainType: chainType as __chainType })}
        />
        <TextInput
          testID="mnemonicNetwork"
          value={network}
          placeholder={'network'}
          style={styles.input}
          onChangeText={(network) => this.setState({ network: network as __networkType })}
        />
        <Button
          testID="mnemonicSubmit"
          title="submit"
          onPress={this.handleSubmit}
        />
        {!!address && <Text testID="mnemonicAddress">{address}</Text>}
        <Loading animating={isLoading} />
      </View>
    )
  }

  handleSubmit = async () => {
    const { mnemonic, password, chainType, network } = this.state
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
        overwrite: true,
        passwordHint: ''
      }
      this.setState({ isLoading: true })
      const res = await walletAPI.importWalletFromMnemonic(params)
      console.log('res', res)
      this.setState({ id: res.id, address: res.accounts[0].address, isLoading: false })
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
