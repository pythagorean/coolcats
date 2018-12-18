import { Client } from 'rpc-websockets'
import { holochainMiddleware } from '@holochain/hc-redux-middleware'

// borrowed from https://github.com/holochain/holochain-ui/blob/develop/ui-src/src/utils/hc-web-client.ts
const connect = (url: string): any => new Promise((fulfill, reject) => {
  const ws = new Client(url)
  ws.on('open', () => {
    const call = (...segments: Array<string>) => (params: any) => {
      const method = segments.length === 1 ? segments[0] : segments.join('/')
      return ws.call(method, params)
    }
    const close = () => ws.close()
    fulfill({ call, close, ws })
  })
})

// export to redux.rs
declare var hcMw: any
hcMw = holochainMiddleware(connect('ws://localhost:8888'))
