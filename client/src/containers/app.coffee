React = require 'react'
{ bindActionCreators } = require 'redux'
{ connect } = require 'react-redux'

Header = React.createFactory require '../components/header/header.coffee'
Footer = React.createFactory require '../components/footer/footer.coffee'
MainSection = React.createFactory require '../components/main_section.coffee'
TodoActions = require '../actions/todos.coffee'

{ div } = React.DOM
PropTypes = React.PropTypes

App = React.createClass
  render: ->
    {todos, dispatch} = @props
    actions = bindActionCreators TodoActions, dispatch

    div {},
      Header {}
      MainSection { todos, actions }

App.propTypes =
  dispatch: PropTypes.func.isRequired

select = (state) -> todos: state.todos

module.exports = connect(select)(App)
