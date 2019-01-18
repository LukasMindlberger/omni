import { http } from './http'
import { ui } from './ui'

// Get posts on DOM load
document.addEventListener('DOMContentLoaded', init)

// Listen for add post
document.querySelector('.post-submit').addEventListener('click', submitPost)

// Listen for delete
document.querySelector('#posts').addEventListener('click', deletePost)

// Listen for edit state
document.querySelector('#posts').addEventListener('click', enableEdit)

// Listen for cancel edit
document.querySelector('.card-form').addEventListener('click', cancelEdit)

function init() {
  ui.clearFields()
  getPosts()
}

// Get posts from database
function getPosts() {
  http.get('http://localhost:3004/posts')
    .then(data => ui.showPosts(data))
    .catch(err => console.error('Error getting posts'))
}

// Submit post
function submitPost() {
  const title = document.querySelector('#title').value
  const body = document.querySelector('#body').value
  const id = document.querySelector('#id').value

  const data = {
    title, body
  }

  // Validate input
  if (title === '' || body === '') {
    ui.showAlert('Missing title or body', 'alert alert-danger')
  } else {
    // Check if existing ID
    if (id === '') {
      // Add new post
      // Create POST request
      http.post('http://localhost:3004/posts', data)
        .then((data) => {
          ui.showAlert('Post added', 'alert alert-success')
          ui.clearFields()
          getPosts()
        })
        .catch((err) => {
          console.error('Error posting post');
        })
    } else {
      // Update existing post
      http.put(`http://localhost:3004/posts/${id}`, data)
        .then((data) => {
          ui.showAlert('Post updated', 'alert alert-success')
          ui.changeFormState('add')
          getPosts()
        })
        .catch((err) => {
          console.error('Error posting post');
        })
    }
  }
}

// Delete post
function deletePost(event) {
  event.preventDefault()
  if (event.target.parentElement.classList.contains('delete')) {
    const id = event.target.parentElement.dataset.id
    if (confirm('Delete?')) {
      http.delete(`http://localhost:3004/posts/${id}`)
        .then((data) => {
          ui.showAlert('Post removed', 'alert alert-success')
          getPosts()
        })
        .catch((err) => {
          console.log(err);
        })
    }
  }
}

// Enable edit state
function enableEdit(event) {
  event.preventDefault()
  if (event.target.parentElement.classList.contains('edit')) {
    const id = event.target.parentElement.dataset.id
    const body = event.target.parentElement.previousElementSibling.textContent
    const title = event.target.parentElement.previousElementSibling.previousElementSibling.textContent

    const data = {
      id, title, body
    }

    // Fill form with current post
    ui.fillForm(data)
  }
}

// Cancel edit state
function cancelEdit(event) {
  event.preventDefault()
  if (event.target.classList.contains('post-cancel')) {
    ui.changeFormState('add')
  }
}
