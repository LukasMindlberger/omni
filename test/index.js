// This test file uses the tape testing framework.
// To learn more, go here: https://github.com/substack/tape
const test = require('tape')
// const tapSpec = require('tap-spec');
// test.createStream()
//   .pipe(tapSpec())
//   .pipe(process.stdout);

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

  // console.log(alice.agentId);
  // console.log(cameron.agentId);

  t.ok(alice.agentId, "alice should have id")
  t.ok(cameron.agentId, "cameron should have id")
})

test('alice send message to cameron', (t) => {
  t.plan(1)

  const input = {
    to_agent: cameron.agentId,
    message: "Hi Cameron"
  }

  const response = alice.call("messages", "main", "send_message", input)

  // console.log(response);

  t.ok(response.success, "Message should reach destination if both agents online")

  t.end()
})

test('create an article', (t) => {
  t.plan(2)

  const input = {
    title: "Article Title",
    abst: "abstract text",
    body: "body of article"
  }

  const response = alice.call("articles", "main", "create_article", input)

  // console.log(response);

  t.ok(response.Ok, "Should be able to create article")
  t.deepEqual(response.Ok.length, 46, "Address length should be 46 chars")

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

  const response = alice.call("articles", "main", "get_article", input)

  // console.log(response);

  t.ok(response.Ok, "hdk::get_entry should return article content at address for all users")
  t.deepEqual(JSON.parse(response.Ok.App[1]), expect, "Returned article should match expected data")

  t.end()
})

test('get article sources', (t) => {
  const input = {
    address: "QmTuvXiW6MRXG4gQsXSTPPVqxwPCp6ytDxboiLVsTSThbc"
  }

  const response = cameron.call("articles", "main", "get_sources_latest", input)

  // console.log(response);

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

  const response = alice.call("articles", "main", "article_address", input)

  // console.log(response);

  t.deepEqual(response.Ok, expect, "Returned article should match expected data")
})

test('cameron get article addresses authored by alice', (t) => {
  t.plan(2)

  const input = {
   agent_addr: alice.agentId
  }

  const response = cameron.call("articles", "main", "get_authored_articles", input)

  // console.log(response);

  t.ok(response.Ok, "hdk::get_links shouldn't return Err")
  t.ok(response.Ok.addresses[0] != undefined, "Should return addresses of live articles Alice has created")

  t.end()
})

test('delete the article', (t) => {
  t.plan(1)

  const input = {
    article_addr: "QmTuvXiW6MRXG4gQsXSTPPVqxwPCp6ytDxboiLVsTSThbc"
  }

  const response = alice.call("articles", "main", "delete_article", input)

  // console.log(response);

  t.ok(response.Ok === null, "Alice should be able to delete her article")

  t.end()
})

test('fail to get deleted article', (t) => {
  t.plan(1)

  const input = {
    article_addr: "QmTuvXiW6MRXG4gQsXSTPPVqxwPCp6ytDxboiLVsTSThbc"
  }

  const response = alice.call("articles", "main", "get_article", input)

  // console.log(response);

  t.ok(response.Ok === null, "Deletion of entry should return null")

  t.end()
})

test('create an article and then update it', (t) => {
  t.plan(4)

  const create_input = {
    title: "1 Article Title",
    abst: "1 abstract text",
    body: "1 body of article"
  }

  const create_response = alice.call("articles", "main", "create_article", create_input)

  // console.log(create_response);

  t.ok(create_response.Ok, "Alice should create an article")
  t.deepEqual(create_response.Ok.length, 46, "Should get back article address of 46 chars")

  const update_input = {
    article_addr: create_response.Ok,
    title: "2 Article Title",
    abst: "2 abstract text",
    body: "2 body of article"
  }

  const update_response = alice.call("articles", "main", "update_article", update_input)

  // console.log(update_response);

  t.ok(update_response.Ok, "hdk::update_entry shouldn't return Err")
  t.deepEqual(update_response.Ok.length, 46, "Should return address hash with 46 chars")

  t.end()
})

test('alice create article, cameron get it', (t) => {
  t.plan(3)

  const create_input = {
    title: "3 Article Title",
    abst: "3 abstract text",
    body: "3 body of article"
  }

  const create_response = alice.call("articles", "main", "create_article", create_input)

  // console.log(create_response);

  t.ok(create_response.Ok, "Alice should create an article")

  const get_input = {
    article_addr: create_response.Ok
  }

  const get_response = cameron.call("articles", "main", "get_article", get_input)

  // console.log(get_response);

  t.ok(get_response.Ok, "Cameron should be able to get Alice's article")
  t.deepEqual(JSON.parse(get_response.Ok.App[1]), create_input, "Returned article should match article Alice just made in this test")

  t.end()
})

test('create article exceeding valid title length', (t) => {
  const input = {
    title: "haH"+"A".repeat(300),
    abst: "abstract text",
    body: "body of article"
  }

  const response = cameron.call("articles", "main", "create_article", input)

  // console.log(response);

  if (response.hasOwnProperty('Err')) {
    if (response.Err.hasOwnProperty('Internal')) {
      const error = JSON.parse(response.Err.Internal).kind

      // console.log(error);
    }
  }

  t.ok(JSON.parse(response.Err.Internal).kind.ValidationFailed, "Validation should fail")

  t.end()
})

test('alice create article, cameron fail to delete it', (t) => {
  const create_input = {
    title: "4 Article Title",
    abst: "4 abstract text",
    body: "4 body of article"
  }

  const create_response = alice.call("articles", "main", "create_article", create_input)

 console.log(create_response)

  t.ok(create_response.Ok, "Alice should create an article")

  const delete_input = {
    article_addr: create_response.Ok
  }

  const delete_response = cameron.call("articles", "main", "delete_article", delete_input)

 console.log(delete_response);

 if (delete_response.hasOwnProperty('Err')) {
   if (delete_response.Err.hasOwnProperty('Internal')) {
     const error = JSON.parse(delete_response.Err.Internal).kind

     console.log(error);
   }
 } else if (delete_response.hasOwnProperty('Ok')) {
   t.notOk(delete_response.Ok === null, "Cameron shouldn't be able to delete Alice's article")
 }

 t.end()
})

// container.stop()
