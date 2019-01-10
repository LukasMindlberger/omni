// This test file uses the tape testing framework.
// To learn more, go here: https://github.com/substack/tape
const test = require('tape');
const Container = require('@holochain/holochain-nodejs');

// instantiate an app from the DNA JSON bundle
const container = Container.loadAndInstantiate("dist/bundle.json")

// activate the new instance
container.start()


// Tests
test('try creating an article', (t) => {
  t.plan(1)

  const input = {
    "article": {
      "title": "Article Title",
      "abst": "abstract text",
      "body": "body of article"      
    }
  }

  const expect = {
    "success": true
  }

  const result = container.call("articles", "main", "create_article", input)

  t.looseEqual(result, expect)

  t.end()
})
