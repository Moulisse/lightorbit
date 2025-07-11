export default defineNuxtRouteMiddleware(async (to, from) => {
  const session = useSupabaseSession()

  if (!session.value) {
    return navigateTo('/login')
  }

})
