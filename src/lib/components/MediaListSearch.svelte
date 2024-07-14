<script lang="ts">
import { invoke } from "@tauri-apps/api/core"

import ResourceCards from "$lib/components/ResourceCards.svelte"
import { Button } from "$lib/components/ui/button"
import { Input } from "$lib/components/ui/input"
import { Resource } from "$lib/types"

let query = ""
let promise: Promise<Resource[]>

async function fetchBggResources() {
  const api_resources = await invoke<Resource[]>("search_bgg", { query })
  const db_resources = await invoke<Resource[]>("list_resources", { resources: api_resources })
  const bgg_to_db_id = db_resources.reduce((acc, resource) => acc.set(resource.bgg_id, resource.id), new Map())
  return api_resources.map(resource => ({ ...resource, id: bgg_to_db_id.get(resource.bgg_id) }))
}
</script>

<div class="flex flex-col items-center">
  <h2>BoardGameGeek</h2>
  <form class="flex gap-4" on:submit={() => promise = fetchBggResources()}>
      <Input bind:value={query} placeholder="Enter a query ..." />
      <Button type="submit">Search</Button>
  </form>
  <ResourceCards {promise} />
</div>
