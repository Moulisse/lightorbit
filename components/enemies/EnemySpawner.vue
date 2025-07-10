<script setup lang="ts">
import type Enemy from './Enemy';
import NulosModel from './models/NulosModel.vue';

const enemies: Enemy[] = [
  {
    id: 0,
    model: NulosModel,
    position: [50, 0]
  },
  {
    id: 1,
    model: NulosModel,
    position: [100, 0]
  },
  {
    id: 2,
    model: NulosModel,
    position: [50, 50]
  },
]
</script>

<template>
  <template v-for="enemy of enemies" :key="enemy.id">
    <Enemy :data="enemy"></Enemy>
  </template>
</template>
