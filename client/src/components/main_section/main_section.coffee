React = require 'react'
PropTypes = React.PropTypes
require './main_section.styl'

{div} = React.DOM
LoginForm = React.createFactory require './login_form.coffee'
RegisterForm = React.createFactory require './registration_form.coffee'
Home = React.createFactory require './home.coffee'
Help = React.createFactory require './help.coffee'
Footer = React.createFactory require '../footer/footer.coffee'
Repos = React.createFactory require '../repo_list/repos.coffee'
SingleRepo = React.createFactory require '../repo_list/single_repo.coffee'

MainSection = React.createClass
  render: ->
    {repo, repoActions, login, pageLocation, pageLocationActions,loginActions, registerActions} = @props
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
                pageLocationActions
              }
            else
              Home {}
          when 'help' then Help {}
          when 'login' then LoginForm { loginActions }
          when 'registration' then RegisterForm { registerActions }
          when 'repo' then SingleRepo {
            token: login.get 'token'
            repo,
            repoActions,
            pageLocation
          }
      div {}, Footer {}

MainSection.propTypes =
  repo: PropTypes.object.isRequired
  repoActions: PropTypes.object.isRequired
  login: PropTypes.object.isRequired
  loginActions: PropTypes.object.isRequired
  pageLocation: PropTypes.object.isRequired
  pageLocationActions: PropTypes.object.isRequired
  registerActions: PropTypes.object.isRequired

module.exports = MainSection
