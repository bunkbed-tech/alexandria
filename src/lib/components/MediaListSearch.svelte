<script lang="ts">
import { invoke } from "@tauri-apps/api/core"

import Pagination from "$lib/components/Pagination.svelte"
import ResourceCards from "$lib/components/ResourceCards.svelte"
import { Button } from "$lib/components/ui/button"
import { Input } from "$lib/components/ui/input"
import { Skeleton } from "$lib/components/ui/skeleton"
import { Resource } from "$lib/types"

let query = ""
let promise: Promise<number[]>
let page: number
let perPage = 20

async function searchBgg() {
  return await invoke<number[]>("search_bgg", { query })
}
async function listPageResources(ids: number[]) {
  const api_resources = await invoke<Resource[]>("list_bgg_things", { ids })
  const db_resources = await invoke<Resource[]>("list_resources", { ids })
  const bgg_to_db_id = db_resources.reduce((acc, resource) => acc.set(resource.bgg_id, resource.id), new Map())
  return api_resources.map(resource => ({ ...resource, id: bgg_to_db_id.get(resource.bgg_id) }))
}
</script>

<div class="flex flex-col items-center">
  <h2>BoardGameGeek</h2>
  <form class="flex gap-4" on:submit={() => promise = searchBgg()}>
    <Input bind:value={query} placeholder="Enter a query ..." />
    <Button type="submit">Search</Button>
  </form>
  {#if promise}
    {#await promise}
      <Skeleton class="h-4 w-[200px]" />
    {:then resourceIds}
      <Pagination count={resourceIds.length} {perPage} bind:page />
      {#await listPageResources(resourceIds.slice(perPage * (page - 1), perPage * page))}
        <Skeleton class="h-4 w-[200px]" />
      {:then resources}
        <ResourceCards {resources} />
      {:catch error}
        <p>Error: {error}</p>
      {/await}
    {:catch error}
      <p>Error: {error}</p>
    {/await}
  {/if}
</div>
