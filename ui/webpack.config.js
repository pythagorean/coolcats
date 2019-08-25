const {
  createConfig,
  entryPoint,
  setOutput,
  resolve,
  match,
  typescript,
  css,
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
const distPath = path.resolve(__dirname, "target/deploy");

module.exports = createConfig([
  entryPoint(['./bootstrap.js', './src/runtime.ts']),
  setOutput({
    path: distPath,
    filename: 'coolcats-ui.js',
    webassemblyModuleFilename: 'coolcats-ui.wasm'
  }),
  resolve({
    extensions: ['.wasm']
  }),
  typescript(),
  css({
    options: {
      styleLoader: true
    }
  }),
  match(['*.png', '*.jpg', '*.gif'], [
    file({
      name: '[name].[ext]',
    })
  ]),
  addPlugins([
    new CleanWebpackPlugin({
      verbose: true,
      cleanOnceBeforeBuildPatterns: [distPath],
    }),
    new WasmPackPlugin({
      crateDirectory: ".",
      extraArgs: "--no-typescript",
    }),
    new HtmlWebpackPlugin({
      inject: false,
      template: require('html-webpack-template'),
      filename: 'index.html',
      title: 'coolcats-ui',
      meta: [{
        name: 'viewport',
        content: 'width=device-width, initial-scale=1, shrink-to-fit=no',
      }],
      links: [{
        rel: 'stylesheet',
        href: 'https://maxcdn.bootstrapcdn.com/bootstrap/3.3.7/css/bootstrap.min.css',
        integrity: 'sha384-BVYiiSIFeK1dGmJRAkycuHAHRg32OmUcww7on3RYdg4Va+PmSTsz/K68vbdEjh4u',
        crossorigin: 'anonymous',
      }],
      favicon: 'src/application/interfaces/images/favicon.png',
      appMountIds: ['holoclient', 'application'],
      scripts: ['coolcats-ui.js'],
      chunks: [],
    }),
    new HtmlBeautifyPlugin({
      config: {
        html: {
          indent_size: 2,
          end_with_newline: true,
        },
      },
    }),
  ]),
  env('development', [
    devServer({
      contentBase: distPath,
      port: 8000
    })
  ])
])
