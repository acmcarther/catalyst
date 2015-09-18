import React, { Component, PropTypes } from 'react'
import _ from 'underscore'


var exists = (value) => !(value === null || value === undefined)
var isEmpty = (string) => string === '' || !exists(string)

class RegisterForm extends Component {
  static propTypes = {
    registerActions: PropTypes.object.isRequired
  }

  constructor (props, context) {
    super(props, context)
    this.state = {
      username: '',
      password: '',
      passwordVerification: ''
    }
    this.onSubmitClick.bind(this)
  }

  onSubmitClick (register) {
    var x = [this.state.username, this.state.password, this.state.passwordVerification]
    var y = _.some(x, isEmpty)
    if (y) { return }
    if (this.state.password !== this.state.passwordVerification) { return }
    register(this.state.username, this.state.password)
    this.setState({
      username: '',
      password: '',
      passwordVerification: ''
    })
  }

  render () {
    var { registerActions } = this.props

    return (
      <div>
        <span>
          <label>Username:</label>
          <input
            type='text'
            placeholder='Username'
            onBlur={(e) => this.setState({ username: e.target.value })} />
        </span>
        <span>
          <label>Password:</label>
          <input
            type='password'
            placeholder='Password'
            onBlur={(e) => this.setState({ password: e.target.value })} />
        </span>
        <span>
          <label>Password Again:</label>
          <input
            type='password'
            placeholder='Password again'
            onBlur={(e) => this.setState({ passwordVerification: e.target.value })} />
        </span>
        <button onClick={() => this.onSubmitClick(registerActions.register)}>Submit</button>
      </div>
    )
  }
}


export default RegisterForm
