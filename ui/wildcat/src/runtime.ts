// Any imports and Typescript initialization code can go here

require('./application/resources/styles/application.scss');
require('./application/resources/images/logo.svg');

// Polyfill needed for Edge browser:
if (!window['TextDecoder']) {
  window['TextDecoder'] = require('text-encoder-lite').TextDecoderLite;
}
