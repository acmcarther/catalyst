React = require 'react'
PropTypes = React.PropTypes
{div, span, input} = React.DOM

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
      div {},
        div {}, 'Configuration:'
        div {},
          span {}, 'Recommened Reviewers: '
          input
            type: 'checkbox'
            value: selectedRepo.getIn ['settings', 'recommendReviewers']
            onChange: (e) -> repoActions.setRecommendReviewers e.target.value
        div {},
          span {}, 'Lint Watch'
          input
            type: 'checkbox'
            value: selectedRepo.getIn ['settings', 'lintWatch']
            onChange: (e) -> repoActions.setLintWatch e.target.value
        div {},
          span {}, 'Automated Merging'
          input
            type: 'checkbox'
            value: selectedRepo.getIn ['settings', 'automatedMerging']
            onChange: (e) -> repoActions.setAutomatedMerging e.target.value

SingleRepo.propTypes =
  repoActions: PropTypes.object.isRequired
  pageLocation: PropTypes.object.isRequired
  repo: PropTypes.object.isRequired
  loginToken: PropTypes.object.isRequired

module.exports = SingleRepo
