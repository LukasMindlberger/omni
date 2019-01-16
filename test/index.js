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

  const result = alice.call(aliceInstanceId, "articles", "main", "create_article", input)

  console.log(result);

  t.deepEqual(result.success, expect.success)
  t.deepEqual(result.address.length, 46)

  t.end()
})

// stop all running instances
container.stop()
