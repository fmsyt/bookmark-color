import { invoke } from "@tauri-apps/api/core";

export async function startMouseHook() {
  const started = invoke<boolean>("start_mouse_hook");
  return started;
}

export async function stopMouseHook() {
  const stopped = invoke<boolean>("stop_mouse_hook");
  return stopped;
}
