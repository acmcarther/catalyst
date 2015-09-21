import React, { Component, PropTypes } from 'react'
import './headerbar.styl'


var exists = (value) => {
  return !(value === undefined || value === null)
}

var Username = ({ username }) => {
  if (!exists(username)) { return null }

  return (
    <span className='header-elem logged-in-as'>
      { "Welcome, " + username }
    </span>
  )
}

var Login = ({ onClick }) => {
  return (
    <span className='header-elem clickable' onClick={onClick}>Login</span>
  )
}

var Logout = ({ onClick }) => {
  return (
    <span className='header-elem clickable' onClick={onClick}>Logout</span>
  )
}

var Register = ({ onClick }) => {
  return (
    <span className='header-elem clickable' onClick={onClick}>Register</span>
  )
}

class HeaderBar extends Component {
  static propTypes = {
    loginActions: PropTypes.object.isRequired,
    pageLocationActions: PropTypes.object.isRequired
  }

  render () {
    var { login, loginActions, pageLocationActions } = this.props
    var username = login.get('username')

    return (
      <div className='top-bar'>
        {Username({ username })}
        <span
          className='header-elem clickable'
          onClick={() => pageLocationActions.goToHome()}>
          Home
        </span>
        <span
          className='header-elem clickable'
          onClick={() => pageLocationActions.goToHelp()}>
          Help
        </span>

        {(() => {
          if (exists(username)) {
            return Logout({ onClick: () => loginActions.logOut() })
          }
        })()}
        
        {(() => {
          if (!exists(username)) {
            return Login({ onClick: () => pageLocationActions.goToLogIn() })
          }
        })()}
        
        {(() => {
          if (!exists(username)) {
            return Register({ onClick: () => pageLocationActions.goToRegistration() })
          }
        })()}
      </div>
    )
  }
}


export default HeaderBar
