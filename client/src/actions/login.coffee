type = require '../constants/action_types.coffee'
Promise = require 'es6-promise'

LoginActions =
  logIn: (username, password) ->
    (dispatch) ->
      # TODO: No dummy
      payload =
        user: 'test-user'
        role: 'user'

      Promise.resolve {
        type: types.LOG_IN
        token: jwt.encode payload, 'dummy-secret', 'HS512'
        username
      }

  logOut: -> type: types.LOG_OUT

module.exports = LoginActions
