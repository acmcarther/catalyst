React = require 'react'
{ combineReducers } = require 'redux'
login = require './login.coffee'
pageLocation = require './page_location.coffee'

rootReducer = combineReducers {
  login
  pageLocation
}

module.exports = rootReducer
