// https://nuxt.com/docs/api/configuration/nuxt-config

import wasm from 'vite-plugin-wasm'
import topLevelAwait from 'vite-plugin-top-level-await'
export default defineNuxtConfig({
  devtools: { enabled: true },
  vite: {
    plugins: [wasm(), topLevelAwait()],
    optimizeDeps: {
      exclude: [],
    },
      },
})
