import React, { Component, PropTypes } from 'react'
import HeaderBar from './headerbar'
import './header.styl'


class Header extends Component {
  static propTypes = {
    loginActions: PropTypes.object.isRequired,
    pageLocationActions: PropTypes.object.isRequired
  }

  render () {
    var { login, loginActions, pageLocationActions } = this.props
    return (
      <div>
        <div>
          <HeaderBar
            login={login}
            loginActions={loginActions}
            pageLocationActions={pageLocationActions} />
        </div>
        <div className='app-title'>
          <h1>Catalyst</h1>
          <h3>A bot to manage your github build process!</h3>
        </div>
      </div>
    )
  }
}


export default Header
