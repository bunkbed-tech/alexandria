import { createEffect, createSignal } from "solid-js"
import { invoke } from "@tauri-apps/api/tauri"

export function App() {
  const [message, setMessage] = createSignal("")
  const [query, setQuery] = createSignal("")
  const [result, setResult] = createSignal("")
  createEffect(() => invoke("list_resources").then(setMessage))
  return <div>
    <p>{message()}</p>
    <input
      type="text"
      value={query()}
      onInput={(e) => setQuery(e.target.value)}
    />
  <button onClick={() => invoke("search_bgg", {query: query()}).then(setResult).catch(setResult)}>Search</button>
    <p>{result()}</p>
  </div>
}

export default App
