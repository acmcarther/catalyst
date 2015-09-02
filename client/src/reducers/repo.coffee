Immutable = require 'immutable'
Jwt = require 'jwt-simple'

{GET_REPOS, LOG_OUT} = require '../constants/action_types.coffee'

initialState = Immutable.fromJS
  list: []
  loaded: false

repo = (state = initialState, action) ->
  switch action.type
    when GET_REPOS
      Immutable.fromJS
        list: action.repos
        loaded: true
    when LOG_OUT
      Immutable.fromJS
        list: []
        loaded: false
    else state

module.exports = repo
