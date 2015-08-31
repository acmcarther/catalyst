React = require 'react'
{ combineReducers } = require 'redux'
todos = require './todos.coffee'

rootReducer = combineReducers { todos }

module.exports = rootReducer
