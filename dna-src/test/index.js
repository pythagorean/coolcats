const { Config, Conductor, DnaInstance } = require('@holochain/holochain-nodejs')

const test = require('tape');
const tapSpec = require('tap-spec');

test.createStream()
  .pipe(tapSpec())
  .pipe(process.stdout);

const dnaPath = "./dist/bundle.json"
const aliceName = "alice"
const bobName = "bob"

const dna = Config.dna(dnaPath)
const agentAlice = Config.agent(aliceName)
const instanceAlice = Config.instance(agentAlice, dna)
const conductorAlice = Config.conductor([instanceAlice])

function display(result) {
  console.dir(result, {depth: null, colors: true})
  return result
}

function sleep(milliseconds) {
  var start = new Date().getTime();
  for (var i = 0; i < 1e7; i++) {
    if ((new Date().getTime() - start) > milliseconds){
      break;
    }
  }
}

Conductor.run(conductorAlice, (stop, conductor) => {
  alice = new DnaInstance(aliceName, conductor)
  call = (method, params) => alice.call("coolcats", method, params);
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

  test('clutter', (t) => {
    t.test('get the agent address', (t) => {
      t.plan(1)
      const result = display(call("app_property", {key: "Agent_Address"}))
      t.equal(result.value, "alice-----------------------------------------------------------------------------AAAIuDJb4M")
    })

    t.test('get the agent handle which is not set', (t) => {
      t.plan(1)
      const result = display(call("app_property", {key: "Agent_Handle"}))
      t.equal(result.error.ValidationFailed, "handle_not_found")
    })

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

    t.test('get the first name of the user which is not set', (t) => {
      t.plan(1)
      const result = display(call("get_first_name", {}))
      t.equal(result.error.ValidationFailed, "unlinked_tag: first_name")
    })

    t.test('set the first name of the user', (t) => {
      t.plan(1)
      const result = display(call("set_first_name",
        {name: aliceName}
      ))
      t.equal(result.value, aliceName)
      sleep(1000)
    })

    t.test('get the first name of the user', (t) => {
      t.plan(1)
      const result = display(call("get_first_name", {}))
      t.equal(result.value, aliceName)
    })

    t.test('reset the first name of the user', (t) => {
      t.plan(1)
      const result = display(call("set_first_name",
        {name: bobName}
      ))
      t.equal(result.value, bobName)
      sleep(1000)
    })

    t.test('get the new first name of the user', (t) => {
      t.plan(1)
      const result = display(call("get_first_name", {}))
      t.equal(result.value, bobName)
    })

    t.test('get the profile pic of the user which is not set', (t) => {
      t.plan(1)
      const result = display(call("get_profile_pic", {}))
      t.equal(result.error.ValidationFailed, "unlinked_tag: profile_pic")
    })

    t.test('set the profile_pic of the user', (t) => {
      t.plan(1)
      const result = display(call("set_profile_pic",
        {dataurl: "random stuff for now"}
      ))
      t.equal(result.value, "random stuff for now")
      sleep(1000)
    })

    t.test('get the profile pic of the user', (t) => {
      t.plan(1)
      const result = display(call("get_profile_pic", {}))
      t.equal(result.value, "random stuff for now")
    })

    t.test('reset the profile_pic of the user', (t) => {
      t.plan(1)
      const result = display(call("set_profile_pic",
        {dataurl: "random other stuff"}
      ))
      t.equal(result.value, "random other stuff")
      sleep(1000)
    })

    t.test('get the new profile pic of the user', (t) => {
      t.plan(1)
      const result = display(call("get_profile_pic", {}))
      t.equal(result.value, "random other stuff")
    })

    t.test('make a post', (t) => {
      t.plan(1)
      const result = display(call("post",
        {message: "This is a test message", stamp: "12345"}
      ))
      t.equal(result.value, "Qmef7MUqr5ecqZQGDZYAek2gJwiQNyZkAxPv3hNwGidE76")

      stop()
    })

    t.end()
  })
})
