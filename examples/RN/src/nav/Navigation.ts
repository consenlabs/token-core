import { createAppContainer } from 'react-navigation'
import { createStackNavigator } from 'react-navigation-stack'

import HomeScreen from '../components/HomeScreen'
import MnemonicScreen from '../components/MnemonicScreen'
import PrivateKeyScreen from '../components/PrivateKeyScreen'
import CreateScreen from '../components/CreateScreen'


const MainNavigator = createStackNavigator(
  {
    Home: { screen: HomeScreen },
    Mnemonic: { screen: MnemonicScreen },
    PrivateKey: { screen: PrivateKeyScreen },
    Create: { screen: CreateScreen },
  },
  {
    initialRouteName: 'Home'
  }
)

const App = createAppContainer(MainNavigator)

export default App
