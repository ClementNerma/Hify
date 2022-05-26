import { svelte } from '@sveltejs/vite-plugin-svelte'
import { defineConfig } from 'vite'

export default ({mode}) => {
  const dotenv = require('dotenv')
  dotenv.config({ path: `./.env.${mode}` })

  return defineConfig({
    plugins: [svelte()],

    /*server: {
      port: process.env['VITE_PORT'] ?? 8892,
      host: !!process.env['VITE_HOST']
    }*/
  })
}
