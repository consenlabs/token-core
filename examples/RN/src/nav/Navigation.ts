import { createAppContainer } from 'react-navigation'
import { createStackNavigator } from 'react-navigation-stack'

import HomeScreen from '../components/HomeScreen'
import MnemonicScreen from '../components/MnemonicScreen'
import PrivateKeyScreen from '../components/PrivateKeyScreen'
import CreateScreen from '../components/CreateScreen'
import SignTxScreen from '../components/SignTxScreen'
import TronSignMessageScreen from '../components/TronSignMessageScreen'
import MnemonicPrivateKeyScreen from '../components/MnemonicPrivateKeyScreen'


const MainNavigator = createStackNavigator(
  {
    Home: { screen: HomeScreen },
    Mnemonic: { screen: MnemonicScreen },
    PrivateKey: { screen: PrivateKeyScreen },
    MnemonicPrivateKey: { screen: MnemonicPrivateKeyScreen},
    Create: { screen: CreateScreen },
    SignTx: { screen: SignTxScreen },
    TronSignMessage: { screen: TronSignMessageScreen },
  },
  {
    initialRouteName: 'Home'
  }
)

const App = createAppContainer(MainNavigator)

export default App
