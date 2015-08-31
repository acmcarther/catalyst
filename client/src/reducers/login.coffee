{ LOG_IN, LOG_OUT } = require '../constants/action_types.coffee'

Immutable = require 'immutable'
Jwt = require 'jwt-simple'

initialState = Immutable.fromJS
  login: null

# TODO: Validation
validatedToken = (jwt) -> jwt

login = (state = initialState, action) ->
  switch action.type
    when LOG_IN
      return state unless token?
      Immutable.fromJS
        login:
          token: token
          username: username
    when LOG_OUT then Immutable.fromJS login: null
    else state

module.exports = login





