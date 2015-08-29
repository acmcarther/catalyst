Timer = require './timer.coffee'

{div, h1, h2} = React.DOM

Main = React.createClass
  render: ->
    div {},
      h1 {}, 'Catalyst'
      h2 {}, 'A github build management bot'
      div {}, Timer {}

module.exports = React.createFactory Main

