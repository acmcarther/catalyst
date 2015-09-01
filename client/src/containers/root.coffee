React = require 'react'
Provider = React.createFactory require('react-redux').Provider
{ applyMiddleware, compose, createStore } = require 'redux'
promiseMiddleware = require 'redux-promise'
thunkMiddleware = require 'redux-thunk'

rootReducer = require '../reducers/root.coffee'
App = React.createFactory require './app.coffee'

if __DEVTOOLS__
  { devTools } = require 'redux-devtools'
  finalCreateStore = compose(
    applyMiddleware thunkMiddleware
    devTools()
    createStore
  )
  store = finalCreateStore rootReducer
else
  finalCreateStore = compose(
    applyMiddleware thunkMiddleware
    createStore
  )
  store = finalCreateStore rootReducer

{ div } = React.DOM

Root = React.createClass
  render: ->
    div {},
      if __DEVTOOLS__
        { DevTools, DebugPanel, LogMonitor } = require 'redux-devtools/lib/react'
        DevTools = React.createFactory DevTools
        DebugPanel = React.createFactory DebugPanel
        DiffMonitor = require 'redux-devtools-diff-monitor'

        DebugPanel { top: false, right: true, bottom: true },
          DevTools { store, monitor: DiffMonitor }
      Provider { store }, App

module.exports = Root
