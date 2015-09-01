Immutable = require 'immutable'
Jwt = require 'jwt-simple'

{BEGIN_LOG_IN, LOG_IN, LOG_OUT} = require '../constants/action_types.coffee'

initialState = Immutable.fromJS login: null

handleLogin = (state, action) ->
  return state unless action.token?
  state.merge
    login:
      token: action.token
      username: action.username

login = (state = initialState, action) ->
  switch action.type
    when BEGIN_LOG_IN
      state.merge loggingIn: true
    when LOG_IN
      handleLogin state, action
    when LOG_OUT
      state.merge login: null
    else state

module.exports = login
