import { createAppContainer } from 'react-navigation'
import { createStackNavigator } from 'react-navigation-stack'

import HomeScreen from '../components/HomeScreen'
import MnemonicScreen from '../components/MnemonicScreen'


const MainNavigator = createStackNavigator(
  {
    Home: { screen: HomeScreen },
    Mnemonic: { screen: MnemonicScreen },
  },
  {
    initialRouteName: 'Home'
  }
)

const App = createAppContainer(MainNavigator)

export default App
