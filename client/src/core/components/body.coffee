require './body.styl'
React = require 'react'

CoreActions = require '../actions/core_actions.coffee'
CoreStore = require '../stores/core_store.coffee'

{ div, span, input, button } = React.DOM

Immutable = require 'immutable'

getCoreState = -> allItems: CoreStore.getAll()

Body = React.createClass
  getInitialState: ->
    state = Immutable.Map(itemText: '')
    data: state.merge getCoreState()

  componentDidMount: ->
    CoreStore.addChangeListener @_onChange

  componentWillUnmount:->
    CoreStore.removeChangeListener @_onChange

  render: ->
    div {},
      @state.data.get('allItems').entrySeq().map ([id, item], idx) =>
        div key: idx,
          span key: idx, item.get 'text'
          button
            onClick: => CoreActions.deleteItem id
            'Delete Item'

      div {},
        div className: 'description', 'Debug list component'
        input
          type: 'text'
          placeholder: 'Item details'
          onChange: (e) =>
            itemText = e.target.value
            @setState data: @state.data.merge {itemText}
        button
          onClick: =>
            @setState data: @state.data.merge itemText: ''
            CoreActions.create @state.data.get 'itemText'
          'click to add item'

  _onChange: -> @setState data: @state.data.merge getCoreState()

module.exports = Body

