React = require 'react'
{div} = React.DOM

Home = React.createClass
  render: ->
    div {},
      div {}, 'Home'
      div {}, 'You should log in'

module.exports = Home
