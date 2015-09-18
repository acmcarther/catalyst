import { Promise } from 'es6-promise'
import Jwt from 'jwt-simple'
import types from '../constants/action_types'
import { getRepos } from './repo'


var LoginActions = {
  logIn (username, password) {
    return (dispatch) => {
      // TODO: No dummy
      var payload = {
        user: 'test-user',
        role: 'user'
      }

      var logInResult = Promise.resolve({
        type: types.LOG_IN,
        token: Jwt.encode(payload, 'dummy-secret', 'HS512'),
        username: username
      })

      logInResult.then(dispatch)
      return logInResult.then(({ token }) => {
        setTimeout(
          (() => getRepos(token)(dispatch)), 5000
        )
      })
    }
  },

  logOut () { return { type: types.LOG_OUT } }
}


export default LoginActions
