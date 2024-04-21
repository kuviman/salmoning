let bridge_replier = console.log;

(async () => {
  try {
    const based = "../../salmoning.js";
    const path = `${based}`;
    const salmoning = await import(/* @vite-ignore */ path);
    bridge_replier = salmoning.bridge_reply;
  } catch (e) {
    console.warn("activating ui debug");
    const debug = await import("./debug");
    debug.activate();
  }
})();

export function bridge_reply() {
  return bridge_replier(...arguments);
}
