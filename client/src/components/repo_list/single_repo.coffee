React = require 'react'
PropTypes = React.PropTypes
{div} = React.DOM

SingleRepo = React.createClass
  render: ->
    {repoActions, pageLocation, repo} = @props
    repoId = pageLocation.getIn ['meta', 'repoId']
    selectedRepo = repo
      .get 'list'
      .filter (entry) -> entry.get('id') is repoId
      .first()
    div {},
      div {}, selectedRepo.get 'id'
      div {}, selectedRepo.get 'name'


SingleRepo.propTypes =
  repoActions: PropTypes.object.isRequired
  pageLocation: PropTypes.object.isRequired
  repo: PropTypes.object.isRequired

module.exports = SingleRepo
