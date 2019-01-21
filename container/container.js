const { Config, Container } = require('@holochain/holochain-nodejs')

const dnaPath = "./bundle.json"

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

// Begin script
const result = alice.call("messages", "main", "send_message", input)

console.log(result);

container.stop()
