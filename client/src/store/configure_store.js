import { applyMiddleware, compose, createStore } from 'redux'
import thunkMiddleware from 'redux-thunk'
import rootReducer from '../reducers/root'


var finalCreateStore = () => {
  if (__DEVTOOLS__) {
    var { devTools, persistState } = require('redux-devtools')

    return compose(
      applyMiddleware(thunkMiddleware),
      devTools(),
      persistState(window.location.href.match(/[?&]debug_session=([^&]+)\b/))
    )(createStore)
  }

  return compose(applyMiddleware(thunkMiddleware))(createStore)
}

var configureStore = (initialState) => {
  var store = finalCreateStore()(rootReducer, initialState)

  if (module.hot) {
    module.hot.accept('../reducers/root', () => {
      const nextReducer = require('../reducers/root')
      store.replaceReducer(nextReducer)
    })
  }

  return store
}


export default configureStore
