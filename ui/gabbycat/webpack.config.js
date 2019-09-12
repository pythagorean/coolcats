const {
  createConfig,
  entryPoint,
  resolve,
  setOutput,
  typescript,
  match,
  css,
  sass,
  file,
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
const distPath = path.resolve(__dirname, "../target/deploy");

module.exports = createConfig([
  entryPoint(['./bootstrap.js', './src/runtime.ts']),
  resolve({
    extensions: ['.wasm']
  }),
  setOutput({
    path: distPath,
    publicPath: '/',
    filename: 'gabbycat.js',
    webassemblyModuleFilename: 'gabbycat.wasm'
  }),
  typescript(),
  match('*.scss', [
    css({
      options: {
        styleLoader: true
      }
    }),
    sass()
  ]),
  match(['*.png', '*.svg'], [
    file({
      name: '[name].[ext]',
      outputPath: 'images/'
    })
  ]),
  match(['*.eot', '*.ttf', '*.woff', '*.woff2'], [
    file({
      name: '[name].[ext]',
      outputPath: 'fonts/'
    })
  ]),
  addPlugins([
    new CleanWebpackPlugin({
      dry: false,
      verbose: true,
      cleanOnceBeforeBuildPatterns: [distPath],
      dangerouslyAllowCleanPatternsOutsideProject: true,
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
      favicon: 'src/application/resources/images/favicon.ico',
      appMountIds: ['application'],
      scripts: ['/gabbycat.js'],
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
      port: 8000,
      historyApiFallback: true
    })
  ])
])
