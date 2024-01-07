import { createEffect, createSignal } from "solid-js"
import { invoke } from "@tauri-apps/api/tauri"

export function App() {
  const [message, setMessage] = createSignal()
  createEffect(() => invoke("greet", { name: "Joe Schmoe" }).then(setMessage))
  return <p>{message}</p>
}

export default App
