import { createPinia } from 'pinia'

export default defineNuxtPlugin((nuxtApp) => {
  // Create and install Pinia manually since we removed the module
  const pinia = createPinia()
  nuxtApp.vueApp.use(pinia)
}) 