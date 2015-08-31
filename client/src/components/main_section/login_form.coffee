React = require 'react'
PropTypes = React.PropTypes

{ section, ul, div, button, input, span} = React.DOM

LoginForm = React.createClass
  getInitialState: ->
    username: ''
    password: ''

  render: ->
    { loginActions } = @props
    div {},
      div {}, 'Log In'
      span {},
        span {}, 'Username:'
        input
          type: 'text'
          placeholder: 'Username'
          onBlur: (e) => @setState username: e.target.value
      span {},
        span {}, 'Password:'
        input
          type: 'password'
          placeholder: 'Password'
          onBlur: (e) => @setState password: e.target.value

      button
        onClick: =>
          return unless @state.username isnt '' and @state.password isnt ''
          loginActions.logIn @state.username, @state.password
          @setState username: '', password: ''

        'Submit'

LoginForm.propTypes = loginActions: PropTypes.object.isRequired

module.exports = LoginForm
