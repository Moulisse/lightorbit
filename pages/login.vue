<script setup lang="ts">
const supabase = useSupabaseClient()
const router = useRouter()

const email = ref('')
const password = ref('')

const signIn = async () => {
  const { error } = await supabase.auth.signInWithPassword({
    email: email.value,
    password: password.value,
  })
  if (error)
    return console.log(error)

  router.push('/')
}
</script>

<template>
  <div>
    <button @click="signIn">
      Sign In with E-Mail
    </button>
    <input v-model="email" type="email" />
    <input v-model="password" type="password" />
  </div>
</template>
