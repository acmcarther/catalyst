var path = require('path');
var webpack = require('webpack');

module.exports = {
  entry: [
    'webpack-dev-server/client?http://0.0.0.0:3000', // WebpackDevServer host and port
    'webpack/hot/only-dev-server', // "only" prevents reload on syntax errors
    './src/index.js'
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
    new webpack.HotModuleReplacementPlugin(),
    new webpack.IgnorePlugin(/vertx/), // https://github.com/webpack/webpack/issues/353
    new webpack.DefinePlugin({
      __DEVTOOLS__: true
    })
  ],
  resolve: {
    extensions: ['', '.js', '.styl']
  },
  module: {
    loaders: [
      { test: /\.styl$/, loader: 'style-loader!css-loader!stylus-loader' },
      {
        test: /\.js/,
        loaders: [/*'react-hot', */'babel-loader'],
        include: path.join(__dirname, 'src'),
        exclude: /node_modules/
      }
    ]
  }
}
