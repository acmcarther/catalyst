React = require 'react'
{combineReducers} = require 'redux'

login = require './login.coffee'
pageLocation = require './page_location.coffee'
repo = require './repo.coffee'

rootReducer = combineReducers {
  login
  pageLocation
  repo
}

module.exports = rootReducer
