// This test file uses the tape testing framework.
// To learn more, go here: https://github.com/substack/tape
const test = require('tape')
const { Config, Container } = require('@holochain/holochain-nodejs')

const dnaPath = "dist/bundle.json"

const aliceName = "alice"
const tashName = "tash"

const agentAlice = Config.agent(aliceName)
const agentTash = Config.agent(tashName)
const dna = Config.dna(dnaPath)
const instanceAlice = Config.instance(agentAlice, dna)
const instanceTash = Config.instance(agentTash, dna)
const config = Config.container([instanceAlice, instanceTash])

const container = new Container(config)

const aliceInstanceId = aliceName + '::' + dnaPath

test('create an article', (t) => {
  t.plan(2)

  const alice = container.makeCaller(aliceName, dnaPath)

  const input = {
    article: {
      title: "Article Title",
      abst: "abstract text",
      body: "body of article"
    }
  }

  const expect = {
    "success": true
  }

  const aliceResult = alice.call("articles", "main", "create_article", input)
  console.log(result);

  t.deepEqual(aliceResult.success, expect.success)
  t.deepEqual(aliceResult.address.length, 46)

  t.end()
})

// stop all running instances
container.stop()



/*
test('create an article', (t) => {
  t.plan(2)
  const input = {
    article: {
      title: "Article Title",
      abst: "abstract text",
      body: "body of article"
    }
  }

  const expect = {
    "success": true
  }

  const result = container.call("articles", "main", "create_article", input)
  console.log(result);

  t.deepEqual(result.success, expect.success)
  t.deepEqual(result.address.length, 46)

  t.end()
})

test('get article', (t) => {
  t.plan(1)
  const input = {
    article_addr: "QmWGENspZamWiJMXsYX7ChMTTtbHSP3aUFHcJSibioqKxE"
  }

  const expect = {
    title: "Article Title",
    abst: "abstract text",
    body: "body of article"
  }

  const result = container.call("articles", "main", "get_article", input)
  console.log(result);

  t.deepEqual(result, expect)

  t.end()
})
*/
