{Promise} = require 'es6-promise'
Jwt = require 'jwt-simple'

types = require '../constants/action_types.coffee'

{getRepos} = require './repo.coffee'

LoginActions =
  logIn: (username, password) ->
    (dispatch) ->
      # TODO: No dummy
      payload =
        user: 'test-user'
        role: 'user'

      logInResult = Promise.resolve({
        type: types.LOG_IN
        token: Jwt.encode payload, 'dummy-secret', 'HS512'
        username
      })

      logInResult.then dispatch
      logInResult.then ({token}) ->
        setTimeout (=> getRepos(token)(dispatch)), 5000

  logOut: -> type: types.LOG_OUT

module.exports = LoginActions
