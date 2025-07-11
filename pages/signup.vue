<script setup lang="ts">

const email = ref('')
const password = ref('')

async function signup() {

}
</script>

<template>
  <div>
    <button @click="signup">
      Create new account
    </button>
    <input v-model="email" type="email" />
    <input v-model="password" type="password" />
  </div>
</template>
