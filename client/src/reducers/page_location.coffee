{ GO_TO_LOG_IN, LOG_IN, GO_TO_HOME } = require '../constants/action_types.coffee'


Immutable = require 'immutable'

initialState = Immutable.fromJS
  currentPage: 'home'

pageLocation = (state = initialState, action) ->
  switch action.type
    when GO_TO_LOG_IN
      state.merge currentPage: 'login'
    when LOG_IN
      state.merge currentPage: 'home'
    when GO_TO_HOME
      state.merge currentPage: 'home'
    else state

module.exports = pageLocation
