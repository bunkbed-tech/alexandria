<script lang="ts">
import { invoke } from "@tauri-apps/api/core"

import Pagination from "$lib/components/Pagination.svelte"
import ResourceCards from "$lib/components/ResourceCards.svelte"
import { Button } from "$lib/components/ui/button"
import { Input } from "$lib/components/ui/input"
import { Skeleton } from "$lib/components/ui/skeleton"
import { Resource } from "$lib/types"

let query = ""
let promise: Promise<[Resource[], string[]]>
let page: number
let perPage = 20

async function searchBggThings(): Promise<[Resource[], string[]]> {
  const [resources, errors] = await invoke<[Resource[], string[]]>("search_bgg_things", { query })
  const db_resources = await invoke<Resource[]>("list_resources", { ids: resources.map(resource => resource.bgg_id) })
  const bgg_to_db_id = db_resources.reduce((acc, resource) => acc.set(resource.bgg_id, resource.id), new Map())
  return [resources.map(resource => ({ ...resource, id: bgg_to_db_id.get(resource.bgg_id) })), errors]
}
</script>

<div class="flex flex-col items-center">
  <h2>BoardGameGeek</h2>
  <form class="flex gap-4" on:submit={() => promise = searchBggThings()}>
    <Input bind:value={query} placeholder="Enter a query ..." />
    <Button type="submit">Search</Button>
  </form>
  {#if promise}
    {#await promise}
      <Skeleton class="h-4 w-[200px]" />
    {:then [resources, errors]}
      <Pagination count={resources.length} {perPage} bind:page />
      <ResourceCards resources={resources.slice(perPage * (page - 1), perPage * page)} />
    {:catch error}
      <p>Error: {error}</p>
    {/await}
  {/if}
</div>
