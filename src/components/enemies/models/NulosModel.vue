<script setup lang="ts">
import { useGLTF } from '@tresjs/cientos';

const { scene } = await useGLTF('/corvette_2.gltf')
</script>

<template>
  <primitive :object="scene" />
</template>
