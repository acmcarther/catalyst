React = require 'react'
{div} = React.DOM
Provider = React.createFactory require('react-redux').Provider
{applyMiddleware, compose, createStore} = require 'redux'
thunkMiddleware = require 'redux-thunk'

rootReducer = require '../reducers/root.coffee'
App = React.createFactory require './app.coffee'

if __DEVTOOLS__
  {DevTools, DebugPanel, LogMonitor} = require 'redux-devtools/lib/react'
  DevTools = React.createFactory DevTools
  DebugPanel = React.createFactory DebugPanel
  DiffMonitor = require 'redux-devtools-diff-monitor'
  {devTools} = require 'redux-devtools'

finalCreateStore =
  if __DEVTOOLS__
    compose(
      applyMiddleware thunkMiddleware
      devTools()
      createStore
    )
  else
    compose(
      applyMiddleware thunkMiddleware
      createStore
    )

store = finalCreateStore rootReducer

Root = React.createClass
  render: ->
    div {},
      if __DEVTOOLS__
        DebugPanel {top: false, right: true, bottom: true},
          DevTools {store, monitor: DiffMonitor}
      Provider {store}, App

module.exports = Root
