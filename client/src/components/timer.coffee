React = require 'react'

{ div } = React.DOM

Timer = React.createClass
  getInitialState: ->
    return {secondsElapsed: 0};

  tick: ->
    @setState({secondsElapsed: @state.secondsElapsed + 1});

  componentDidMount: ->
    @interval = setInterval(@tick, 1000);

  componentWillUnmount: ->
    clearInterval(@interval);

  render: ->
    div {}, "Seconds Elapsed: #{@state.secondsElapsed}"

module.exports = React.createFactory Timer

