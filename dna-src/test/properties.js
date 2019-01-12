// This test file uses the tape testing framework.
// To learn more, go here: https://github.com/substack/tape
const test = require('tape');
const Container = require('@holochain/holochain-nodejs');

// instantiate an app from the DNA JSON bundle
const app = Container.instanceFromNameAndDna("app", "dist/bundle.json")

// activate the new instance
app.start()

function display(result) {
  process.stdout.write(" ".repeat(2))
  console.log(result)
  process.stdout.write(" ".repeat(4))
}

test("Properties tests", (t) => { t.end() })

test("Make sure we can get the agent address", (t) => {
  const result = app.call("coolcats", "main", "app_property", {
    name: "Agent_Address"
  })
  display(result)
  t.equal(result.value.substr(0,2), "Qm")
  t.end()
})

test("If there is no handle set returns ''", (t) => {
  const result = app.call("coolcats", "main", "app_property", {
    name: "Agent_Handle"
  })
  display(result)
  t.equal(result.value, "")
  t.end()
})

test("We can create a new handle", (t) => {
  const result = app.call("coolcats", "main", "use_handle", {
    handle: "buffaloBill"
  })
  display(result)
  t.equal(result.value, "QmdG4gpbEbcsK15ophm46DaAqS8j2uaTUe5qHH88Ygkbka")
  t.end()
})

test("We can retrieve the new handle", (t) => {
  const result = app.call("coolcats", "main", "app_property", {
    name: "Agent_Handle"
  })
  display(result)
  t.equal(result.value, "buffaloBill")
  t.end()
})
