{ BEGIN_LOG_IN, LOG_IN, LOG_OUT } = require '../constants/action_types.coffee'

Immutable = require 'immutable'
Jwt = require 'jwt-simple'

initialState = Immutable.fromJS login: null

login = (state = initialState, action) ->
  switch action.type
    when BEGIN_LOG_IN
      state.merge loggingIn: true
    when LOG_IN
      return state unless action.token?
      state.merge
        login:
          token: action.token
          username: action.username
    when LOG_OUT then state.merge login: null
    else state

module.exports = login
