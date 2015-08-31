{ GO_TO_LOG_IN, LOG_IN, LOG_OUT, GO_TO_HOME, GO_TO_HELP } = require '../constants/action_types.coffee'

Immutable = require 'immutable'

initialState = Immutable.fromJS
  currentPage: 'home'

pageLocation = (state = initialState, action) ->
  switch action.type
    when LOG_IN, LOG_OUT, GO_TO_HOME
      state.merge currentPage: 'home'
    when GO_TO_LOG_IN
      state.merge currentPage: 'login'
    when GO_TO_HELP
      state.merge currentPage: 'help'
    else state

module.exports = pageLocation
