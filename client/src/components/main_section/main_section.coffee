React = require 'react'
PropTypes = React.PropTypes
{div} = React.DOM

require './main_section.styl'
LoginForm = React.createFactory require './login_form.coffee'
Home = React.createFactory require './home.coffee'
Help = React.createFactory require './help.coffee'
Footer = React.createFactory require '../footer/footer.coffee'
Repos = React.createFactory require '../repo_list/repos.coffee'

MainSection = React.createClass
  render: ->
    {repo, repoActions, login, pageLocation, loginActions} = @props
    username = login.get 'username'
    div {},
      div className: 'main-body',
        switch pageLocation.get 'currentPage'
          when 'home'
            if username?
              Repos {
                token: login.get 'token'
                repoActions
                repo
                username
              }
            else
              Home {}
          when 'help' then Help {}
          when 'login' then LoginForm { loginActions }
      div {}, Footer {}

MainSection.propTypes =
  loginActions: PropTypes.object.isRequired
  repo: PropTypes.object.isRequired
  repoActions: PropTypes.object.isRequired
  login: PropTypes.object.isRequired
  pageLocation: PropTypes.object.isRequired
  loginActions: PropTypes.object.isRequired

module.exports = MainSection
