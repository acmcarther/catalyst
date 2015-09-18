import Immutable from 'immutable'
import Jwt from 'jwt-simple'
import { GET_REPOS, LOG_OUT } from '../constants/action_types'


const initialState = Immutable.fromJS({
  list: [],
  loaded: false
})

var repo = (state = initialState, action) => {
  switch (action.type) {
    case GET_REPOS:
      return Immutable.fromJS({
        list: action.repos,
        loaded: true
      })
    case LOG_OUT:
      return Immutable.fromJS({
        list: [],
        loaded: false
      })
    default:
      return state
  }
}


export default repo
