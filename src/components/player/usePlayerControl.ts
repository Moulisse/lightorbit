import { useLoop, type ThreeEvent } from "@tresjs/core"
import { Camera, Group, Object3D, Raycaster, Vector2, Vector3 } from "three"
import type { ShallowRef } from "vue"


export default function (
  speed: MaybeRef<number>,
  playerRef: ShallowRef<Group | undefined>,
  cameraRef: ShallowRef<Camera | undefined>,
  planeRef: ShallowRef<Object3D | undefined>,
) {
  const { x: pointerX, y: pointerY, pressure } = usePointer()
  const useConnection = useConnectionStore()

  let flagPosition = new Vector3(0, 0, 0)
  const shouldFollowPointer = ref(false)

  const targetPosition = computed(() => shouldFollowPointer.value ? getMouseWorldPosition() : flagPosition)

  /**
   * Mouse down
   */
  function pointerDown($event: ThreeEvent<PointerEvent>) {
    if (!$event.button)
      shouldFollowPointer.value = true
  }

  /**
   * Mouse up
   */
  watch(pressure, (pressure) => {
    if (!pressure && shouldFollowPointer) {
      shouldFollowPointer.value = false
      flagPosition = getMouseWorldPosition()
    }
  })

  const raycaster = new Raycaster()

  /**
   * Raycast screen mouse to the plane to get its world coordinate
   */
  function getMouseWorldPosition() {
    if (!playerRef.value || !cameraRef.value || !planeRef.value)
      return new Vector3(0, 0, 0)

    raycaster.setFromCamera(new Vector2(
      (pointerX.value / window.innerWidth) * 2 - 1,
      -(pointerY.value / window.innerHeight) * 2 + 1,
    ), cameraRef.value)

    return raycaster.intersectObject(planeRef.value)[0]?.point
  }

  const { onBeforeRender } = useLoop()

  /**
   * Move player
   */
  onBeforeRender(({ delta }) => {
    if (!playerRef.value || !cameraRef.value || !planeRef.value)
      return

    if (!targetPosition.value)
      return

    const distanceToFlagX = targetPosition.value.x - playerRef.value.position.x
    const distanceToFlagZ = targetPosition.value.z - playerRef.value.position.z
    const distanceToFlag = Math.sqrt(distanceToFlagX * distanceToFlagX + distanceToFlagZ * distanceToFlagZ)

    if (distanceToFlag < toValue(speed) / 100)
      return

    playerRef.value.lookAt(targetPosition.value)

    playerRef.value.position.x += distanceToFlagX / distanceToFlag * delta * toValue(speed)
    playerRef.value.position.z += distanceToFlagZ / distanceToFlag * delta * toValue(speed)
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

  setInterval(() => {
    if (shouldFollowPointer.value) {
      useConnection.conn?.reducers.setDirection({ x: targetPosition.value.x, z: targetPosition.value.z })
    } else {
      useConnection.conn?.reducers.setFlag({ x: targetPosition.value.x, z: targetPosition.value.z })
    }
  }, 1000);

  return {
    flagPosition,
    pointerDown,
  }

}
