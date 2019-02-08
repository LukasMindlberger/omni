// Create event listeners
const title = document.querySelector('#title')
const abstract = document.querySelector('#abstract')
const body = document.querySelector('#body')
const address = document.querySelector('#address')

window.onload = () => {
  // Add article
  document.querySelector('#submit').addEventListener('click', (e) => {
    event.preventDefault()

    const new_article = {
      title: title.value,
      abst: abstract.value,
      body: body.value
    }

    // TODO: use HTTP for simple request/response

    const message = JSON.stringify({
      jsonrpc: '2.0',
      method: 'test-instance/articles/main/create_article',
      id: 1,
      params: new_article
    })

    // Send the message through the WebSocket.
    console.log(message);
    socket.send(message);
  })

  // Get article
  document.querySelector('#retrieve').addEventListener('click', (e) => {
    e.preventDefault()

    const article_addr = {
      article_addr: address.value
    }

    // TODO: use HTTP for simple request/response

    const message = JSON.stringify({
      jsonrpc: '2.0',
      method: 'test-instance/articles/main/get_article',
      id: 1,
      params: article_addr
    })

    console.log(message);
    socket.send(message);
  })

  // Create a new WebSocket.
  const socket = new WebSocket('ws://localhost:8888');

  console.log('WebSocket connection with Holochain Conductor initialised');

  // Handle any errors that occur.
  socket.onerror = function (error) {
    console.error('WebSocket Error: ' + error);
  };

  // Handle messages sent by the server.
  socket.onmessage = function (event) {
    console.log(event.data);
  };
}
