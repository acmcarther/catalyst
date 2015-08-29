var path = require('path');
var webpack = require('webpack');

module.exports = {
  entry: [
    './src/index.coffee'
  ],
  devtool: "eval",
  debug: true,
  output: {
    path: path.join(__dirname, 'dist'),
    publicPath: '/assets/',
    filename: 'bundle.js'
  },
  resolveLoader: {
    modulesDirectories: ['node_modules']
  },
  plugins: [
    new webpack.NoErrorsPlugin(),
    new webpack.IgnorePlugin(/vertx/) // https://github.com/webpack/webpack/issues/353
  ],
  resolve: {
    extensions: ['', '.js', '.coffee', '.styl']
  },
  module: {
    loaders: [
      { test: /\.styl$/, loader: 'style-loader!css-loader!stylus-loader' },
      { test: /\.coffee$/, loaders: ['coffee'] }
    ]
  }
}
