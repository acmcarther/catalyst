require './headerbar.styl'

React = require 'react'

{ div, span, } = React.DOM

HeaderBar = React.createClass
  render: ->
    div className: 'top-bar',
      span className: 'header-elem', 'Help'
      span className: 'header-elem login', 'Login'

module.exports = HeaderBar

