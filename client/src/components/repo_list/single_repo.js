import { Component, PropTypes } from 'react'


class SingleRepo extends Component {
  static propTypes = {
    repoActions: PropTypes.object.isRequired,
    pageLocation: PropTypes.object.isRequired,
    repo: PropTypes.object.isRequired,
    loginToken: PropTypes.object.isRequired
  }

  render () {
    var { repoActions, pageLocation, repo } = this.props
    var repoId = pageLocation.getIn(['meta', 'repoId'])
    var selectedRepo = repo
      .get('list')
      .filter((entry) => entry.get('id') === repoId)
      .first()

    return (
      <div>
        <div>{ selectedRepo.get('id') }</div>
        <div>{ selectedRepo.get('name') }</div>
        <div>
          <div>"Configuration:"</div>
          <div>
            <span>"Recommended Reviewers: "</span>
            <input
              type='checkbox'
              value={selectedRepo.getIn(['settings', 'recommendReviewers'])}
              onChange={(e) => {
                repoActions.setRecommendReviewers(e.target.value)
              }} />
          </div>
          <div>
            <span>"Lint Watch"</span>
            <input
              type='checkbox'
              value={selectedRepo.getIn(['settings', 'lintWatch'])}
              onChange={(e) => {
                repoActions.setLintWatch(e.target.value)
              }} />
          </div>
          <div>
            <span>"Automated Merging"</span>
            <input
              type='checkbox'
              value={selectedRepo.getIn(['settings', 'automatedMerging'])}
              onChange={(e) => {
                repoActions.setAutomatedMerging(e.target.value)
              }} />
          </div>
        </div>
      </div>
    )
  }
}


export default SingleRepo
