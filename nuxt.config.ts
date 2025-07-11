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
    supabaseUrl: '',
    supabaseKey: '',
    public: {
      supabaseUrl: process.env.SUPABASE_URL,
      supabaseAnonKey: process.env.SUPABASE_ANON_KEY,
    },
  },
})