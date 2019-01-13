// This test file uses the tape testing framework.
// To learn more, go here: https://github.com/substack/tape
const test = require('tape');
const Container = require('@holochain/holochain-nodejs');
// const { Config, Container } = require('@holochain/holochain-nodejs');

const dnaPath = "dist/bundle.json"

// Multi-agent container scenario testing (waiting for container version 0.3.0)
/*
const multi_container = (function() {
  const agentMarcus = Config.agent("marcus")
  const agentCameron = Config.agent("cameron")

  const dna = Config.dna(dnaPath)

  const instanceMarcus = Config.instance(agentMarcus, dna)
  const instanceCameron = Config.instance(agentCameron, dna)

  const containerConfig = Config.container([instanceMarcus, instanceCameron])
  return new Container(containerConfig)
}());

activate container instance
multi_container.start()

const marcus = multi_container.makeCaller('marcus', dnaPath)
const cameron = multi_container.makeCaller('cameron', dnaPath)
*/

// Basic container scenario testing
const container = Container.loadAndInstantiate(dnaPath)

container.start()


// Non-agent tests
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


// Multi-agent tests
/*
test('agentId', (t) => {
  t.plan(2)

  t.ok(marcus.agentId)

  t.notEqual(marcus.agentId, cameron.agentId)
})
*/
