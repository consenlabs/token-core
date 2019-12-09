/**
 * Sample React Native App
 * https://github.com/facebook/react-native
 *
 * Generated with the TypeScript template
 * https://github.com/react-native-community/react-native-template-typescript
 *
 * @format
 */

import React from 'react'
import { StyleSheet, ScrollView, View, Text, TouchableOpacity } from 'react-native'
import { Header, Colors } from 'react-native/Libraries/NewAppScreen'

interface Props {
  navigation: any
}

class HomeScreen extends React.Component<Props> {
  static navigationOptions = {
    title: 'Home',
  }

  render() {
    const screens = [
      { name: 'Mnemonic' },
      { name: 'PrivateKey' },
    ]

    return (
      <ScrollView
        contentInsetAdjustmentBehavior="automatic"
        style={styles.scrollView}>
        <Header />
        <View style={styles.body}>
          {
            screens.map(((screen, i) => {
              return (
                <TouchableOpacity
                  testID={screen.name}
                  key={screen.name}
                  onPress={() => this.handleNavigate(screen)}
                  style={styles.item}
                >
                  <Text>{screen.name}</Text>
                </TouchableOpacity>
              )
            }))
          }
        </View>
      </ScrollView>
    )
  }

  handleNavigate = (screen: any) => {
    this.props.navigation.push(screen.name)
  }
}

const styles = StyleSheet.create({
  scrollView: {
    flex: 1,
    backgroundColor: Colors.lighter,
  },
  body: {
    backgroundColor: Colors.white,
  },
  item: {
    padding: 10,
    justifyContent: 'center',
  },
})

export default HomeScreen
