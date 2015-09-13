React = require 'react'
{div} = React.DOM
Provider = React.createFactory require('react-redux').Provider

App = React.createFactory require './containers/app.coffee'

configureStore = require './store/configure_store.coffee'

if __DEVTOOLS__
  {DevTools, DebugPanel, LogMonitor} = require 'redux-devtools/lib/react'
  DevTools = React.createFactory DevTools
  DebugPanel = React.createFactory DebugPanel

store = configureStore()

React.render(
  (div {},
    Provider {store}, App
    if __DEVTOOLS__
      DebugPanel top: true, bottom: true,
        DevTools {store, monitor: LogMonitor})
  , document.body
)
