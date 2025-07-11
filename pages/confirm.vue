<script setup lang="ts">
const supabase = useSupabaseClient()

const user = shallowRef()
const { data } = supabase.auth.onAuthStateChange(async () => {
  user.value = await supabase.auth.getUser()
  console.log(user.value);

})

onUnmounted(() => {
  data.subscription.unsubscribe()
})

watch(user, () => {
  if (user.value) {
    // Redirect to protected page
    return navigateTo('/')
  }
}, { immediate: true })
</script>

<template>
  <div>Waiting for login...</div>
</template>
