import React from 'react'
import { Provider } from 'react-redux'
import App from './containers/app'
import configureStore from './store/configure_store'


var store = configureStore()

var extras = () => {
  if (!__DEVTOOLS__) { return null }

  const { DevTools, DebugPanel, LogMonitor } = require('redux-devtools/lib/react')

  return (
    <DebugPanel top={true} bottom={true}>
      <DevTools store={store} monitor={LogMonitor} />
    </DebugPanel>
  )
}

React.render(
  <div>
    <Provider store={store}>
      {() => {
        return (
          <div>
            <App />
            { extras() }
          </div>
        )
      }}
    </Provider>
  </div>
  , document.body
)
