{Promise} = require 'es6-promise'

types = require '../constants/action_types.coffee'
mockSettings = require '../mock/repos/settings.coffee'

originalMock = -> [
  {
    id: 0
    name: 'Test repo 1'
    active: true
    settings: mockSettings
  },
  {
    id: 1
    name: 'Test repo 2'
    active: true
    settings: mockSettings
  },
  {
    id: 2
    name: 'Test repo 3'
    active: true
    settings: mockSettings
  },
]

setActivityMock = (id, value) ->
  mock = originalMock()
  mock[id].active = value
  mock

RepoActions =
  setRepoStatus: (id, active, loginToken) ->
    (dispatch) ->
      return unless loginToken?

      # TODO: No dummy
      Promise.resolve({
        type: types.GET_REPOS
        repos: setActivityMock id, active
      })
      .then dispatch

  getRepos: (loginToken) ->
    (dispatch) ->
      return unless loginToken?

      # TODO: No dummy
      Promise.resolve({
        type: types.GET_REPOS
        repos: originalMock()
      })
      .then dispatch

module.exports = RepoActions

