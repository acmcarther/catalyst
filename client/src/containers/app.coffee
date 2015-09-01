React = require 'react'
PropTypes = React.PropTypes
{div} = React.DOM
{bindActionCreators} = require 'redux'
{connect} = require 'react-redux'

Header = React.createFactory require '../components/header/header.coffee'
Footer = React.createFactory require '../components/footer/footer.coffee'
MainSection = React.createFactory require '../components/main_section/main_section.coffee'
LoginActions = require '../actions/login.coffee'
PageLocationActions = require '../actions/page_location.coffee'

App = React.createClass
  render: ->
    {login, pageLocation, dispatch} = @props
    loginActions = bindActionCreators LoginActions, dispatch
    pageLocationActions = bindActionCreators PageLocationActions, dispatch

    div {},
      Header {login, loginActions, pageLocationActions}
      MainSection {pageLocation, loginActions}

App.propTypes =
  dispatch: PropTypes.func.isRequired

select = (state) ->
  login: state.login
  pageLocation: state.pageLocation

module.exports = connect(select)(App)
