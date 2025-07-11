import tailwindcss from "@tailwindcss/vite";

// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  compatibilityDate: '2025-05-15',
  devtools: { enabled: true },

  nitro: {
    preset: "cloudflare_module",

    cloudflare: {
      deployConfig: true,
      nodeCompat: true
    },

    experimental: {
      websocket: true,
    }
  },
  modules: [
    "nitro-cloudflare-dev",
    '@nuxt/icon',
    '@nuxt/fonts',
    '@nuxt/eslint',
    '@tresjs/nuxt',
    '@vueuse/nuxt',
    '@pinia/nuxt',
    '@nuxtjs/supabase',
  ],
  css: ['~/assets/css/main.css'],
  vite: { plugins: [tailwindcss(),], },
  components: [
    {
      path: '~/components',
      pathPrefix: false,
    },
  ],
  runtimeConfig: {
    supabaseServiceKey: '',
    public: {
      supabaseUrl: '',
      supabaseKey: '',
    },
  },
})