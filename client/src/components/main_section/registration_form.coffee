React = require 'react'
PropTypes = React.PropTypes
{div, button, label, input, span} = React.DOM

isEmpty = (string) -> string is '' or not string?

_ = require 'underscore'

RegisterForm = React.createClass
  getInitialState: ->
    username: ''
    password: ''
    passwordVerification: ''

  onSubmitClick: (register) ->
    return if _.some [@state.username, @state.password, @state.passwordVerification], isEmpty
    return if @state.password isnt @state.passwordVerification
    register @state.username, @state.password
    @setState username: '', password: '', passwordVerification: ''

  render: ->
    {registerActions} = @props
    div {},
      div {}, 'Register'

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

      span {},
        label {}, 'Password Again:'
        input
          type: 'password'
          placeholder: 'Password again'
          onBlur: (e) => @setState passwordVerification: e.target.value

      button
        onClick: => @onSubmitClick registerActions.register
        'Submit'

RegisterForm.propTypes =
  registerActions: PropTypes.object.isRequired

module.exports = RegisterForm
