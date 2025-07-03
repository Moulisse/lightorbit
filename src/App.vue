<script setup lang="ts">
import { Grid } from '@tresjs/cientos';
import { TresCanvas } from '@tresjs/core'
import { EffectComposerPmndrs, FXAAPmndrs } from '@tresjs/post-processing';

const useConnection = useConnectionStore()
useConnection.init()
</script>

<template>
  <TresCanvas window-size v-if="useConnection.conn?.isActive">
    <TresPerspectiveCamera :position="[0, 150, 30]" :look-at="[0, 0, 0]" />

    <Grid :args="[10.5, 10.5]" cell-color="#f97316" :cell-size="1" :cell-thickness="0.8" section-color="#f97316"
      :section-size="10" :section-thickness="1" :infinite-grid="true" :fade-from="0" :fade-distance="200"
      :fade-strength="1" />

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
