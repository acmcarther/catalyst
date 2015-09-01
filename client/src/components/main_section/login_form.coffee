React = require 'react'
PropTypes = React.PropTypes
{div, button, label, input, span} = React.DOM

isEmpty = (string) -> string is '' or not string?

LoginForm = React.createClass
  getInitialState: ->
    username: ''
    password: ''

  onSubmitClick: ->
    return if isEmpty(@state.username) or isEmpty(@state.password)
    loginActions.logIn @state.username, @state.password
    @setState username: '', password: ''

  render: ->
    {loginActions} = @props
    div {},
      div {}, 'Log In'
      span {},
        label {}, 'Username:'
        input
          type: 'text'
          placeholder: 'Username'
          onBlur: (e) => @setState username: e.target.value
      span {},
        label {}, 'Password:'
        input
          type: 'password'
          placeholder: 'Password'
          onBlur: (e) => @setState password: e.target.value

      button
        onClick: => @onSubmitClick()
        'Submit'

LoginForm.propTypes = loginActions: PropTypes.object.isRequired

module.exports = LoginForm
