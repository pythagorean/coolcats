const test = require('tape');
const tapSpec = require('tap-spec');

test.createStream()
  .pipe(tapSpec())
  .pipe(process.stdout);

const { Config, Container } = require("@holochain/holochain-nodejs")

const dnaPath = "./dist/bundle.json"
const aliceName = "alice"

// closure to keep config-only stuff out of test scope
const container = (() => {
    const agentAlice = Config.agent(aliceName)
    const dna = Config.dna(dnaPath)
    const instanceAlice = Config.instance(agentAlice, dna)
    const containerConfig = Config.container([instanceAlice])
    return new Container(containerConfig)
})()

// Initialize the Container
container.start()

const alice = container.makeCaller(aliceName, dnaPath)
const agent_id = container.agent_id(aliceName + '::' + dnaPath)

function display(result) {
  console.dir(result, {depth: null, colors: true})
  return result
}

function call(method, params) {
  return alice.call("coolcats", "main", method, params)
}

function sleep(milliseconds) {
  var start = new Date().getTime();
  for (var i = 0; i < 1e7; i++) {
    if ((new Date().getTime() - start) > milliseconds){
      break;
    }
  }
}

test('anchors', (t) => {
  t.test('create and get anchors', (t) => {
    t.plan(1)
    const addr = call("create_anchor", {anchor:
      {anchor_type: "testing", anchor_text: "1-2-3"}
    })
    const result = display(call("get_anchor", {address: addr.value}))
    t.deepEqual(result.value,
      {anchor_type: "testing", anchor_text: "1-2-3"}
    )
  })

  t.test('check that anchor exists works', (t) => {
    t.plan(2)
    const result1 = display(call("anchor_exists", {anchor:
      {anchor_type: "testing", anchor_text: "1-2-3"}
    }))
    const result2 = display(call("anchor_exists", {anchor:
      {anchor_type: "testing", anchor_text: "3-2-1"}
    }))
    t.equal(result1.value, true)
    t.equal(result2.value, false)
  })

  t.test('get anchors from links', (t) => {
    t.plan(1)
    const result = display(call("get_anchors",
      {anchor_type: "testing"}
    ))
    t.deepEqual(result.value, [
      {anchor_type: "testing", anchor_text: "1-2-3"}
    ])
  })

  t.end()
})

test('properties', (t) => {
  t.test('get the agent address', (t) => {
    t.plan(1)
    const result = display(call("app_property", {key: "Agent_Address"}))
    t.equal(result.value, agent_id)
  })

  // If this lookup is enabled here, getting the agent handle fails later too
  //t.test('get the agent handle which is not set', (t) => {
  //  const result = display(call("app_property", {key: "Agent_Handle"}))
  //  t.end()
  //})

  t.test('set the agent handle', (t) => {
    t.plan(1)
    const result = display(call("use_handle", {handle: "buffaloBill"}))
    t.equal(result.value, "QmUXkCgPqXcniV2JvRLeNZs21j4UyXoPWJ4pMtygRCdo8c")
    sleep(1000)
  })

  t.test('get the agent handle which is now set', (t) => {
    t.plan(1)
    const result = display(call("app_property", {key: "Agent_Handle"}))
    t.equal(result.value, "buffaloBill")
  })

  t.test("trying to use a handle already in use returns error", (t) => {
    t.plan(1)
    const result = display(call("use_handle", {handle: "buffaloBill"}))
    t.equal(result.error.ValidationFailed, "handle_in_use")
  })

  t.test('get the handle by its own address', (t) => {
    t.plan(1)
    const result = display(call("get_handle",
      {address: "QmUXkCgPqXcniV2JvRLeNZs21j4UyXoPWJ4pMtygRCdo8c"}
    ))
    t.equal(result.value, "buffaloBill")
  })

  t.end()
})
