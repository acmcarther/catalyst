import {combineReducers} from 'redux'
import login from './login'
import pageLocation from './page_location'
import repo from './repo'


var rootReducer = combineReducers({
  login,
  pageLocation,
  repo
})


export default rootReducer
