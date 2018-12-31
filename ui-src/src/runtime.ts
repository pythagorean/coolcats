// Any imports and Typescript initialization code can go here
declare function require(name: string): string

require('./app.css')

require('./images/favicon.png')
var favicon = document.querySelector('link[rel="shortcut icon"]')
if (!favicon) {
  favicon = document.createElement('link')
  favicon.setAttribute('rel', 'shortcut icon')
  var head = document.querySelector('head')
  head.appendChild(favicon)
}
favicon.setAttribute('type', 'image/png')
favicon.setAttribute('href', 'favicon.png')

require('./images/cat-eating-bird-circle.png')
