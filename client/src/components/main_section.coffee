require './main_section.styl'
React = require 'react'

PropTypes = React.PropTypes

{ section, ul, div, button, input, span} = React.DOM

Footer = React.createFactory require './footer/footer.coffee'

MainSection = React.createClass
  getInitialState: ->
    username: ''
    password: ''

  render: ->
    {pageLocation, loginActions} = @props
    pageLocation = pageLocation.get 'currentPage'
    div {},
      div className: 'main-body',
        if pageLocation is 'home'
          div {}, 'Home'
        else if pageLocation is 'login'
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

      div {}, Footer {}



MainSection.propTypes =
  loginActions: PropTypes.object.isRequired

module.exports = MainSection
