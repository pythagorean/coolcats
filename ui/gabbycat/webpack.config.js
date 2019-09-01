const {
  createConfig,
  entryPoint,
  resolve,
  setOutput,
  addPlugins,
  env,
  devServer
} = require('webpack-blocks');

const {
  CleanWebpackPlugin
} = require('clean-webpack-plugin');
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const HtmlBeautifyPlugin = require('html-beautify-webpack-plugin');

const path = require('path');
const distPath = path.resolve(__dirname, "target/deploy");

module.exports = createConfig([
  entryPoint('./bootstrap.js'),
  resolve({
    extensions: ['.wasm']
  }),
  setOutput({
    path: distPath,
    filename: 'gabbycat.js',
    webassemblyModuleFilename: 'gabbycat.wasm'
  }),
  addPlugins([
    new CleanWebpackPlugin({
      verbose: true,
      cleanOnceBeforeBuildPatterns: [distPath],
    }),
    new WasmPackPlugin({
      crateDirectory: '.',
      extraArgs: '--no-typescript'
    }),
    new HtmlWebpackPlugin({
      inject: false,
      template: require('html-webpack-template'),
      filename: 'index.html',
      title: 'gabbycat',
      appMountIds: ['application'],
      scripts: ['gabbycat.js'],
      chunks: []
    }),
    new HtmlBeautifyPlugin({
      config: {
        html: {
          indent_size: 2,
          end_with_newline: true,
        }
      }
    })
  ]),
  env('development', [
    devServer({
      contentBase: distPath,
      host: '0.0.0.0',
      port: 3000
    })
  ])
])
