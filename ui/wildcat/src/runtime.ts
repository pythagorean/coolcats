// Any imports and Typescript initialization code can go here

require('../node_modules/font-awesome/scss/font-awesome.scss');
require('./application/resources/styles/contrast.scss');
require('./application/resources/images/logo.svg');

// Polyfill needed for Edge browser:
if (!window['TextDecoder']) {
  window['TextDecoder'] = require('text-encoder-lite').TextDecoderLite;
}
