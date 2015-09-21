import React, { Component, DOM, PropTypes } from 'react'
import { bindActionCreators } from 'redux'
import { connect } from 'react-redux'

import Header from '../components/header/header'
import Footer from '../components/footer/footer'
import MainSection from '../components/main_section/main_section'
import LoginActions from '../actions/login'
import PageLocationActions from '../actions/page_location'
import RepoActions from '../actions/repo'
import RegisterActions from '../actions/register'


class App extends Component {
  static propTypes = {
    dispatch: PropTypes.func.isRequired
  }

  render () {
    var { repo, login, pageLocation, dispatch } = this.props
    var loginActions = bindActionCreators(LoginActions, dispatch)
    var pageLocationActions = bindActionCreators(PageLocationActions, dispatch)
    var repoActions = bindActionCreators(RepoActions, dispatch)
    var registerActions = bindActionCreators(RegisterActions, dispatch)

    return (
      <div>
        <Header
          login={login}
          loginActions={loginActions}
          pageLocationActions={pageLocationActions} />

        <MainSection
          login={login}
          repo={repo}
          repoActions={repoActions}
          pageLocation={pageLocation}
          loginActions={loginActions}
          pageLocationActions={pageLocationActions}
          registerActions={registerActions} />
      </div>
    )
  }
}

var mapStateToProps = (state) => {
  return {
    login: state.login,
    pageLocation: state.pageLocation,
    repo: state.repo
  }
}


export default connect(mapStateToProps)(App)
