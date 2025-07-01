<script setup lang="ts">
import { GLTFModel, Plane } from '@tresjs/cientos';
import { useLoop, type TresInstance } from '@tresjs/core';
import { Vector3 } from 'three';


const playerRef = shallowRef<TresInstance | null>()
const cameraRef = shallowRef<TresInstance | null>()

const targetPosition = new Vector3(0, 0, 0)
let shouldFollowPointer = false

function pointerDown() {
  shouldFollowPointer = true
}

function pointerUp(x: number, z: number) {
  shouldFollowPointer = false
  targetPosition.x = x
  targetPosition.z = z
}

const speed = 100

const { onBeforeRender } = useLoop()

/**
 * Move player
 */
onBeforeRender(({ delta }) => {
  if (!playerRef.value)
    return

  if (shouldFollowPointer) {

  } else {
    const distanceToFlagX = targetPosition.x - playerRef.value.position.x
    const distanceToFlagZ = targetPosition.z - playerRef.value.position.z
    const distanceToFlag = Math.sqrt(distanceToFlagX * distanceToFlagX + distanceToFlagZ * distanceToFlagZ)

    if (distanceToFlag < speed / 100)
      return

    playerRef.value.lookAt(targetPosition)

    playerRef.value.position.x += distanceToFlagX / distanceToFlag * delta * speed
    playerRef.value.position.z += distanceToFlagZ / distanceToFlag * delta * speed
  }
})

/**
 * Move camera
 */
onBeforeRender(() => {
  if (!playerRef.value || !cameraRef.value)
    return
  cameraRef.value.position.x = playerRef.value.position.x
  cameraRef.value.position.z = playerRef.value.position.z + 60

  cameraRef.value.lookAt(playerRef.value.position)
})

</script>

<template>
  <TresPerspectiveCamera :position="[0, 150, 30]" :look-at="[0, 0, 0]" ref="cameraRef" />

  <TresGroup ref="playerRef" :position="targetPosition">
    <Suspense>
      <GLTFModel path="/corvette_2.glb" draco />
    </Suspense>
  </TresGroup>


  <TresGroup @pointer-down="pointerDown()" @pointer-up="pointerUp($event.point.x, $event.point.z)">
    <Plane :args="[1000, 1000]">
      <TresMeshToonMaterial :visible="false" />
    </Plane>
  </TresGroup>
</template>
