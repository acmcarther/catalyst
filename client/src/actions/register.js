import { Promise } from 'es6-promise'
import Jwt from 'jwt-simple'
import types from '../constants/action_types'
import { getRepos } from './repo'


var RegisterActions = {
  register (username, password) {
    return (dispatch) => {
      var payload = {
        user: 'test-user',
        role: 'user'
      }

      var registerResult = Promise.resolve({
        type: types.REGISTER_SUCCESS,
        token: Jwt.encode(payload, 'dummy-secret', 'HS512'),
        username: username
      })

      registerResult.then(dispatch)
      registerResult.then(({ token }) => {
        setTimeout((() => getRepos(token)(dispatch)), 5000)
      })
    }
  }
}


module.exports = RegisterActions
