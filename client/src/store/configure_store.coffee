{applyMiddleware, compose, createStore} = require 'redux'
thunkMiddleware = require 'redux-thunk'
rootReducer = require '../reducers/root.coffee'

if __DEVTOOLS__
  {devTools, persistState} = require 'redux-devtools'

finalCreateStore =
  if __DEVTOOLS__
    compose(
      applyMiddleware thunkMiddleware
      devTools()
      persistState(window.location.href.match(/[?&]debug_session=([^&]+)\b/))
    )(createStore)
  else
    compose(applyMiddleware thunkMiddleware)(createStore)

configureStore = (initialState) ->
  store = finalCreateStore rootReducer, initialState

  console.log 'module', module
  if module.hot
    module.hot.accept '../reducers/root.coffee', ->
      store.replaceReducer require '../reducers/root.coffee'

  store

module.exports = configureStore
