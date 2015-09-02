React = require 'react'
PropTypes = React.PropTypes
{div, span, input} = React.DOM

RepoList = React.createClass
  render: ->
    {setRepoStatus, username, repos, token} = @props
    div {},
      if repos.isEmpty()
        div {},
          div {}, "No repos found for #{username}"
          div {}, "Make sure our bot pt-195 can see your repos!"
      else
        div {},
          repos.map (repo, idx) ->
            repoActive = repo.get 'active'
            div key: idx,
              span {}, repo.get 'name'
              input
                type: 'checkbox'
                checked: repoActive
                onChange: -> setRepoStatus not repoActive, token

RepoList.propTypes =
  setRepoStatus: PropTypes.func.isRequired
  username: PropTypes.string.isRequired
  repos: PropTypes.object.isRequired
  token: PropTypes.string.isRequired

module.exports = RepoList
