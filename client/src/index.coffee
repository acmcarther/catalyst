React = require 'react'

App = React.createFactory require './core/components/core.coffee'

React.render (App {}) , document.body
