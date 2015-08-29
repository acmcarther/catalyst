require './core.styl'

React = require 'react'
Body = React.createFactory(require './body.coffee')
Header = React.createFactory(require './header/header.coffee')
Footer = React.createFactory(require './footer/footer.coffee')

{div, h1, h2} = React.DOM

Main = React.createClass
  render: ->
    div {},
      div className: 'main-header', Header {}
      div className: 'main-body', Body {}
      div className: 'main-footer', Footer {}

module.exports = Main

