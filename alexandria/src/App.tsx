import { createMemo, createSignal } from "solid-js"
import { invoke } from "@tauri-apps/api/tauri"
import { useMachine, normalizeProps } from "@zag-js/solid"
import * as toggle from "@zag-js/toggle"

export function App() {
  const [state, send] = useMachine(toggle.machine({ id: "2" }))
  const api = createMemo(() => toggle.connect(state, send, normalizeProps))
  return (
    <button {...api().buttonProps}>
      {api().isPressed ? "On" : "Off"}
    </button>
  )
}

export default App
