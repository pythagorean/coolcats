// Any imports and Typescript initialization code can go here

require('./application/resources/styles/application.scss');

// Polyfill needed for Edge browser:
if (!window['TextDecoder']) {
  window['TextDecoder'] = require('text-encoder-lite').TextDecoderLite;
}
