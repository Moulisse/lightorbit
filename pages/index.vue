<script lang="ts" setup>
definePageMeta({
  middleware: 'auth'
})
</script>

<template>
  <TresCanvas window-size>
    <TresPerspectiveCamera :position="[0, 150, 30]" :look-at="[0, 0, 0]" />

    <TresDirectionalLight :position="[0, 100, 0]" :intensity="3" />

    <Suspense>
      <Scene />
    </Suspense>

    <Suspense>
      <EffectComposerPmndrs>
        <FXAAPmndrs />
      </EffectComposerPmndrs>
    </Suspense>
  </TresCanvas>
</template>
