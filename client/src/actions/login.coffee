types = require '../constants/action_types.coffee'
{Promise} = require 'es6-promise'
Jwt = require 'jwt-simple'

LoginActions =
  logIn: (username, password) ->
    (dispatch) ->
      # TODO: No dummy
      payload =
        user: 'test-user'
        role: 'user'

      Promise.resolve({
        type: types.LOG_IN
        token: Jwt.encode payload, 'dummy-secret', 'HS512'
        username
      })
      .then (res) -> dispatch(res)

  logOut: -> type: types.LOG_OUT

module.exports = LoginActions
