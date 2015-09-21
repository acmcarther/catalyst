import { Promise } from 'es6-promise'
import types from '../constants/action_types'
import mockSettings from '../mock/repos/settings'


var originalMock = () => [
  {
    id: 0,
    name: 'Test repo 1',
    active: true,
    settings: mockSettings
  },
  {
    id: 1,
    name: 'Test repo 2',
    active: true,
    settings: mockSettings
  },
  {
    id: 2,
    name: 'Test repo 3',
    active: true,
    settings: mockSettings
  }
]

var exists = (value) => value === null || value === undefined

var setActivityMock = (id, value) => {
  var mock = originalMock()
  mock[id].active = value
  return mock
}

var RepoActions = {
  setRepoStatus (id, active, loginToken) {
    return (dispatch) => {
      if (!exists(loginToken)) { return }

      // TODO: No dummy
      return Promise.resolve({
        type: types.GET_REPOS,
        repos: setActivityMock(id, active)
      })
      .then(dispatch)
    }
  },

  getRepos (loginToken) {
    return (dispatch) => {
      if (!exists(loginToken)) { return }

      // TODO: No dummy
      return Promise.resolve({
        type: types.GET_REPOS,
        repos: originalMock()
      })
      .then(dispatch)
    }
  }
}


export default RepoActions
