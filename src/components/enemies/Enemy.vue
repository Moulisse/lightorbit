<script setup lang="ts">
import type { ThreeEvent } from '@tresjs/core';
import type Enemy from './Enemy';
import { Cylinder, Plane } from '@tresjs/cientos';

const props = defineProps<{
  data: Enemy
}>()

const size = 36

const { currentTarget } = useTarget()

function clickEnemy($event: ThreeEvent<PointerEvent>) {
  $event.stopPropagation()
  currentTarget.value = props.data.id
}

const vertexShader = `
varying vec2 vUv;

void main() {
    gl_Position = projectionMatrix * viewMatrix * modelMatrix * vec4(position, 1.0);
    vUv = uv;
}
`

const fragmentShader = `
precision mediump float;
varying vec2 vUv;

void main() {
    vec4 circleColor = vec4(1, 0, 0, 0.7);
    float thickness = 0.03;
    float fade = 0.02;

    vec2 uv = vec2((vUv - 0.5) * 2.0);

    float distance =  1.0 - length(uv);
    vec4 color = vec4(smoothstep(0.0, fade, distance));
    color *= vec4(smoothstep(thickness + fade, thickness, distance));

    gl_FragColor = circleColor * vec4(color);
}
`

</script>

<template>
  <TresGroup :position="[data.position[0], 0, data.position[1]]">
    <component :is="data.model"></component>
    <Cylinder :args="[size / 2, size / 2, 1]" @pointer-down="clickEnemy">
      <TresMeshBasicMaterial :visible="false" />
    </Cylinder>
    <Plane :args="[size, size]" :position="[currentTarget === data.id ? 0 : Infinity, 0, 0]">
      <TresShaderMaterial :vertexShader :fragment-shader transparent />
    </Plane>
  </TresGroup>
</template>
