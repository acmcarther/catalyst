React = require 'react'

PropTypes = React.PropTypes

{ section, ul, div, button, input} = React.DOM

MainSection = React.createClass
  getInitialState: ->
    newItemText: ''

  render: ->
    { todos, actions } = @props

    div {},
      div {}, 'Main Section'
      section className: 'main',
        ul className: 'todo-list',
          todos.map (item, idx) ->
            id = item.get 'id'
            div
              key: id,
              onClick: -> actions.deleteTodo id
              item.get 'text'

      input
        onChange: (e) => @setState newItemText: e.target.value
        value: @state.newItemText
        type: 'text'

      button
        onClick: =>
          @setState newItemText: ''
          if @state.newItemText isnt '' then actions.addTodo @state.newItemText
        'Add new item'



MainSection.propTypes =
  actions: PropTypes.object.isRequired

module.exports = MainSection
