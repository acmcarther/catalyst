import Immutable from 'immutable'
import Jwt from 'jwt-simple'
import { LOG_IN, LOG_OUT, REGISTER_SUCCESS } from '../constants/action_types'


const initialState = Immutable.fromJS({
  token: null,
  username: null
})

var nullOrUndefined = (value) => {
  return value === undefined || value === null
}

var handleLogin = (state, action) => {
  if (!(nullOrUndefined(action.token))) {
    return state
  }

  return state.merge({
    token: action.token,
    username: action.username
  })
}

var login = (state = initialState, action) => {
  switch (action.type) {
    case LOG_IN, REGISTER_SUCCESS:
      return handleLogin(state, action)
    case LOG_OUT:
      return state.merge({
        token: null,
        username: null
      })
    default:
      return state
  }
}


export default login
