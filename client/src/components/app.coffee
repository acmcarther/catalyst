require './app.styl'

React = require 'react'
Timer = require './timer.coffee'
AppHeader = require './app_header.coffee'
AppFooter = require './app_footer.coffee'

{div, h1, h2} = React.DOM

Main = React.createClass
  render: ->
    div {},
      div className: 'main-header', AppHeader {}
      div className: 'main-body', Timer {}
      div className: 'main-footer', AppFooter {}

module.exports = React.createFactory Main

