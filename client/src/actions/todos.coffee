types = require '../constants/action_types.coffee'

TodoActions =
  addTodo: (text) -> {
    type: types.ADD_TODO
    text
  }

  deleteTodo: (id) -> {
    type: types.DELETE_TODO
    id
  }

module.exports = TodoActions

