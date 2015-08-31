types = require '../constants/action_types.coffee'

PageLocationActions =
  goToLogIn: ->
    type: types.GO_TO_LOG_IN

  goToHome: ->
    type: types.GO_TO_HOME

module.exports = PageLocationActions
