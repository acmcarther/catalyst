import React, { Component, PropTypes } from 'react'


class RepoList extends Component {
  static propTypes = {
    setRepoStatus: PropTypes.func.isRequired,
    goToRepo: PropTypes.func.isRequired,
    username: PropTypes.string.isRequired,
    repos: PropTypes.object.isRequired,
    token: PropTypes.string.isRequired
  }

  render () {
    var { setRepoStatus, goToRepo, username, repos, token } = this.props
    return (
      <div>
        {(() => {
          if (repos.isEmpty()) {
            return (
              <div>
                <div>{ "No repos found for " + username }</div>
                <div>Make sure our bot pt-195 can see your repos!</div>
              </div>
            )
          } else {
            return (
              <div>
                {repos.map((repo, idx) => {
                  var repoActive = repo.get('active')
                  var repoId = repo.get('id')
                  return (
                    <div key={idx}>
                      <span onClick={() => goToRepo(repoId)}>
                        { repo.get('name') }
                      </span>
                      <input
                        type='checkbox'
                        checked={repoActive}
                        onChange={(e) => {
                          setRepoStatus(repoId, e.target.value, token)
                        }} />
                    </div>
                  )
                })}
              </div>
            )
          }
        })()}
      </div>
    )
  }
}

export default RepoList
