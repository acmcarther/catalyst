CoreDispatcher = require '../dispatchers/core_dispatcher.coffee'
EventEmitter = require('events').EventEmitter
CoreConstants = require '../constants/core_constants.coffee'
assign = require 'object-assign'
Immutable = require 'immutable'

CHANGE_EVENT = 'change'

items = Immutable.Map()

create = (text) ->
  id = (+new Date() + Math.floor(Math.random() * 999999).toString(36))
  items = items.merge "#{id}": {id, text}

update = (id, updates) ->
  items = items.merge "#{id}": updates

destroy = (id) -> items = items.delete id

CoreStore = assign {}, EventEmitter.prototype, {
  getAll: -> items

  emitChange: -> @emit CHANGE_EVENT

  addChangeListener: (callback) -> @on CHANGE_EVENT, callback

  removeChangeListener: (callback) -> @removeListener CHANGE_EVENT, callback
}

CoreDispatcher.register (action) ->
  switch action.actionType
    when CoreConstants.CORE_CREATE
      create action.body
      CoreStore.emitChange()
    when CoreConstants.CORE_DESTROY
      destroy action.id
      CoreStore.emitChange()
    when CoreConstants.CORE_UPDATE
      update action.id, text: action.body
      CoreStore.emitChange()

module.exports = CoreStore
