Immutable = require 'immutable'

{GO_TO_LOG_IN, LOG_IN, LOG_OUT, GO_TO_HOME, GO_TO_HELP, GO_TO_REPO, GO_TO_REGISTER, REGISTER} = require '../constants/action_types.coffee'

initialState = Immutable.fromJS
  currentPage: 'home'
  meta: null

pageLocation = (state = initialState, action) ->
  switch action.type
    when GO_TO_REPO
      state.merge
        currentPage: 'repo'
        meta: repoId: action.repoId
    when LOG_IN, LOG_OUT, GO_TO_HOME, REGISTER
      state.merge currentPage: 'home'
    when GO_TO_LOG_IN
      state.merge currentPage: 'login'
    when GO_TO_HELP
      state.merge currentPage: 'help'
    when GO_TO_REGISTER
      state.merge currentPage: 'registration'
    else state

module.exports = pageLocation
