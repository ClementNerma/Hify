import App from './App.svelte'

const targetSelector = '#app'

const target = document.querySelector(targetSelector)

if (!target) {
  throw new Error('Target element not found with selector: ' + targetSelector)
}

const app = new App({ target })

export default app
