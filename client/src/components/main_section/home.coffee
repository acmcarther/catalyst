React = require 'react'

{ div } = React.DOM

Home = React.createClass
  render: ->
    div {},
      div {}, 'Home'

module.exports = Home
