<script setup lang="ts">
import type { ThreeEvent } from '@tresjs/core';

const { currentTarget } = useTarget()

function clickEnemy($event: ThreeEvent<PointerEvent>) {
  $event.stopPropagation()
}
</script>

<template>
  <TresGroup @pointer-down="clickEnemy">
    <slot />
  </TresGroup>
</template>
