import { DbConnection } from "@/module_bindings"
import type { Identity } from "@clockworklabs/spacetimedb-sdk";

export const useConnectionStore = defineStore('connection', () => {
  const conn = shallowRef<DbConnection>()

  function onConnect(
    _conn: DbConnection,
    identity: Identity,
    token: string
  ) {
    console.log(
      'Connected to SpacetimeDB with identity:',
      identity.toHexString()
    );
    conn.value = _conn
    localStorage.setItem('auth_token', token);
  }

  function init() {
    DbConnection.builder()
      .withUri('ws://localhost:3000')
      .withModuleName('orbit')
      .withToken(localStorage.getItem('auth_token') || '')
      .onConnect(onConnect)
      .onDisconnect(console.log)
      .onConnectError(console.log)
      .build()
  }

  return {
    init,
    conn,
  }
})

if (import.meta.hot)
  import.meta.hot.accept(acceptHMRUpdate(useConnectionStore, import.meta.hot))
