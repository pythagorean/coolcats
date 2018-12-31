const path = require('path');
const CleanWebpackPlugin = require('clean-webpack-plugin');
const HtmlWebpackPlugin = require('html-webpack-plugin');

module.exports = {
  entry: './src/runtime.ts',
  module: {
    rules: [
      {
        test: /\.ts$/,
        use: 'ts-loader',
        exclude: /node_modules/
      },
      {
        test: /\.css$/,
        use: [ 'style-loader', 'css-loader' ]
      },
      {
        test: /\.(png|jpg|gif)$/,
        use: [
          {
            loader: 'file-loader',
            options: {
              name: '[name].[ext]',
              outputPath: '../static',
              publicPath: './',
            },
          },
        ],
      },
    ],
  },
  resolve: {
    extensions: [ '.ts', '.js' ]
  },
  output: {
    filename: 'runtime.js',
    path: path.resolve(__dirname, 'target')
  },
  plugins: [
    new CleanWebpackPlugin([ 'static' ]),
    new HtmlWebpackPlugin({
      inject: false,
      template: require('html-webpack-template'),
      filename: '../static/index.html',
      title: 'Coolcats2',
      meta: [{
        name: 'viewport',
        content: 'width=device-width, initial-scale=1, shrink-to-fit=no',
      }],
      links: [{
        rel: 'stylesheet',
        href: 'https://stackpath.bootstrapcdn.com/bootstrap/4.2.1/css/bootstrap.min.css',
        integrity: 'sha384-GJzZqFGwb1QTTN6wy59ffF1BuGJpLSa9DkKMp0DgiMDm4iYMj70gZWKYbI706tWS',
        crossorigin: 'anonymous',
      }],
      favicon: 'src/app/images/favicon.png',
      appMountIds: [ 'holoclient', 'application' ],
      scripts: [ 'coolcats2.js' ],
      chunks: [ ],
    }),
  ],
  mode: 'none',
}
