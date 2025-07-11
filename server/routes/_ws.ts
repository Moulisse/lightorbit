export default defineWebSocketHandler({
  async upgrade(request) {


    throw new Response('Unauthorized', {
      status: 401,
    })
  },

  async open(peer) {
    if (!peer.request.headers.get('cookie'))
      return peer.close()
    console.log("[ws] open", peer.id);
  },

  message(peer, message) {
    console.log("[ws] message", peer.id, message);
    if (message.text().includes("ping")) {
      peer.send("pong");
    }
  },

  close(peer, event) {
    console.log("[ws] close", peer.id, event);
  },

  error(peer, error) {
    console.log("[ws] error", peer.id, error);
  },
});

