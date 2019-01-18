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
  t.plan(2)

  const input = {
    title: "Article Title",
    abst: "abstract text",
    body: "body of article"
  }

  const result = alice.call("articles", "main", "create_article", input)

  console.log(result);

  t.ok(result.success)
  t.deepEqual(result.address.length, 46)

  t.end()
})

test('get the article', (t) => {
  t.plan(2)

  const input = {
    article_addr: "QmTuvXiW6MRXG4gQsXSTPPVqxwPCp6ytDxboiLVsTSThbc"
  }

  const expect = {
    title: "Article Title",
    abst: "abstract text",
    body: "body of article"
  }

  const result = alice.call("articles", "main", "get_article", input)

  console.log(result);

  t.ok(result.success)
  t.deepEqual(JSON.parse(result.entry.App[1]), expect)

  t.end()
})

test('delete the article', (t) => {
  t.plan(2)

  const input = {
    article_addr: "QmTuvXiW6MRXG4gQsXSTPPVqxwPCp6ytDxboiLVsTSThbc"
  }

  const expect = {
    success: true
  }

  const result = alice.call("articles", "main", "delete_article", input)

  console.log(result);

  t.ok(result.success)
  t.deepEqual(result.success, expect.success)

  t.end()
})

test('create an article and then update it', (t) => {
  t.plan(4)

  const create_input = {
    title: "1 Article Title",
    abst: "1 abstract text",
    body: "1 body of article"
  }

  const create_result = alice.call("articles", "main", "create_article", create_input)

  console.log(create_result)

  t.ok(create_result.success)
  t.deepEqual(create_result.address.length, 46)

  const update_input = {
    article_addr: create_result.address,
    title: "2 Article Title",
    abst: "2 abstract text",
    body: "2 body of article"
  }

  const update_result = alice.call("articles", "main", "update_article", update_input)

  console.log(update_result);

  t.ok(update_result.success)
  t.deepEqual(update_result.address.length, 46)

  t.end()
})
