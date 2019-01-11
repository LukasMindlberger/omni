// This test file uses the tape testing framework.
// To learn more, go here: https://github.com/substack/tape
const test = require('tape');
const Container = require('@holochain/holochain-nodejs');

// instantiate app from DNA JSON bundle into hc container
const container = Container.loadAndInstantiate("dist/bundle.json")

// activate container instance
container.start()


// Tests
test('try creating an article', (t) => {
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


test('get created article', (t) => {
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


// test('delete created article', (t) => {
//   const input = {
//     article_addr: "QmWGENspZamWiJMXsYX7ChMTTtbHSP3aUFHcJSibioqKxE"
//   }
//
//   const expect = { "success": true }
//
//   const result = container.call("articles", "main", "delete_article", input)
//   console.log(result);
//
//   t.deepEqual(result, expect)
//
//   t.end()
// })
