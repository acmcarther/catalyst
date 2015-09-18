import { Component, PropTypes } from 'react'
import RepoList from './repo_list'
import ReposLoading from './repos_loading'


class Repos extends Component {
  static propTypes = {
    repoActions: PropTypes.object.isRequired,
    pageLocationActions: PropTypes.object.isRequired,
    repo: PropTypes.object.isRequired,
    token: PropTypes.string.isRequired,
    username: PropTypes.string.isRequired
  }

  render () {
    var { repoActions, pageLocationActions, username, repo, token } = this.props
    var { setRepoStatus } = repoActions
    var { goToRepo } = pageLocationActions

    return (
      <div>
        {(() => {
          if (repo.get('loaded')) {
            return (
              <RepoList
                repos={repo.get('list')}
                setRepoStatus={setRepoStatus}
                token={token}
                username={username}
                goToRepo={goToRepo} />
            )
          } else {
            return <ReposLoading />
          }
        })()}
      </div>
    )
  }
}


export default Repos
