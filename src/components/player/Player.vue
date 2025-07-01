<script setup lang="ts">
import { Plane } from '@tresjs/cientos';
import { useTresContext } from '@tresjs/core';
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

  <TresGroup @pointer-down="pointerDown()" ref="planeRef">
    <Plane :args="[1e10, 1e10]">
      <TresMeshToonMaterial :visible="false" />
    </Plane>
  </TresGroup>
</template>
