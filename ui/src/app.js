// Create element references.
const title = document.querySelector('#title')
const abstract = document.querySelector('#abstract')
const body = document.querySelector('#body')
const submitBtn = document.querySelector('#submitBtn')
const message = document.getElementById("message")
const clearBtn = document.querySelector('#clearBtn')
const address = document.querySelector('#address')
const retrieveBtn = document.querySelector("#retrieveBtn")
const show_article = document.querySelector('#show_article')

const port = 8888
const holochainUrl =  `ws://localhost:${port}/`

clearBtn.addEventListener('click', e => {
  title.value = ''
  abstract.value = ''
  body.value = ''
})

window.holochainClient.connect(holochainUrl)
.then(({call, close}) => {

  submitBtn.addEventListener('click', e => {
    e.preventDefault()

    submitBtn.classList.add("loading")

    call('info/instances')().then(info => {
      const instance = 'test-instance'
      const zomeName = 'articles'
      const capability = 'main'
      const functionName = 'create_article'
      const params = {
        title: title.value,
        abst: abstract.value,
        body: body.value
      }
      const createArticle = call(instance, zomeName, capability, functionName)

      createArticle(params)
      .then((res) => {
        const response = JSON.parse(res)

        if (response.Err) {
          console.error(response.Err);
          submitBtn.classList.remove("loading")
          message.style.display = "none"
        } else {
          message.innerText = 'Address: ' + response.Ok
          message.style.display = "block"
          submitBtn.classList.remove("loading")
        }
      })
    })
  })


  retrieveBtn.addEventListener('click', e => {
    e.preventDefault()

    retrieveBtn.classList.add("loading")
    show_article.style.display = "block"

    call('info/instances')().then(info => {
      const instance = 'test-instance'
      const zomeName = 'articles'
      const capability = 'main'
      const functionName = 'get_article'
      const params = {
        article_addr: address.value
      }
      const getArticle = call(instance, zomeName, capability, functionName)

      getArticle(params)
      .then((res) => {
        const response = JSON.parse(res)

        if (response.Err) {
          console.error(response.Err)
          show_article.style.display = "none"
          retrieveBtn.classList.remove("loading")
        } else {
          const article = JSON.parse(response.Ok.App[1])

          show_article.innerHTML = `
          <div class="ui center aligned header">${article.title}</div>
          <div class="ui segment">${article.abst}</div>
          <div class="ui basic segment">
            <div style="white-space: pre-wrap">${article.body}</div>
          </div>
          `

          retrieveBtn.classList.remove("loading")
        }
      })
    })
  })
})
