{ ADD_TODO, DELETE_TODO } = require '../constants/action_types.coffee'
Immutable = require 'immutable'

initialState =
  Immutable.fromJS [
    { id: 0, text: 'hello' }
  ]

todos = (state = initialState, action) ->
  switch action.type
    when ADD_TODO
      state.push Immutable.fromJS
        id: 1 + state.reduce ((maxId, todo) -> Math.max(maxId, todo.get 'id')), 0
        text: action.text
    when DELETE_TODO
      state.filter (todo) -> todo.get('id') isnt action.id
    else state

module.exports = todos
