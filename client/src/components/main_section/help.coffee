React = require 'react'
{ div } = React.DOM

Help = React.createClass
  render: ->
    div {},
      div {}, 'Help'
      div {}, 'No help yet'

module.exports = Help
