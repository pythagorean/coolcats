const path = require('path');

module.exports = {
  entry: './src/runtime.js',
  output: {
    filename: 'runtime.js',
    path: path.resolve(__dirname, 'target'),
  },
  mode: 'none',
}
