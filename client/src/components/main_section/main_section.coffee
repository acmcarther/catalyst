React = require 'react'
PropTypes = React.PropTypes
{ section, ul, div, button, input, span} = React.DOM

require './main_section.styl'
LoginForm = React.createFactory require './login_form.coffee'
Home = React.createFactory require './home.coffee'
Help = React.createFactory require './help.coffee'
Footer = React.createFactory require '../footer/footer.coffee'

MainSection = React.createClass
  render: ->
    { pageLocation, loginActions } = @props
    div {},
      div className: 'main-body',
        switch pageLocation.get 'currentPage'
          when 'home' then Home {}
          when 'help' then Help {}
          when 'login' then LoginForm { loginActions }
      div {}, Footer {}

MainSection.propTypes = loginActions: PropTypes.object.isRequired

module.exports = MainSection
