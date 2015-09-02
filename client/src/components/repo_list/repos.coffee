React = require 'react'
PropTypes = React.PropTypes
{div} = React.DOM

RepoList = React.createFactory require './repo_list.coffee'
ReposLoading = React.createFactory require './repos_loading.coffee'

Repos = React.createClass
  render: ->
    {repoActions, username, repo, token} = @props
    {setRepoStatus} = repoActions
    div {},
      if repo.get 'loaded'
        RepoList {
          repos: repo.get 'list'
          setRepoStatus,
          token,
          username
        }
      else
        ReposLoading {}

Repos.propTypes =
  repoActions: PropTypes.object.isRequired
  repo: PropTypes.object.isRequired
  token: PropTypes.string.isRequired
  username: PropTypes.string.isRequired

module.exports = Repos
