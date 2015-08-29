CoreDispatcher = require '../dispatchers/core_dispatcher.coffee'
CoreConstants = require '../constants/core_constants.coffee'

CoreActions =
  create: (text) ->
    CoreDispatcher.dispatch
      actionType: CoreConstants.CORE_CREATE
      body: text

  updateText: (id, text) ->
    CoreDispatcher.dispatch
      actionType: CoreConstants.CORE_UPDATE
      id: id
      body: text

  deleteItem: (id) ->
    CoreDispatcher.dispatch
      actionType: CoreConstants.CORE_DESTROY
      id: id

module.exports = CoreActions
