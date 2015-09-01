React = require 'react'
PropTypes = React.PropTypes
{div, span} = React.DOM

require './headerbar.styl'

HeaderBar = React.createClass
  render: ->
    {login, loginActions, pageLocationActions} = @props
    username = login.getIn ['login', 'username']
    div className: 'top-bar',
      if username?
        span
          className: 'header-elem logged-in-as'
          "Welcome, #{username}"
      span
        className: 'header-elem clickable'
        onClick: -> pageLocationActions.goToHome()
        'Home'
      span
        className: 'header-elem clickable',
        onClick: -> pageLocationActions.goToHelp()
        'Help'
      if username?
        span
          className: 'header-elem clickable'
          onClick: -> loginActions.logOut()
          'Log out'
      else
        span
          className: 'header-elem clickable',
          onClick: -> pageLocationActions.goToLogIn()
          'Login'

HeaderBar.propTypes =
  loginActions: PropTypes.object.isRequired
  pageLocationActions: PropTypes.object.isRequired

module.exports = HeaderBar
