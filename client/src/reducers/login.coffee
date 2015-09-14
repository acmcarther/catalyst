Immutable = require 'immutable'
Jwt = require 'jwt-simple'

{LOG_IN, LOG_OUT, REGISTER} = require '../constants/action_types.coffee'

initialState = Immutable.fromJS
  token: null
  username: null

handleLogin = (state, action) ->
  return state unless action.token?
  state.merge
    token: action.token
    username: action.username

login = (state = initialState, action) ->
  switch action.type
    when LOG_IN, REGISTER
      handleLogin state, action
    when LOG_OUT
      state.merge
        token: null
        username: null
    else state

module.exports = login
