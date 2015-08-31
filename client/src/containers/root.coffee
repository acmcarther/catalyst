React = require 'react'
Provider = React.createFactory require('react-redux').Provider
{ compose, createStore } = require 'redux'
rootReducer = require '../reducers/root.coffee'


if __DEVTOOLS__
  { devTools } = require 'redux-devtools'
  { DevTools, DebugPanel, LogMonitor } = require 'redux-devtools/lib/react'
  DevTools = React.createFactory DevTools
  DebugPanel = React.createFactory DebugPanel
  DiffMonitor = require 'redux-devtools-diff-monitor'

App = React.createFactory require './app.coffee'

if __DEVTOOLS__
  finalCreateStore = compose(
    devTools()
    createStore
  )
  store = finalCreateStore rootReducer
else
  store = createStore rootReducer

{ div } = React.DOM

Root = React.createClass 
  render: ->
    div {},
      if __DEVTOOLS__
        DebugPanel { top: false, right: true, bottom: true },
          DevTools { store, monitor: DiffMonitor }
      Provider { store }, App

module.exports = Root
