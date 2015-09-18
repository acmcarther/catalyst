import React, { Component, PropTypes } from 'react'


var exists = (value) => !(value === null || value === undefined)
var isEmpty = (string) => string === '' || !exists(string)

class LoginForm extends Component {
  static propType = {
    loginActions: PropTypes.object.isRequired
  }

  constructor (props, context) {
    super(props, context)
    this.state = {
      username: '',
      password: ''
    }
    this.onSubmitClick.bind(this)
  }

  onSubmitClick () {
    var { username, password } = this.state
    if (isEmpty(username) || isEmpty(password)) { return }

    this.props.loginActions.logIn(username, password)

    this.setState({
      username: '',
      password: ''
    })
  }

  render () {
    var { loginActions } = this.props
    return (
      <div>
        <div>Log In</div>
        <span>
          <label>Username:</label>
          <input
            type='text'
            placeholder='Username'
            onBlur={(e) => this.setState({ username: e.target.value })}/>
        </span>
        <span>
          <label>Password:</label>
          <input
            type='text'
            placeholder='Password'
            onBlur={(e) => this.setState({ password: e.target.value })}/>
        </span>
        <button
          onClick={() => this.onSubmitClick(loginActions.login)}>
          Submit
        </button>
      </div>
    )
  }
}


export default LoginForm
