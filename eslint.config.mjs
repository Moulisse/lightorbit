// @ts-check
import withNuxt from './.nuxt/eslint.config.mjs'
import oxlint from "eslint-plugin-oxlint"

export default withNuxt({
  rules: {
    'vue/multi-word-component-names': 'off'
  },
  ...oxlint.configs["flat/recommended"]
})
