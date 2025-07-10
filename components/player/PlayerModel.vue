<script setup lang="ts">
const { scene } = await useGLTF('/corvette_2.gltf')
</script>

<template>
  <primitive :object="scene" />
</template>
