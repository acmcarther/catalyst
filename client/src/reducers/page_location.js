import Immutable from 'immutable'
import {
  GO_TO_LOG_IN,
  LOG_IN, LOG_OUT,
  GO_TO_HOME, GO_TO_HELP,
  GO_TO_REPO,
  GO_TO_REGISTER
} from '../constants/action_types'


const initialState = Immutable.fromJS({
  currentPage: 'home',
  meta: null
})

var pageLocation = (state = initialState, action) => {
  switch (action.type) {
    case GO_TO_REPO:
      return state.merge({
        currentPage: 'repo',
        meta: { repoId: action.repoId }
      })
    case LOG_IN, LOG_OUT, GO_TO_HOME:
      return state.merge({currentPage: 'home'})
    case GO_TO_LOG_IN:
      return state.merge({currentPage: 'login'})
    case GO_TO_HELP:
      return state.merge({currentPage: 'help'})
    case GO_TO_REGISTER:
      return state.merge({currentPage: 'registration'})
    default:
      return state
  }
}


export default pageLocation
