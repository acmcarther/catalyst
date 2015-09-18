React = require 'react'
PropTypes = React.PropTypes
{div} = React.DOM
{bindActionCreators} = require 'redux'
{connect} = require 'react-redux'

Header = React.createFactory require '../components/header/header.coffee'
Footer = React.createFactory require '../components/footer/footer.coffee'
MainSection = React.createFactory require '../components/main_section/main_section.coffee'
LoginActions = require '../actions/login.coffee'
PageLocationActions = require '../actions/page_location.coffee'
RepoActions = require '../actions/repo.coffee'
RegisterActions = require '../actions/register.coffee'

App = React.createClass
  render: ->
    {repo, login, pageLocation, dispatch} = @props
    loginActions = bindActionCreators LoginActions, dispatch
    pageLocationActions = bindActionCreators PageLocationActions, dispatch
    repoActions = bindActionCreators RepoActions, dispatch
    registerActions = bindActionCreators RegisterActions, dispatch

    div {},
      Header {login, loginActions, pageLocationActions}
      MainSection {login, repo, repoActions, pageLocation, loginActions, pageLocationActions, registerActions}

App.propTypes =
  dispatch: PropTypes.func.isRequired

select = (state) ->
  login: state.login
  pageLocation: state.pageLocation
  repo: state.repo

module.exports = connect(select)(App)
