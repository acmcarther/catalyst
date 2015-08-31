React = require 'react'

PropTypes = React.PropTypes

{ section, ul, div, button, input} = React.DOM

MainSection = React.createClass
  getInitialState: ->
    newItemText: ''

  render: ->
    { todos, todoActions } = @props

    div {},
      div {}, 'Main Section'
      section className: 'main',
        ul className: 'todo-list',
          todos.map (item, idx) ->
            id = item.get 'id'
            div
              key: id,
              onClick: -> todoActions.deleteTodo id
              item.get 'text'

      input
        onChange: (e) => @setState newItemText: e.target.value
        value: @state.newItemText
        type: 'text'

      button
        onClick: =>
          @setState newItemText: ''
          if @state.newItemText isnt '' then todoActions.addTodo @state.newItemText
        'Add new item'



MainSection.propTypes =
  todoActions: PropTypes.object.isRequired

module.exports = MainSection
