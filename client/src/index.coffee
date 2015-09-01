React = require 'react'

Root = React.createFactory require './containers/root.coffee'

React.render (Root {}), document.body
