// This test file uses the tape testing framework.
// To learn more, go here: https://github.com/substack/tape
const test = require('tape')
const tapSpec = require('tap-spec');
test.createStream()
  .pipe(tapSpec())
  .pipe(process.stdout);

const { Config, Container } = require('@holochain/holochain-nodejs')

const dnaPath = "./dist/bundle.json"

const aliceName = "alice"
const cameronName = "cameron"

// closure to keep config-only stuff out of test scope
const container = (() => {

  const agentAlice = Config.agent(aliceName)
  const agentCameron = Config.agent(cameronName)

  const dna = Config.dna(dnaPath)

  const instanceAlice = Config.instance(agentAlice, dna)
  const instanceCameron = Config.instance(agentCameron, dna)

  const containerConfig = Config.container([instanceAlice, instanceCameron])

  return new Container(containerConfig)
})()

container.start()

const alice = container.makeCaller(aliceName, dnaPath)
const cameron = container.makeCaller(cameronName, dnaPath)

// Tests
test('has agentId', (t) => {
  t.plan(2)

  console.log(alice.agentId);
  console.log(cameron.agentId);

  t.ok(alice.agentId, "alice should have id")
  t.ok(cameron.agentId, "cameron should have id")
})

test('alice send message to cameron', (t) => {
  t.plan(1)

  const input = {
    to_agent: cameron.agentId,
    message: "Hi Cameron"
  }

  const result = alice.call("messages", "main", "send_message", input)

  console.log(result);

  t.ok(result.success, "Message should reach destination if both agents online")

  t.end()
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

  t.ok(result.Ok, "Should be able to create article")
  t.deepEqual(result.Ok.length, 46, "Address length should be 46 chars")

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

  t.ok(result.Ok, "hdk::get_entry should return article content at address for all users")
  t.deepEqual(JSON.parse(result.Ok.App[1]), expect, "Returned article should match expected data")

  t.end()
})

test('get article address by content', (t) => {
  t.plan(1)

  const input = {
    title: "Article Title",
    abst: "abstract text",
    body: "body of article"
  }

  const expect = "QmTuvXiW6MRXG4gQsXSTPPVqxwPCp6ytDxboiLVsTSThbc"

  const result = alice.call("articles", "main", "article_address", input)

  console.log(result);

  t.deepEqual(result.Ok, expect)
})

test('cameron get article addresses authored by alice', (t) => {
  t.plan(2)

  const input = {
   agent_addr: alice.agentId
  }

  const result = cameron.call("articles", "main", "get_authored_articles", input)

  console.log(result);

  t.ok(result.Ok)
  t.ok(result.Ok.addresses[0] != undefined, "Should return addresses of live articles Alice has created")

  t.end()
})

test('delete the article', (t) => {
  t.plan(1)

  const input = {
    article_addr: "QmTuvXiW6MRXG4gQsXSTPPVqxwPCp6ytDxboiLVsTSThbc"
  }

  const result = alice.call("articles", "main", "delete_article", input)

  console.log(result);

  t.ok(result.Ok === null, "Alice should be able to delete her article")

  t.end()
})

test('fail to get deleted article', (t) => {
  t.plan(1)

  const input = {
    article_addr: "QmTuvXiW6MRXG4gQsXSTPPVqxwPCp6ytDxboiLVsTSThbc"
  }

  const result = alice.call("articles", "main", "get_article", input)

  console.log(result);

  t.ok(result.Ok === null, "Should return null")

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

  console.log(create_result);

  t.ok(create_result.Ok)
  t.deepEqual(create_result.Ok.length, 46)

  const update_input = {
    article_addr: create_result.Ok,
    title: "2 Article Title",
    abst: "2 abstract text",
    body: "2 body of article"
  }

  const update_result = alice.call("articles", "main", "update_article", update_input)

  console.log(update_result);

  t.ok(update_result.Ok)
  t.deepEqual(update_result.Ok.length, 46)

  t.end()
})

test('alice create article, cameron get it', (t) => {
  t.plan(3)

  const create_input = {
    title: "3 Article Title",
    abst: "3 abstract text",
    body: "3 body of article"
  }

  const create_result = alice.call("articles", "main", "create_article", create_input)

  console.log(create_result);

  t.ok(create_result.Ok)

  const get_input = {
    article_addr: create_result.Ok
  }

  const get_result = cameron.call("articles", "main", "get_article", get_input)

  console.log(get_result);

  t.ok(get_result.Ok, "Cameron should be able to get Alice's article")
  t.deepEqual(JSON.parse(get_result.Ok.App[1]), create_input, "Returned article should match article Alice just made in this test")

  t.end()
})

test('alice create article, cameron fail to delete it', (t) => {
  t.plan(3)

  const create_input = {
    title: "4 Article Title",
    abst: "4 abstract text",
    body: "4 body of article"
  }

  const create_result = alice.call("articles", "main", "create_article", create_input)

  console.log(create_result)

  t.ok(create_result.Ok)

  const delete_input = {
    article_addr: create_result.Ok
  }

  const delete_result = cameron.call("articles", "main", "delete_article", delete_input)

  console.log(delete_result);

  t.notOk(delete_result.Err != undefined, "Shouldn't return Err")
  t.notOk(delete_result.Ok === null, "Cameron shouldn't be able to delete Alice's article")

  t.end()
})
