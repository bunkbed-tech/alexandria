<script lang="ts">
import { invoke } from "@tauri-apps/api/core"

import ResourceCard from "$lib/components/ResourceCard.svelte"
import { Button } from "$lib/components/ui/button"
import { Input } from "$lib/components/ui/input"
import { Skeleton } from "$lib/components/ui/skeleton"
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
  {#if promise}
      {#await promise}
          <Skeleton class="h-4 w-[200px]" />
      {:then resources}
          <div class="resource-grid flex flex-wrap justify-between gap-3">
              {#each resources as resource}
                  <ResourceCard {resource} />
              {:else}
                  <p>No results found<p>
              {/each}
          </div>
      {:catch error}
          <p>Error: {error}</p>
      {/await}
  {/if}
</div>
<style>
 /* .resource-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, 200px);
    grid-gap: 1rem;
    justify-content: space-between;
    } */
 .resource-grid::after {
     content: "";
     flex: auto;
 }
</style>
