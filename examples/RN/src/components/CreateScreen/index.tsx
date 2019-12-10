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
  chainType: __chainType
  network: __networkType
  isLoading: boolean
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
      chainType: '' as __chainType,
      network: '' as __networkType,
      segWit: '',
      isLoading: false,
    }
  }
  render() {
    const { password, chainType, network, segWit, address, isLoading } = this.state
    const inputs = {
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
          testID="submit-btn"
          title="create"
          onPress={this.handleSubmit}
        />
        {!!address && <Text testID="expected-address">{address}</Text>}
        <Loading animating={isLoading} />
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
