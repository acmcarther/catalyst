types = require '../constants/action_types.coffee'

PageLocationActions =
  goToLogIn: -> type: types.GO_TO_LOG_IN
  goToRegistration: -> type: types.GO_TO_REGISTER
  goToHelp: -> type: types.GO_TO_HELP
  goToHome: -> type: types.GO_TO_HOME

  goToRepo: (repoId) ->
    {
      type: types.GO_TO_REPO
      repoId
    }

module.exports = PageLocationActions
