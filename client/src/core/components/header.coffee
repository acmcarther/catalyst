React = require 'react'

{ div, h1, h3 } = React.DOM

AppHeader = React.createClass
  render: ->
    div {},
      h1  {}, 'Catalyst'
      h3  {}, 'A bot to manage your github build process!'

module.exports = AppHeader
