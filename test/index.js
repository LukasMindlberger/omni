// This test file uses the tape testing framework.
// To learn more, go here: https://github.com/substack/tape
const { Config, Scenario } = require("@holochain/holochain-nodejs");
Scenario.setTape(require("tape"));

const dnaPath = "./dist/omni.dna.json";
const agentAlice = Config.agent("alice");
const agentBob = Config.agent("bob");
const dna = Config.dna(dnaPath);
const instanceAlice = Config.instance(agentAlice, dna);
const instanceBob = Config.instance(agentBob, dna);
const scenario = new Scenario([instanceAlice, instanceBob]);

// Tip: Insert new tests at the top of this file

scenario.runTape(
  "alice add and get article, instanceBob get authored articles by alice",
  async (t, { alice, bob }) => {
    const addr = await alice.callSync("articles", "create_article", {
      title: "test title",
      abst: "test abstract",
      body: "test body"
    });
    const result = await alice.callSync("articles", "get_article", {
      article_addr: addr.Ok
    });
    console.log(result);
    t.deepEqual(result, {
      Ok: {
        App: [
          "article",
          JSON.stringify({
            title: "test title",
            abst: "test abstract",
            body: "test body"
          })
        ]
      }
    });

    const get_mine = await alice.callSync("articles", "get_my_articles", {});
    console.log(get_mine);
    t.true(
      get_mine.Ok.addresses.length === 1,
      "alice gets her list of articles"
    );

    const get_alice_authored = await bob.callSync(
      "articles",
      "get_authored_articles",
      {
        agent_addr: alice.agentId
      }
    );
    console.log(get_alice_authored);
    t.true(get_alice_authored.Ok.addresses.length === 1, "returns 1 article");
  }
);
