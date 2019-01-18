// This test file uses the tape testing framework.
// To learn more, go here: https://github.com/substack/tape
const test = require('tape')

const { Config, Container } = require('@holochain/holochain-nodejs')

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

container.start()

const alice = container.makeCaller(aliceName, dnaPath)


// Tests
test('has agentId', (t) => {
  t.plan(1)

  console.log(alice.agentId);

  t.ok(alice.agentId)
})


test('create an article', (t) => {
  t.plan(1)

  const input = {
    title: "Article Title",
    abst: "abstract text",
    body: "body of article"
  }

  const result = alice.call("articles", "main", "create_article", input)

  console.log(result);

  t.deepEqual(result.Ok.length, 46)

  t.end()
})


test('get article', (t) => {
  t.plan(1)

  const input = {
    article_addr: 'QmTuvXiW6MRXG4gQsXSTPPVqxwPCp6ytDxboiLVsTSThbc'
  }

  const expect = {
    title: "Article Title",
    abst: "abstract text",
    body: "body of article"
  }

  const result = alice.call("articles", "main", "get_article", input)

  console.log(result)

  if (result.Err) {
    t.notOk(result.Err)
  } else {
    t.deepEqual(JSON.parse(result.Ok.App[1]), expect)
  }
})


test('create a new article and then update', (t) => {
  t.plan(2)

  const input1 = {
    title: "First Article Title",
    abst: "first abstract text",
    body: "first body of article"
  }

  // Alice creates first article
  const create = alice.call("articles", "main", "create_article", input1)

  console.log(create);

  t.deepEqual(create.Ok.length, 46)

  const input2 = {
    address: create.Ok,
    title: "Second Article Title",
    abst: "second abstract text",
    body: "second body of article"
  }

  // Alice updates first article to second article
  const result = alice.call("articles", "main", "update_article", input2)

  console.log(result);

  if (result.Ok) {
    t.notOk(result.Err)
  } else {
    t.deepEqual(result.Ok.length, 46)
  }

  t.end()
})


test('delete article', (t) => {
  const input = {
    article_addr: 'QmTuvXiW6MRXG4gQsXSTPPVqxwPCp6ytDxboiLVsTSThbc'
  }

  const result = alice.call("articles", "main", "delete_article", input)

  console.log(result);

  t.equal(result.Ok, null)

  t.end()
})
