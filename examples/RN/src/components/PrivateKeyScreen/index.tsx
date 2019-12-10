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
  segWit: string
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
      segWit: '',
      id: '',
      isLoading: false,
    }
  }

  render() {
    const { privateKey, password, chainType, network, address, segWit, isLoading } = this.state
    const inputs = {
      privateKey,
      password,
      chainType,
      network,
      segWit,
    }
    return (
      <View style={styles.container}>
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
          testID="import-btn"
          title="import"
          onPress={this.handleImport}
        />
        {!!address && <Text testID="import-address">{address}</Text>}
        <Loading animating={isLoading} />
      </View>
    )
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
      console.log('res', res)
      // @ts-ignore
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

export default CPK
