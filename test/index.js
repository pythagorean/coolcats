// This test file uses the tape testing framework.
// To learn more, go here: https://github.com/substack/tape
const test = require('tape');
const Container = require('@holochain/holochain-nodejs');

// instantiate an app from the DNA JSON bundle
const app = Container.instanceFromNameAndDna("app", "dist/bundle.json")

// activate the new instance
app.start()

test("If there is no handle set returns ''", (t) => {
  const result = app.call("coolcats", "main", "get_handle", {})
  t.equal(result[0], "")
  t.end()
})

test("We can create a new handle", (t) => {
  const result = app.call("coolcats", "main", "use_handle", {handle: "buffaloBill"})
  console.log(result)
  t.end()
})

test("We can retrieve the new handle", (t) => {
  const result = app.call("coolcats", "main", "get_handle", {})
  t.equal(result[0], "buffaloBill")
  t.end()
})
