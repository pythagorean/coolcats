const path = require('path');

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
  mode: 'none'
}
