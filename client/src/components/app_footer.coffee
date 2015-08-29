React = require 'react'

{ div, a } = React.DOM

AppFooter = React.createClass
  render: ->
    div {},
      div {}, 'Made by Alex McArther'
      a
        className: 'github-link'
        onClick: -> window.location = 'https://github.com/acmcarther/catalyst'
        'Catalyst on Github'

module.exports = React.createFactory AppFooter

