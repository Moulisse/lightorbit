<script setup lang="ts">
import { Group } from 'three';
import usePlayerControl from './usePlayerControl';

const { camera } = useTresContext()

const playerRef = shallowRef<Group>()
const planeRef = shallowRef<Group>()

const speed = 100

// Setup player camera and movement
const { flagPosition, pointerDown } = usePlayerControl(speed, playerRef, camera, planeRef)
</script>

<template>
  <TresGroup ref="playerRef" :position="flagPosition">
    <Suspense>
      <PlayerModel />
    </Suspense>
  </TresGroup>

  <TresGroup @pointer-down="pointerDown($event)" ref="planeRef" :position="[0, 0, 0]">
    <Plane :args="[1e10, 1e10]">
      <TresMeshBasicMaterial :visible="false" />
    </Plane>
  </TresGroup>
</template>
