React = require 'react'
{ div, span, a } = React.DOM

require './footer.styl'

AppFooter = React.createClass
  render: ->
    div className: 'bottom-bar',
      span className: 'left-container',
        span className: 'footer-elem', 'Made by Alex McArther'
      span className: 'right-container',
        a
          className: 'footer-elem github-link'
          onClick: -> window.location = 'https://github.com/acmcarther/catalyst'
          'Catalyst on Github'

module.exports = AppFooter

