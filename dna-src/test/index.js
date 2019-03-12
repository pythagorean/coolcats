const {
  Config,
  Conductor,
  DnaInstance,
  Scenario
} = require('@holochain/holochain-nodejs')

const test = require('tape')
const tapSpec = require('tap-spec')

test.createStream()
  .pipe(tapSpec())
  .pipe(process.stdout)

var runtests = [
  "anchors", "properties", "handles", "posts", "hashtags", "favourites",
  "profile", "collisions", "follows"
]
if (process.env.RUNTEST) {
  runtests = [process.env.RUNTEST]
}

const dnaPath = "./dist/dna-src.dna.json"
const dna = Config.dna(dnaPath)

const aliceName = "alice"
const agentAlice = Config.agent(aliceName)
const instanceAlice = Config.instance(agentAlice, dna)
const conductorAlice = Config.conductor([instanceAlice])

const bobName = "bob"
const agentBob = Config.agent(bobName)
const instanceBob = Config.instance(agentBob, dna)

const carolName = "carol"
const agentCarol = Config.agent(carolName)
const instanceCarol = Config.instance(agentCarol, dna)

const scenario1 = new Scenario([instanceAlice])
const scenario2 = new Scenario([instanceAlice, instanceBob])
const scenario3 = new Scenario([instanceAlice, instanceBob, instanceCarol])

const colors = require('colors')

function underline(text) {
  console.log(text.underline)
}

function display(result) {
  console.dir(result, {
    depth: null,
    colors: true
  })
  return result
}

function sleep(milliseconds) {
  var start = new Date().getTime();
  for (var i = 0; i < 1e7; i++) {
    if ((new Date().getTime() - start) > milliseconds) {
      break;
    }
  }
}

runtests.includes('anchors') && test('anchors', (t) => {
  Conductor.run(conductorAlice, (stop, conductor) => {
    alice = new DnaInstance(aliceName, conductor)
    call = (method, params) => alice.call("coolcats", method, params)

    t.test('create and get anchors', (t) => {
      t.plan(1)
      const addr = call("create_anchor", {
        anchor: {
          anchor_type: "testing",
          anchor_text: "1-2-3"
        }
      })
      const result = display(call("get_anchor", {
        address: addr.value
      }))
      t.deepEqual(result.value, {
        anchor_type: "testing",
        anchor_text: "1-2-3"
      })
    })

    t.test('check that anchor exists works', (t) => {
      t.plan(2)
      const result1 = display(call("anchor_exists", {
        anchor: {
          anchor_type: "testing",
          anchor_text: "1-2-3"
        }
      }))
      const result2 = display(call("anchor_exists", {
        anchor: {
          anchor_type: "testing",
          anchor_text: "3-2-1"
        }
      }))
      t.equal(result1.value, true)
      t.equal(result2.value, false)
    })

    t.test('get anchors from links', (t) => {
      t.plan(1)
      const result = display(call("get_anchors", {
        anchor_type: "testing"
      }))
      t.deepEqual(result.value, [{
        anchor_type: "testing",
        anchor_text: "1-2-3"
      }])

      stop()
    })
  })
  t.end()
})

runtests.includes('properties') && test('properties', (t) => {
  Conductor.run(conductorAlice, (stop, conductor) => {
    alice = new DnaInstance(aliceName, conductor)
    call = (method, params) => alice.call("coolcats", method, params)

    t.test('test for unset agent handle', (t) => {
      t.plan(1)
      const result = display(call("app_property", {
        key: "Agent_Handle"
      }))
      t.equal(result.value, undefined)
    })

    t.test('we can create a new handle', (t) => {
      t.plan(1)
      const result = display(call("use_handle", {
        handle: "buffaloBill"
      }))
      t.equal(result.value, "QmUXkCgPqXcniV2JvRLeNZs21j4UyXoPWJ4pMtygRCdo8c")
      sleep(1000)
    })

    t.test('test for now set agent handle', (t) => {
      t.plan(1)
      const result = display(call("app_property", {
        key: "Agent_Handle"
      }))
      t.equal(result.value, "buffaloBill")
    })

    t.test('we can obtain the dna address', (t) => {
      t.plan(1)
      const result = display(call("app_property", {
        key: "DNA_Address"
      }))
      t.equal(result.value, alice.dnaAddress)
    })

    t.test('we can obtain the agent address', (t) => {
      t.plan(1)
      const result = display(call("app_property", {
        key: "Agent_Address"
      }))
      t.equal(result.value, alice.agentId)
    })

    t.test('test requesting invalid app property', (t) => {
      t.plan(1)
      const result = display(call("app_property", {
        key: "garbage"
      }))
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
    call = (method, params) => alice.call("coolcats", method, params)

    t.test('test that handle is not set', (t) => {
      t.plan(1)
      const result = display(call("get_handle", {
        address: "QmUXkCgPqXcniV2JvRLeNZs21j4UyXoPWJ4pMtygRCdo8c"
      }))
      t.equal(result.value, undefined)
    })

    t.test("we can create a new handle", (t) => {
      t.plan(1)
      const result = display(call("use_handle", {
        handle: "buffaloBill"
      }))
      t.equal(result.value, "QmUXkCgPqXcniV2JvRLeNZs21j4UyXoPWJ4pMtygRCdo8c")
      sleep(1000)
    })

    t.test('we can retrieve the new handle', (t) => {
      t.plan(1)
      const result = display(call("get_handle", {
        address: "QmUXkCgPqXcniV2JvRLeNZs21j4UyXoPWJ4pMtygRCdo8c"
      }))
      t.equal(result.value, "buffaloBill")
    })

    t.test("we can update our handle to a unique handle", (t) => {
      t.plan(1)
      const result = display(call("use_handle", {
        handle: "phil"
      }))
      t.equal(result.value, "QmZeUu4dzkJpcZLbbn4pTN8n39CZncmQoRAWKjCuKYazN2")
      sleep(1000)
    })

    t.test("trying to use a handle already in use returns error", (t) => {
      t.plan(1)
      const result = display(call("use_handle", {
        handle: "phil"
      }))
      t.equal(result.error.ValidationFailed, "handle_in_use")
    })

    t.test("get_agent request on non-existent handle returns undefined", (t) => {
      t.plan(1)
      const result = display(call("get_agent", {
        handle: "fooHandle"
      }))
      t.equal(result.value, undefined)
    })

    t.test("we can retrieve agent by handle", (t) => {
      t.plan(1)
      const result = display(call("get_agent", {
        handle: "buffaloBill"
      }))
      t.equal(result.value, alice.agentId)
    })

    t.test("we can retrieve list of handles, in single node mode there will be only one", (t) => {
      t.plan(1)
      const result = display(call("get_handles", {}))
      t.deepEqual(result.value,
        [{
          handle: "phil",
          address: "QmZeUu4dzkJpcZLbbn4pTN8n39CZncmQoRAWKjCuKYazN2"
        }]
      )

      stop()
    })
  })
  t.end()
})

runtests.includes('posts') && test('posts', (t) => {
  Conductor.run(conductorAlice, (stop, conductor) => {
    alice = new DnaInstance(aliceName, conductor)
    call = (method, params) => alice.call("coolcats", method, params)

    t.test("setup handle for posting", (t) => {
      t.plan(1)
      const result = display(call("use_handle", {
        handle: "buffaloBill"
      }))
      t.equal(result.value, "QmUXkCgPqXcniV2JvRLeNZs21j4UyXoPWJ4pMtygRCdo8c")
      sleep(1000)
    })

    t.test('getting non-existent posts returns empty list', (t) => {
      t.plan(1)
      const result = display(call("get_posts_by", {
        user_handle: "buffaloBill"
      }))
      t.deepEqual(result.value, [])
    })

    t.test('post must have non-zero length', (t) => {
      t.plan(1)
      const result = display(call("post", {
        message: "",
        stamp: "12345"
      }))
      t.equal(result.value, undefined)
    })

    t.test('post must have length < 256 chars', (t) => {
      t.plan(1)
      const result = display(call("post", {
        message: "1234567890".repeat(26),
        stamp: "12345"
      }))
      t.equal(result.value, undefined)
    })

    t.test('we can create a new post', (t) => {
      t.plan(1)
      const result = display(call("post", {
        message: "This is a test post",
        stamp: "12345"
      }))
      t.equal(result.value, "QmWZZxnYwVuBBShQSqK7E8TTjix8bKMaA1nKkiyFhbfxHv")
      sleep(1000)
    })

    t.test('we can retrieve posts', (t) => {
      t.plan(1)
      const result = display(call("get_posts_by", {
        user_handle: "buffaloBill"
      }))
      t.deepEqual(result.value, [{
        address: "QmWZZxnYwVuBBShQSqK7E8TTjix8bKMaA1nKkiyFhbfxHv",
        post: {
          message: "This is a test post",
          stamp: "12345"
        },
        author: "buffaloBill"
      }])
    })

    t.test('we can retrieve a single post', (t) => {
      t.plan(1)
      const result = display(call("get_post", {
        address: "QmWZZxnYwVuBBShQSqK7E8TTjix8bKMaA1nKkiyFhbfxHv"
      }))
      t.deepEqual(result.value, {
        address: "QmWZZxnYwVuBBShQSqK7E8TTjix8bKMaA1nKkiyFhbfxHv",
        post: {
          message: "This is a test post",
          stamp: "12345"
        },
        author: "buffaloBill"
      })
    })

    t.test('retrieving single post will fail if not found', (t) => {
      t.plan(1)
      const result = display(call("get_post", {
        address: "QmWZZxnYwVuBBShQSqK7E8TTjix8bKMaA1nKkiyFhbfbad"
      }))
      t.equal(result.value, undefined)

      stop()
      // We can consider supporting post modifications later if desirable
    })
  })
  t.end()
})

runtests.includes('hashtags') && test('hashtags', (t) => {
  Conductor.run(conductorAlice, (stop, conductor) => {
    alice = new DnaInstance(aliceName, conductor)
    call = (method, params) => alice.call("coolcats", method, params);

    t.test('a handle is setup correctly', (t) => {
      t.plan(1)
      const result = display(call("use_handle", {
        handle: "hashmasterBill"
      }))
      t.equal(result.value, "QmWWgqWEyVpNY2qcP3S1MJrmDUySeJr1mSH146VcMTLL6p")
      sleep(1000)
    })

    t.test("a message with a hashtag is successfully created", (t) => {
      t.plan(1)
      const result = display(call("post", {
        message: "here is a test post with a #hashtag",
        stamp: "12345"
      }))
      t.equal(result.value, "Qmc91z3qNcyAFu5boQXZbTkjm27gqLXQxvaq9iPj6LyWwW")
      sleep(1000)
    })

    t.test("given a hashtag, a post containing that hashtag is returned", (t) => {
      t.plan(1)
      const result = display(call("get_posts_with_hashtag", {
        hashtag: "#hashtag"
      }))
      t.deepEqual(result.value, [{
        address: "Qmc91z3qNcyAFu5boQXZbTkjm27gqLXQxvaq9iPj6LyWwW",
        post: {
          message: "here is a test post with a #hashtag",
          stamp: "12345"
        },
        author: "hashmasterBill"
      }])

      stop()
    })
  })
  t.end()
})

runtests.includes('favourites') && test('favourites', (t) => {
  Conductor.run(conductorAlice, (stop, conductor) => {
    alice = new DnaInstance(aliceName, conductor)
    call = (method, params) => alice.call("coolcats", method, params)

    t.test("setting up a new handle to test favourites", (t) => {
      t.plan(1)
      const result = display(call("use_handle", {
        handle: "lindsey"
      }))
      t.equal(result.value, "QmTf5gGdsyCXZnZFrhvrWgB1DS29zGeFAvfub43y5YBSLH")
      sleep(1000)
    })

    t.test("creating a new post to add later as a favourite", (t) => {
      t.plan(1)
      const result = display(call("post", {
        message: "here is a test post",
        stamp: "12345"
      }))
      t.equal(result.value, "QmYDs49zjGfcL5ZDhA6bcXE3kX7GkGe2S8jBtWicYk1NLt")
      sleep(1000)
    })

    t.test("adding the last post as a favourite returns an array of one favourite", (t) => {
      t.plan(1)
      const result = display(call("add_favourite", {
        address: "QmYDs49zjGfcL5ZDhA6bcXE3kX7GkGe2S8jBtWicYk1NLt"
      }))
      t.deepEqual(result.value, ["QmYDs49zjGfcL5ZDhA6bcXE3kX7GkGe2S8jBtWicYk1NLt"])
      sleep(1000)
    })

    t.test("creating a new post to add later as a favourite", (t) => {
      t.plan(1)
      const result = display(call("post", {
        message: "here is another test post",
        stamp: "12345"
      }))
      t.equal(result.value, "QmaeajZ8BtH9sRthShKdfa3ChcenUwX7GczuHRiY45Kj51")
      sleep(1000)
    })

    t.test("adding another favourite (2 favourites) returns an array of 2 items", (t) => {
      t.plan(3)
      const result = display(call("add_favourite", {
        address: "QmaeajZ8BtH9sRthShKdfa3ChcenUwX7GczuHRiY45Kj51"
      }))
      t.equal(result.value.length, 2)
      t.equal(result.value.includes("QmaeajZ8BtH9sRthShKdfa3ChcenUwX7GczuHRiY45Kj51"), true)
      t.equal(result.value.includes("QmYDs49zjGfcL5ZDhA6bcXE3kX7GkGe2S8jBtWicYk1NLt"), true)
      sleep(1000)
    })

    t.test("adding a favourite that is not an address returns empty list", (t) => {
      t.plan(1)
      const result = display(call("add_favourite", {
        address: "Hello!"
      }))
      t.deepEqual(result.value, [])
    })

    t.test("removing a favourite that exists from a list of 2 will leave the one favourite", (t) => {
      t.plan(1)
      const result = display(call("remove_favourite", {
        address: "QmYDs49zjGfcL5ZDhA6bcXE3kX7GkGe2S8jBtWicYk1NLt"
      }))
      t.deepEqual(result.value, ["QmaeajZ8BtH9sRthShKdfa3ChcenUwX7GczuHRiY45Kj51"])
      sleep(1000)
    })

    t.test("removing a favourite that doesn't exist returns an unchanged list of favourites", (t) => {
      t.plan(1)
      const result = display(call("remove_favourite", {
        address: "QmaeajZ8BtH9sRthShKdfa3ChcenUwX7GczuHRiY45KBAD"
      }))
      t.deepEqual(result.value, ["QmaeajZ8BtH9sRthShKdfa3ChcenUwX7GczuHRiY45Kj51"])

      stop()
    })
  })
})

runtests.includes('profile') && test('profile', (t) => {
  Conductor.run(conductorAlice, (stop, conductor) => {
    alice = new DnaInstance(aliceName, conductor)
    call = (method, params) => alice.call("coolcats", method, params)

    t.test('get the first name of the user which is not set', (t) => {
      t.plan(1)
      const result = display(call("get_first_name", {}))
      t.equal(result.error.ValidationFailed, "unlinked_tag: first_name")
    })

    t.test('set the first name of the user', (t) => {
      t.plan(1)
      const result = display(call("set_first_name", {
        name: aliceName
      }))
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
      const result = display(call("set_first_name", {
        name: bobName
      }))
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
      const result = display(call("set_profile_pic", {
        dataurl: "random stuff for now"
      }))
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
      const result = display(call("set_profile_pic", {
        dataurl: "random other stuff"
      }))
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

Scenario.setTape(test)

runtests.includes('collisions') && scenario3.runTape('collisions', async (t, {
  alice,
  bob,
  carol
}) => {
  try {
    underline('Bob creates a new handle the first time he uses coolcats')
    var result = display(await bob.callSync("coolcats", "use_handle", {
      handle: "bob"
    }))
    t.equal(result.value, "QmQ19PsiG92X1Jc2zjV6CTE68CNY1X1W4WUDGjBnCE5kze")

    underline("Alice can retrieve a list of all handles")
    var result = display(alice.call("coolcats", "get_handles", {}))
    t.equal(result.value.length, 1)

    underline("Bob can retrieve a list of all handles")
    var result = display(bob.call("coolcats", "get_handles", {}))
    t.equal(result.value.length, 1)

    underline("Carol can retrieve a list of all handles")
    var result = display(carol.call("coolcats", "get_handles", {}))
    t.equal(result.value.length, 1)

    underline("Carol creates a new handle 'Archer' the first time she uses coolcats")
    var result = display(await carol.callSync("coolcats", "use_handle", {
      handle: "Archer"
    }))
    t.equal(result.value, "QmQz48TQHbpqnF4MEVxwTXmpzQs1kFuFkMDQKc3qMBPTYx")

    underline("Alice tries to use handle 'Archer' which is already taken")
    var result = display(await alice.callSync("coolcats", "use_handle", {
      handle: "Archer"
    }))
    t.equal(result.error.ValidationFailed, "handle_in_use")
  } catch (err) {
    t.fail(err.message)
  }
})

runtests.includes('follows') && scenario2.runTape('follows', async (t, {
  alice,
  bob
}) => {
  try {
    underline("setup handle for posting")
    var result = display(await alice.callSync("coolcats", "use_handle", {
      handle: "alice"
    }))
    t.equal(result.value, "QmNUHXyeperNGU2FBo5YxBZ5TvZLtgWBJQwaJ3CzmxJL3g")

    underline("we can retrieve a list of all handles")
    var result = display(alice.call("coolcats", "get_handles", {}))
    t.equal(result.value.length, 1)

    underline("create a new post")
    var result = display(await alice.callSync("coolcats", "post", {
      message: "hello world",
      stamp: "12345"
    }))
    t.equal(result.value, "Qmf3ddxyxXFjHpCCQqGg187mytBLBWa2AZNofYkLPLP4Fg")

    underline("setup handle for posting")
    var result = display(await bob.callSync("coolcats", "use_handle", {
      handle: "bob"
    }))
    t.equal(result.value, "QmQ19PsiG92X1Jc2zjV6CTE68CNY1X1W4WUDGjBnCE5kze")

    underline("There are no followers for Bob yet")
    var result = display(bob.call("coolcats", "get_followers", {
      user_handle: "bob"
    }))
    t.deepEqual(result.value, [])

    underline("follow Alice")
    var result = display(await bob.callSync("coolcats", "follow", {
      user_handle: "alice"
    }))
    t.equal(result.value, true)

    underline("retrieve Alice's posts")
    var result = display(bob.call("coolcats", "get_posts_by", {
      user_handle: "alice"
    }))
    t.deepEqual(result.value, [{
      address: "Qmf3ddxyxXFjHpCCQqGg187mytBLBWa2AZNofYkLPLP4Fg",
      post: {
        message: "hello world",
        stamp: "12345"
      },
      author: "alice"
    }])

    underline("we can retrieve a list of all handles")
    var result = display(alice.call("coolcats", "get_handles", {}))
    t.equal(result.value.length, 2)

    underline("we can retrieve a list of people Bob is following")
    var result = display(bob.call("coolcats", "get_following", {
      user_handle: "bob"
    }))
    t.deepEqual(result.value, ["alice"])
  } catch (err) {
    t.fail(err.message)
  }
})
