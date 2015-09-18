{Promise} = require 'es6-promise'
Jwt = require 'jwt-simple'

types = require '../constants/action_types.coffee'

{getRepos} = require './repo.coffee'

RegisterActions =
  register: (username, password) ->
    (dispatch) ->
      payload =
        user: 'test-user'
        role: 'user'

      registerResult = Promise.resolve({
        type: types.REGISTER_SUCCESS
        token: Jwt.encode payload, 'dummy-secret', 'HS512'
        username
      })

      registerResult.then dispatch
      registerResult.then ({token}) ->
        setTimeout (=> getRepos(token)(dispatch)), 5000

module.exports = RegisterActions
