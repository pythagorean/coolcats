const { Config, Conductor, DnaInstance } = require('@holochain/holochain-nodejs')

const test = require('tape')
const tapSpec = require('tap-spec')

test.createStream()
  .pipe(tapSpec())
  .pipe(process.stdout)

const runtests = ["anchors", "properties", "handles", "posts", "profile"]

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

runtests.includes('anchors') && test('anchors', (t) => {
  Conductor.run(conductorAlice, (stop, conductor) => {
    alice = new DnaInstance(aliceName, conductor)
    call = (method, params) => alice.call("coolcats", method, params);

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

      stop()
    })
  })
  t.end()
})

runtests.includes('properties') && test('properties', (t) => {
  Conductor.run(conductorAlice, (stop, conductor) => {
    alice = new DnaInstance(aliceName, conductor)
    call = (method, params) => alice.call("coolcats", method, params);

    t.test('test for unset agent handle', (t) => {
      t.plan(1)
      const result = display(call("app_property", {key: "Agent_Handle"}))
      t.equal(result.value, undefined)
    })

    t.test('we can create a new handle', (t) => {
      t.plan(1)
      const result = display(call("use_handle", {handle: "buffaloBill"}))
      t.equal(result.value, "QmUXkCgPqXcniV2JvRLeNZs21j4UyXoPWJ4pMtygRCdo8c")
      sleep(1000)
    })

    t.test('test for now set agent handle', (t) => {
      t.plan(1)
      const result = display(call("app_property", {key: "Agent_Handle"}))
      t.equal(result.value, "buffaloBill")
    })

    t.test('we can obtain the dna address', (t) => {
      t.plan(1)
      const result = display(call("app_property", {key: "DNA_Address"}))
      t.equal(result.error, undefined)
    })

    t.test('we can obtain the agent address', (t) => {
      t.plan(1)
      const result = display(call("app_property", {key: "Agent_Address"}))
      t.equal(result.value, "alice-----------------------------------------------------------------------------AAAIuDJb4M")
    })

    t.test('test requesting invalid app property', (t) => {
      t.plan(1)
      const result = display(call("app_property", {key: "garbage"}))
      t.equal(result.error.ValidationFailed, "No App Property with key: garbage")

      stop()
    })
    // No tests on get_property until supported by Holochain Rust
  })
  t.end()
})

runtests.includes('handles') && test('handles', (t) => {
  Conductor.run(conductorAlice, (stop, conductor) => {
    alice = new DnaInstance(aliceName, conductor)
    call = (method, params) => alice.call("coolcats", method, params);

    t.test('test that handle is not set', (t) => {
      t.plan(1)
      const result = display(call("get_handle",
        {address: "QmUXkCgPqXcniV2JvRLeNZs21j4UyXoPWJ4pMtygRCdo8c"}
      ))
      t.equal(result.value, undefined)
    })

    t.test("we can create a new handle", (t) => {
      t.plan(1)
      const result = display(call("use_handle", {handle: "buffaloBill"}))
      t.equal(result.value, "QmUXkCgPqXcniV2JvRLeNZs21j4UyXoPWJ4pMtygRCdo8c")
      sleep(1000)
    })

    t.test('we can retrieve the new handle', (t) => {
      t.plan(1)
      const result = display(call("get_handle",
        {address: "QmUXkCgPqXcniV2JvRLeNZs21j4UyXoPWJ4pMtygRCdo8c"}
      ))
      t.equal(result.value, "buffaloBill")
    })

    t.test("we can update our handle to a unique handle", (t) => {
      t.plan(1)
      const result = display(call("use_handle", {handle: "phil"}))
      t.equal(result.value, "QmZeUu4dzkJpcZLbbn4pTN8n39CZncmQoRAWKjCuKYazN2")
      sleep(1000)
    })

    t.test("trying to use a handle already in use returns error", (t) => {
      t.plan(1)
      const result = display(call("use_handle", {handle: "phil"}))
      t.equal(result.error.ValidationFailed, "handle_in_use")
    })

    t.test("we can retrieve list of handles, in single node mode there will be only one", (t) => {
      t.plan(1)
      const result = display(call("get_handles", {}))
      t.deepEqual(result.value,
        [{handle: "phil", address: "QmZeUu4dzkJpcZLbbn4pTN8n39CZncmQoRAWKjCuKYazN2"}]
      )

      stop()
      // We should add agent tests once we can perform them
    })
  })
  t.end()
})

runtests.includes('posts') && test('posts', (t) => {
  Conductor.run(conductorAlice, (stop, conductor) => {
    alice = new DnaInstance(aliceName, conductor)
    call = (method, params) => alice.call("coolcats", method, params);

    t.test("setup handle for posting", (t) => {
      t.plan(1)
      const result = display(call("use_handle", {handle: "buffaloBill"}))
      t.equal(result.value, "QmUXkCgPqXcniV2JvRLeNZs21j4UyXoPWJ4pMtygRCdo8c")
      sleep(1000)
    })

    t.test('getting non-existent posts returns empty list', (t) => {
      t.plan(1)
      const result = display(call("get_posts_by", {user_handle: "buffaloBill"}))
      t.deepEqual(result.value, [])
    })

    t.test('post must have non-zero length', (t) => {
      t.plan(1)
      const result = display(call("post",
        {message: "", stamp: "12345"}
      ))
      t.equal(result.value, undefined)
    })

    t.test('we can create a new post', (t) => {
      t.plan(1)
      const result = display(call("post",
        {message: "This is a test post", stamp: "12345"}
      ))
      t.equal(result.value, "QmWZZxnYwVuBBShQSqK7E8TTjix8bKMaA1nKkiyFhbfxHv")
      sleep(1000)
    })

    t.test('we can retrieve posts', (t) => {
      t.plan(1)
      const result = display(call("get_posts_by", {user_handle: "buffaloBill"}))
      t.deepEqual(result.value, [{
        address: "QmWZZxnYwVuBBShQSqK7E8TTjix8bKMaA1nKkiyFhbfxHv",
        post: {message: "This is a test post", stamp: "12345"}
      }])
    })

    t.test('we can retrieve a single post', (t) => {
      t.plan(1)
      const result = display(call("get_post",
        {address: "QmWZZxnYwVuBBShQSqK7E8TTjix8bKMaA1nKkiyFhbfxHv"}
      ))
      t.deepEqual(result.value,
        {message: "This is a test post", stamp: "12345"}
      )
    })

    t.test('retrieving single post will fail if not found', (t) => {
      t.plan(1)
      const result = display(call("get_post",
        {address: "QmWZZxnYwVuBBShQSqK7E8TTjix8bKMaA1nKkiyFhbfbad"}
      ))
      t.equal(result.value, undefined)

      stop()
      // We can consider supporting post modifications later if desirable
    })
  })
  t.end()
})

runtests.includes('profile') && test('profile', (t) => {
  Conductor.run(conductorAlice, (stop, conductor) => {
    alice = new DnaInstance(aliceName, conductor)
    call = (method, params) => alice.call("coolcats", method, params);

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

      stop()
    })
  })
  t.end()
})
