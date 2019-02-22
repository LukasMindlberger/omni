// This test file uses the tape testing framework.
// To learn more, go here: https://github.com/substack/tape
const { Config, Scenario } = require("@holochain/holochain-nodejs");
Scenario.setTape(require("tape"));

const dnaPath = "./dist/bundle.json";
const agentAlice = Config.agent("alice");
const agentCameron = Config.agent("cameron");
const dna = Config.dna(dnaPath);
const instanceAlice = Config.instance(agentAlice, dna);
const instanceCameron = Config.instance(agentCameron, dna);
const scenario = new Scenario([instanceAlice, instanceCameron]);

// TESTS
// (tip: add tests you're working on at the top so you don't have to sit through all the other tests (took me too long to realise))

scenario.runTape(
  "alice create article, cameron fail to delete it",
  (t, { alice, cameron }) => {
    const create_input = {
      title: "4 Article Title",
      abst: "4 abstract text",
      body: "4 body of article"
    };

    const create_response = alice.call(
      "articles",
      "create_article",
      create_input
    );

    // console.log(create_response);

    const delete_input = {
      article_addr: create_response.Ok
    };

    const delete_response = cameron.call(
      "articles",
      "delete_article",
      delete_input
    );

    // console.log(delete_response);

    if (delete_response.hasOwnProperty("Err")) {
      if (delete_response.Err.hasOwnProperty("Internal")) {
        const error = JSON.parse(delete_response.Err.Internal).kind;

        console.log(error);
      }
    } else if (delete_response.hasOwnProperty("Ok")) {
      t.notOk(
        delete_response.Ok === null,
        "Cameron shouldn't be able to delete Alice's article"
      );
    }
  }
);

scenario.runTape(
  "create article exceeding valid title length",
  (t, { alice, cameron }) => {
    const input = {
      title: "haH" + "A".repeat(300),
      abst: "abstract text",
      body: "body of article"
    };

    const response = cameron.call("articles", "create_article", input);

    // console.log(response);

    if (response.hasOwnProperty("Err")) {
      if (response.Err.hasOwnProperty("Internal")) {
        const error = JSON.parse(response.Err.Internal).kind;

        // console.log(error);
      }
    }

    t.ok(
      JSON.parse(response.Err.Internal).kind.ValidationFailed,
      "Validation should fail"
    );
  }
);

scenario.runTape(
  "cameron get articles authored by alice",
  (t, { alice, cameron }) => {
    t.plan(1);

    const input = {
      agent_addr: alice.agentId
    };

    const response = cameron.call("articles", "get_authored_articles", input);

    console.log(response.Ok);

    t.ok(response.Ok[0], "should return articles vec");
  }
);

scenario.runTape(
  "link article from keyword, then get articles from keyword",
  (t, { alice, cameron }) => {
    t.plan(2);

    const response1 = alice.call("articles", "create_article", {
      title: "Article Title",
      abst: "abstract text",
      body: "body of article"
    });

    // console.log(response1);

    const input = {
      keyword: "Astrophysics",
      article_addr: response1.Ok
    };

    const response2 = alice.call(
      "articles",
      "create_and_link_keyword_to_article",
      input
    );

    // console.log(response2);

    t.ok(response2.Ok === null, "article should link from keyword");

    // Now get back articles
    const response3 = cameron.call("articles", "get_articles_from_keyword", {
      keyword: input.keyword
    });

    console.log(response3.Ok);

    t.ok(response3.Ok[0], "should retrieve articles from keyword");
  }
);

scenario.runTape(
  "create an article and get the article",
  (t, { alice, cameron }) => {
    t.plan(4);

    // create
    const input = {
      title: "Article Title",
      abst: "abstract text",
      body: "body of article"
    };

    const response = alice.call("articles", "create_article", input);

    // console.log(response);

    t.ok(response.Ok, "Should be able to create article");
    t.deepEqual(response.Ok.length, 46, "Address length should be 46 chars");

    // get
    const input2 = {
      article_addr: response.Ok
    };

    const response2 = alice.call("articles", "get_article", input2);

    // console.log(response);

    t.ok(
      response2.Ok,
      "hdk::get_entry should return article content at address"
    );
    t.deepEqual(
      JSON.parse(response2.Ok.App[1]),
      input,
      "Returned article should match expected data"
    );
  }
);

scenario.runTape("alice send message to cameron", (t, { alice, cameron }) => {
  t.plan(1);

  const input = {
    to_agent: cameron.agentId,
    message: "Hi Cameron"
  };

  const response = alice.call("messages", "send_message", input);

  // console.log(response);

  t.ok(
    response.success,
    "Message should reach destination if both agents online"
  );
});

scenario.runTape("show environment data", (t, { alice, cameron }) => {
  t.plan(1);

  const result = alice.call("articles", "show_env", {});

  console.log(result);

  t.ok(result);
});

scenario.runTape("has agentID", (t, { alice, cameron }) => {
  t.plan(2);

  console.log(alice.agentId);
  console.log(cameron.agentId);

  t.ok(alice.agentId, "alice should have id");
  t.ok(cameron.agentId, "cameron should have id");
});
