React = require 'react'
{ bindActionCreators } = require 'redux'
{ connect } = require 'react-redux'

Header = React.createFactory require '../components/header/header.coffee'
Footer = React.createFactory require '../components/footer/footer.coffee'
MainSection = React.createFactory require '../components/main_section.coffee'
TodoActions = require '../actions/todos.coffee'
LoginActions = require '../actions/login.coffee'

{ div } = React.DOM
PropTypes = React.PropTypes

App = React.createClass
  render: ->
    {todos, login, dispatch} = @props
    todoActions = bindActionCreators TodoActions, dispatch
    loginActions = bindActionCreators LoginActions, dispatch

    div {},
      Header { login, loginActions }
      MainSection { todos, todoActions }

App.propTypes =
  dispatch: PropTypes.func.isRequired

select = (state) ->
  todos: state.todos
  login: state.login

module.exports = connect(select)(App)
