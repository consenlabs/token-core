import React from 'react'
import { ActivityIndicator } from 'react-native'

interface Props {
  color?: any
  size?: 'small' | 'large'
  animating?: boolean
  style?: object
}

function Loading(props: Props) {
  props.color = props.color || '#333'
  props.size = props.size || 'small'
  return <ActivityIndicator {...props} />
}

export default Loading
