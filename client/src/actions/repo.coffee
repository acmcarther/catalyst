{Promise} = require 'es6-promise'

types = require '../constants/action_types.coffee'

RepoActions =
  setRepoStatus: (id, loginToken) ->
    (dispatch) ->
      return unless loginToken?

      # TODO: No dummy
      Promise.resolve({
        type: types.GET_REPOS
        repos: [
          {
            id: 0
            name: 'Test repo 1'
            active: true
          },
          {
            id: 1
            name: 'Test repo 2'
            active: true
          },
          {
            id: 2
            name: 'Test repo 3'
            active: true
          },
        ]
      })
      .then dispatch

  getRepos: (loginToken) ->
    (dispatch) ->
      return unless loginToken?

      # TODO: No dummy
      Promise.resolve({
        type: types.GET_REPOS
        repos: [
          {
            id: 0
            name: 'Test repo 1'
            active: false
          },
          {
            id: 1
            name: 'Test repo 2'
            active: false
          },
          {
            id: 2
            name: 'Test repo 3'
            active: false
          },
        ]
      })
      .then dispatch

module.exports = RepoActions

