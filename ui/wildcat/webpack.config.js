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
    filename: 'wildcat-ui.js',
    webassemblyModuleFilename: 'wildcat-ui.wasm'
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
  match(['*.png', '*.svg', '!*-webfont.svg'], [
    file({
      name: '[name].[ext]',
      outputPath: 'images/'
    })
  ]),
  match(['*.eot', '*.ttf', '*.woff', '*.woff2', '*-webfont.svg'], [
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
      title: 'wildcat-ui',
      favicon: 'public/favicon.png',
      appMountIds: ['holoclient', 'application'],
      scripts: ['/wildcat-ui.js'],
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
