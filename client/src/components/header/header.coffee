require './header.styl'
React = require 'react'

{ div, h1, h3 } = React.DOM

HeaderBar = React.createFactory require './headerbar.coffee'

Header = React.createClass
  render: ->
    div {},
      div {}, HeaderBar {}
      div className: 'app-title',
        h1  {}, 'Catalyst'
        h3  {}, 'A bot to manage your github build process!'

module.exports = Header
