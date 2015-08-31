React = require 'react'
{ combineReducers } = require 'redux'
todos = require './todos.coffee'
login = require './login.coffee'

rootReducer = combineReducers {
  todos
  login
}

module.exports = rootReducer
