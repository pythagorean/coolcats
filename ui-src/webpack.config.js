const path = require('path');

module.exports = {
  entry: './src/runtime.ts',
  module: {
    rules: [
      {
        test: /\.ts$/,
        use: 'ts-loader',
        exclude: /node_modules/
      }
    ]
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
