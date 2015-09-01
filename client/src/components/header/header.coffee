React = require 'react'
PropTypes = React.PropTypes
{div, h1, h3} = React.DOM

require './header.styl'
HeaderBar = React.createFactory require './headerbar.coffee'

Header = React.createClass
  render: ->
    {login, loginActions, pageLocationActions} = @props
    div {},
      div {}, HeaderBar {login, loginActions, pageLocationActions}
      div className: 'app-title',
        h1  {}, 'Catalyst'
        h3  {}, 'A bot to manage your github build process!'

Header.propTypes =
  loginActions: PropTypes.object.isRequired
  pageLocationActions: PropTypes.object.isRequired

module.exports = Header
