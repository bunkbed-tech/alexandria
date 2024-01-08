import { createEffect, createSignal } from "solid-js"
import { invoke } from "@tauri-apps/api/tauri"

export function App() {
  const [message, setMessage] = createSignal()
  createEffect(() => invoke("list_resources").then(setMessage))
  return <p>{message}</p>
}

export default App
