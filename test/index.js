var runtests = [
  "anchors", "properties", "handles", "posts", "hashtags", "favourites",
  "profile", "collisions", "follows"
]
if (process.env.RUNTEST) {
  runtests = [process.env.RUNTEST]
}

const tape = require('tape')
const tapSpec = require('tap-spec')

tape.createStream()
  .pipe(tapSpec())
  .pipe(process.stdout)

const {
  Diorama,
  tapeExecutor,
  backwardCompatibilityMiddleware
} = require('@holochain/diorama')

const dnaPath = "./dist/coolcats.dna.json"
const dna = Diorama.dna(dnaPath, "coolcats")

const config = {
  bridges: [],
  debugLog: false,
  executor: tapeExecutor(tape),
  middleware: backwardCompatibilityMiddleware
}

const diorama1 = new Diorama({
  ...config,
  instances: {
    alice: dna
  }
})

const diorama2 = new Diorama({
  ...config,
  instances: {
    alice: dna,
    bob: dna
  }
})

const diorama3 = new Diorama({
  ...config,
  instances: {
    alice: dna,
    bob: dna,
    carol: dna
  }
})

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

runtests.includes('anchors') && diorama1.registerScenario('anchors', async (s, t, {
  alice
}) => {
  call = (method, params) => alice.call("coolcats", method, params)
  try {
    underline("create and get anchors")
    var addr = display(await call("create_anchor", {
      anchor: {
        anchor_type: "testing",
        anchor_text: "1-2-3"
      }
    }))
    var result = display(await call("get_anchor", {
      address: addr.value
    }))
    t.deepEqual(result.value, {
      anchor_type: "testing",
      anchor_text: "1-2-3"
    })

    underline("check that anchor exists works")
    var result1 = display(await call("anchor_exists", {
      anchor: {
        anchor_type: "testing",
        anchor_text: "1-2-3"
      }
    }))
    var result2 = display(await call("anchor_exists", {
      anchor: {
        anchor_type: "testing",
        anchor_text: "3-2-1"
      }
    }))
    t.equal(result1.value, true)
    t.equal(result2.value, false)

    underline("get anchors from links")
    var result = display(await call("get_anchors", {
      anchor_type: "testing"
    }))
    t.deepEqual(result.value, [{
      anchor_type: "testing",
      anchor_text: "1-2-3"
    }])
  } catch (err) {
    t.fail(err.message)
  }
})

runtests.includes('properties') && diorama1.registerScenario('properties', async (s, t, {
  alice
}) => {
  call = (method, params) => alice.call("coolcats", method, params)
  try {
    underline("test for unset agent handle")
    var result = display(await call("app_property", {
      key: "Agent_Handle"
    }))
    t.equal(result.value, undefined)

    underline("we can create a new handle")
    var result = display(await call("use_handle", {
      handle: "buffaloBill"
    }))
    t.equal(result.value, "QmUXkCgPqXcniV2JvRLeNZs21j4UyXoPWJ4pMtygRCdo8c")
    await s.consistent()

    underline("test for now set agent handle")
    var result = display(await call("app_property", {
      key: "Agent_Handle"
    }))
    t.equal(result.value, "buffaloBill")

    underline("we can obtain the dna address")
    var result = display(await call("app_property", {
      key: "DNA_Address"
    }))
    t.equal(result.value, alice.dnaAddress)

    underline("we can obtain the agent address")
    var result = display(await call("app_property", {
      key: "Agent_Address"
    }))
    t.equal(result.value, alice.agentId)

    underline("test requesting invalid app property")
    var result = display(await call("app_property", {
      key: "garbage"
    }))
    t.equal(result.error.ValidationFailed, "No App Property with key: garbage")
  } catch (err) {
    t.fail(err.message)
  }
})

runtests.includes('handles') && diorama1.registerScenario('handles', async (s, t, {
  alice
}) => {
  call = (method, params) => alice.call("coolcats", method, params)
  try {
    underline("test that handle is not set")
    var result = display(await call("get_handle", {
      address: "QmUXkCgPqXcniV2JvRLeNZs21j4UyXoPWJ4pMtygRCdo8c"
    }))
    t.equal(result.value, undefined)

    underline("we can create a new handle")
    var result = display(await call("use_handle", {
      handle: "buffaloBill"
    }))
    t.equal(result.value, "QmUXkCgPqXcniV2JvRLeNZs21j4UyXoPWJ4pMtygRCdo8c")
    await s.consistent()

    underline("we can retrieve the new handle")
    var result = display(await call("get_handle", {
      address: "QmUXkCgPqXcniV2JvRLeNZs21j4UyXoPWJ4pMtygRCdo8c"
    }))
    t.equal(result.value, "buffaloBill")

    underline("we can update our handle to a unique handle")
    var result = display(await call("use_handle", {
      handle: "phil"
    }))
    t.equal(result.value, "QmZeUu4dzkJpcZLbbn4pTN8n39CZncmQoRAWKjCuKYazN2")
    await s.consistent()

    underline("trying to use a handle already in use returns error")
    var result = display(await call("use_handle", {
      handle: "phil"
    }))
    t.equal(result.error.ValidationFailed, "handle_in_use")

    underline("get_agent request on non-existent handle returns undefined")
    var result = display(await call("get_agent", {
      handle: "fooHandle"
    }))
    t.equal(result.value, undefined)

    underline("we can retrieve agent by handle")
    var result = display(await call("get_agent", {
      handle: "buffaloBill"
    }))
    t.equal(result.value, alice.agentId)

    underline("we can retrieve list of handles, in single node mode there will be only one")
    var result = display(await call("get_handles", {}))
    t.deepEqual(result.value, [{
      handle: "phil",
      address: "QmZeUu4dzkJpcZLbbn4pTN8n39CZncmQoRAWKjCuKYazN2"
    }])
  } catch (err) {
    t.fail(err.message)
  }
})

runtests.includes('posts') && diorama1.registerScenario('posts', async (s, t, {
  alice
}) => {
  call = (method, params) => alice.call("coolcats", method, params)
  try {
    underline("setup handle for posting")
    var result = display(await call("use_handle", {
      handle: "buffaloBill"
    }))
    t.equal(result.value, "QmUXkCgPqXcniV2JvRLeNZs21j4UyXoPWJ4pMtygRCdo8c")
    await s.consistent()

    underline("getting non-existent posts returns empty list")
    var result = display(await call("get_posts_by", {
      handles: ["buffaloBill"]
    }))
    t.deepEqual(result.value, [])

    underline("post must have non-zero length")
    var result = display(await call("post", {
      message: "",
      stamp: "12345"
    }))
    t.equal(result.value, undefined)

    underline("post must have length < 256 chars")
    var result = display(await call("post", {
      message: "1234567890".repeat(26),
      stamp: "12345"
    }))
    t.equal(result.value, undefined)

    underline("we can create a new post")
    var result = display(await call("post", {
      message: "This is a test post",
      stamp: "12345"
    }))
    t.equal(result.value, "QmWZZxnYwVuBBShQSqK7E8TTjix8bKMaA1nKkiyFhbfxHv")
    await s.consistent()

    underline("we can retrieve posts")
    var result = display(await call("get_posts_by", {
      handles: ["buffaloBill"]
    }))
    t.deepEqual(result.value, [{
      address: "QmWZZxnYwVuBBShQSqK7E8TTjix8bKMaA1nKkiyFhbfxHv",
      post: {
        message: "This is a test post",
        stamp: "12345"
      },
      author: "buffaloBill"
    }])

    underline("we can retrieve a single post")
    var result = display(await call("get_post", {
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

    underline("retrieving single post will fail if not found")
    var result = display(await call("get_post", {
      address: "QmWZZxnYwVuBBShQSqK7E8TTjix8bKMaA1nKkiyFhbfbad"
    }))
    t.equal(result.value, undefined)

    // We can consider supporting post modifications later if desirable
  } catch (err) {
    t.fail(err.message)
  }
})

runtests.includes('hashtags') && diorama1.registerScenario('hashtags', async (s, t, {
  alice
}) => {
  call = (method, params) => alice.call("coolcats", method, params)
  try {
    underline("a handle is setup correctly")
    var result = display(await call("use_handle", {
      handle: "hashmasterBill"
    }))
    t.equal(result.value, "QmWWgqWEyVpNY2qcP3S1MJrmDUySeJr1mSH146VcMTLL6p")
    await s.consistent()

    underline("a message with a hashtag is successfully created")
    var result = display(await call("post", {
      message: "here is a test post with a #hashtag",
      stamp: "12345"
    }))
    t.equal(result.value, "Qmc91z3qNcyAFu5boQXZbTkjm27gqLXQxvaq9iPj6LyWwW")
    await s.consistent()

    underline("given a hashtag, a post containing that hashtag is returned")
    var result = display(await call("get_posts_with_hashtag", {
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
  } catch (err) {
    t.fail(err.message)
  }
})

runtests.includes('favourites') && diorama1.registerScenario('favourites', async (s, t, {
  alice
}) => {
  call = (method, params) => alice.call("coolcats", method, params)
  try {
    underline("setting up a new handle to test favourites")
    var result = display(await call("use_handle", {
      handle: "lindsey"
    }))
    t.equal(result.value, "QmTf5gGdsyCXZnZFrhvrWgB1DS29zGeFAvfub43y5YBSLH")
    await s.consistent()

    underline("creating a new post to add later as a favourite")
    var result = display(await call("post", {
      message: "here is a test post",
      stamp: "12345"
    }))
    t.equal(result.value, "QmYDs49zjGfcL5ZDhA6bcXE3kX7GkGe2S8jBtWicYk1NLt")
    await s.consistent()

    underline("adding the last post as a favourite returns an array of one favourite")
    var result = display(await call("add_favourite", {
      address: "QmYDs49zjGfcL5ZDhA6bcXE3kX7GkGe2S8jBtWicYk1NLt"
    }))
    t.deepEqual(result.value, ["QmYDs49zjGfcL5ZDhA6bcXE3kX7GkGe2S8jBtWicYk1NLt"])
    await s.consistent()

    underline("creating a new post to add later as a favourite")
    var result = display(await call("post", {
      message: "here is another test post",
      stamp: "12345"
    }))
    t.equal(result.value, "QmaeajZ8BtH9sRthShKdfa3ChcenUwX7GczuHRiY45Kj51")
    await s.consistent()

    underline("adding another favourite (2 favourites) returns an array of 2 items")
    var result = display(await call("add_favourite", {
      address: "QmaeajZ8BtH9sRthShKdfa3ChcenUwX7GczuHRiY45Kj51"
    }))
    t.equal(result.value.length, 2)
    t.equal(result.value.includes("QmaeajZ8BtH9sRthShKdfa3ChcenUwX7GczuHRiY45Kj51"), true)
    t.equal(result.value.includes("QmYDs49zjGfcL5ZDhA6bcXE3kX7GkGe2S8jBtWicYk1NLt"), true)
    await s.consistent()

    underline("adding a favourite that is not an address returns empty list")
    var result = display(await call("add_favourite", {
      address: "Hello!"
    }))
    t.deepEqual(result.value, [])

    underline("removing a favourite that exists from a list of 2 will leave the one favourite")
    var result = display(await call("remove_favourite", {
      address: "QmYDs49zjGfcL5ZDhA6bcXE3kX7GkGe2S8jBtWicYk1NLt"
    }))
    t.deepEqual(result.value, ["QmaeajZ8BtH9sRthShKdfa3ChcenUwX7GczuHRiY45Kj51"])
    await s.consistent()

    underline("removing a favourite that doesn't exist returns an unchanged list of favourites")
    var result = display(await call("remove_favourite", {
      address: "QmaeajZ8BtH9sRthShKdfa3ChcenUwX7GczuHRiY45KBAD"
    }))
    t.deepEqual(result.value, ["QmaeajZ8BtH9sRthShKdfa3ChcenUwX7GczuHRiY45Kj51"])
  } catch (err) {
    t.fail(err.message)
  }
})

runtests.includes('profile') && diorama1.registerScenario('profile', async (s, t, {
  alice
}) => {
  call = (method, params) => alice.call("coolcats", method, params)
  try {
    underline("get the first name of the user which is not set")
    var result = display(await call("get_first_name", {}))
    t.equal(result.error.ValidationFailed, "unlinked_prop: first_name")

    underline("set the first name of the user")
    var result = display(await call("set_first_name", {
      name: "alice"
    }))
    t.equal(result.value, "alice")
    await s.consistent()

    underline("get the first name of the user")
    var result = display(await call("get_first_name", {}))
    t.equal(result.value, "alice")

    underline("reset the first name of the user")
    var result = display(await call("set_first_name", {
      name: "bob"
    }))
    t.equal(result.value, "bob")
    await s.consistent()

    underline("get the new first name of the user")
    var result = display(await call("get_first_name", {}))
    t.equal(result.value, "bob")

    underline("get the profile pic of the user which is not set")
    var result = display(await call("get_profile_pic", {}))
    t.equal(result.error.ValidationFailed, "unlinked_prop: profile_pic")

    underline("set the profile_pic of the user")
    var result = display(await call("set_profile_pic", {
      dataurl: "random stuff for now"
    }))
    t.equal(result.value, "random stuff for now")
    await s.consistent()

    underline("get the profile pic of the user")
    var result = display(await call("get_profile_pic", {}))
    t.equal(result.value, "random stuff for now")

    underline("reset the profile_pic of the user")
    var result = display(await call("set_profile_pic", {
      dataurl: "random other stuff"
    }))
    t.equal(result.value, "random other stuff")
    await s.consistent()

    underline("get the new profile pic of the user")
    var result = display(await call("get_profile_pic", {}))
    t.equal(result.value, "random other stuff")
  } catch (err) {
    t.fail(err.message)
  }
})

runtests.includes('collisions') && diorama3.registerScenario('collisions', async (s, t, {
  alice,
  bob,
  carol
}) => {
  try {
    underline('Bob creates a new handle the first time he uses coolcats')
    var result = display(await bob.call("coolcats", "use_handle", {
      handle: "bob"
    }))
    t.equal(result.value, "QmQ19PsiG92X1Jc2zjV6CTE68CNY1X1W4WUDGjBnCE5kze")
    await s.consistent()

    underline("Alice can retrieve a list of all handles")
    var result = display(await alice.call("coolcats", "get_handles", {}))
    t.equal(result.value.length, 1)

    underline("Bob can retrieve a list of all handles")
    var result = display(await bob.call("coolcats", "get_handles", {}))
    t.equal(result.value.length, 1)

    underline("Carol can retrieve a list of all handles")
    var result = display(await carol.call("coolcats", "get_handles", {}))
    t.equal(result.value.length, 1)

    underline("Carol creates a new handle 'Archer' the first time she uses coolcats")
    var result = display(await carol.call("coolcats", "use_handle", {
      handle: "Archer"
    }))
    t.equal(result.value, "QmQz48TQHbpqnF4MEVxwTXmpzQs1kFuFkMDQKc3qMBPTYx")
    await s.consistent()

    underline("Alice tries to use handle 'Archer' which is already taken")
    var result = display(await alice.call("coolcats", "use_handle", {
      handle: "Archer"
    }))
    t.equal(result.error.ValidationFailed, "handle_in_use")
  } catch (err) {
    t.fail(err.message)
  }
})

runtests.includes('follows') && diorama2.registerScenario('follows', async (s, t, {
  alice,
  bob
}) => {
  try {
    underline("setup handle for posting")
    var result = display(await alice.call("coolcats", "use_handle", {
      handle: "alice"
    }))
    t.equal(result.value, "QmNUHXyeperNGU2FBo5YxBZ5TvZLtgWBJQwaJ3CzmxJL3g")
    await s.consistent()

    underline("we can retrieve a list of all handles")
    var result = display(await alice.call("coolcats", "get_handles", {}))
    t.equal(result.value.length, 1)

    underline("create a new post")
    var result = display(await alice.call("coolcats", "post", {
      message: "hello world",
      stamp: "12345"
    }))
    t.equal(result.value, "Qmf3ddxyxXFjHpCCQqGg187mytBLBWa2AZNofYkLPLP4Fg")
    await s.consistent()

    underline("setup handle for posting")
    var result = display(await bob.call("coolcats", "use_handle", {
      handle: "bob"
    }))
    t.equal(result.value, "QmQ19PsiG92X1Jc2zjV6CTE68CNY1X1W4WUDGjBnCE5kze")
    await s.consistent()

    underline("There are no followers for Bob yet")
    var result = display(await bob.call("coolcats", "get_followers", {
      user_handle: "bob"
    }))
    t.deepEqual(result.value, [])

    underline("follow Alice")
    var result = display(await bob.call("coolcats", "follow", {
      user_handle: "alice"
    }))
    t.equal(result.value, true)
    await s.consistent()

    underline("retrieve Alice's posts")
    var result = display(await bob.call("coolcats", "get_posts_by", {
      handles: ["alice"]
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
    var result = display(await alice.call("coolcats", "get_handles", {}))
    t.equal(result.value.length, 2)

    underline("we can retrieve a list of people Bob is following")
    var result = display(await bob.call("coolcats", "get_following", {
      user_handle: "bob"
    }))
    t.deepEqual(result.value, ["alice"])
  } catch (err) {
    t.fail(err.message)
  }
})

diorama3.run()
diorama2.run()
diorama1.run()
