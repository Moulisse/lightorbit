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

  let flagPosition = new Vector3(0, 0, 0)
  const shouldFollowPointer = ref(false)

  /**
   * Switch between mouse position and flag position. A flag is placed on mouse up.
   */
  const targetPosition = computed(() => shouldFollowPointer.value ? getMouseWorldPosition() : flagPosition)

  /**
   * Returns the normalized direction vector
   */
  function getDirection() {
    if (!playerRef.value || !cameraRef.value || !planeRef.value)
      return
    const targetDelta = new Vector3(
      targetPosition.value.x - playerRef.value.position.x,
      0,
      targetPosition.value.z - playerRef.value.position.z
    )

    if (targetDelta.length() < toValue(speed) / 100)
      return

    return targetDelta.normalize()
  }

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
    const direction = getDirection()
    if (!playerRef.value || !cameraRef.value || !planeRef.value || !direction)
      return

    playerRef.value.position.x += direction.x * delta * toValue(speed)
    playerRef.value.position.z += direction.z * delta * toValue(speed)

    playerRef.value.lookAt(targetPosition.value)
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

  const { } = useWebSocket('ws://localhost:3000/_ws')

  setInterval(() => {
    const direction = getDirection()
    // useConnection.conn?.reducers.setDirection(direction ? { x: direction.x, z: direction.z } : undefined)
  }, 1000);

  // useConnection.conn?.reducers.onMoveAllEntities(ctx => {
  //   console.log(ctx);

  // })

  return {
    flagPosition,
    pointerDown,
  }

}
