import React, { Component, PropTypes } from 'react'
import LoginForm from './login_form'
import RegisterForm from './registration_form'
import Home from './home'
import Help from './help'
import Footer from '../footer/footer'
import Repos from '../repo_list/repos'
import SingleRepo from '../repo_list/single_repo'
require('./main_section.styl')


var exists = (value) => !(value === null || value === undefined)

class MainSection extends Component {
  static propTypes = {
    repo: PropTypes.object.isRequired,
    repoActions: PropTypes.object.isRequired,
    login: PropTypes.object.isRequired,
    loginActions: PropTypes.object.isRequired,
    pageLocation: PropTypes.object.isRequired,
    pageLocationActions: PropTypes.object.isRequired,
    registerActions: PropTypes.object.isRequired
  }

  render () {
    var {
      repo,
      repoActions,
      login,
      pageLocation,
      pageLocationActions,
      loginActions,
      registerActions
    } = this.props

    var username = login.get('username')

    console.log(pageLocation.get('currentPage'))

    return (
      <div>
        <div className='main-body'>
          {(() => {
            switch (pageLocation.get('currentPage')) {
              case 'home':
                if (exists(username)) {
                  return (
                    <Repos
                      token={login.get('token')}
                      repoActions={repoActions}
                      repo={repo}
                      username={username}
                      pageLocationActions={pageLocationActions}/>
                  )
                } else {
                  return <Home />
                }
              case 'help':
                return <Help />
              case 'login':
                return <LoginForm loginActions={loginActions} />
              case 'registration':
                return <RegisterForm registerActions={registerActions} />
              case 'repo':
                return (
                  <SingleRepo
                    token={login.get('token')}
                    repo={repo}
                    repoActions={repoActions}
                    pageLocation={pageLocation} />
                )
            }
          })()}
        </div>
        <div>
          <Footer />
        </div>
      </div>
    )
  }
}

 export default MainSection
