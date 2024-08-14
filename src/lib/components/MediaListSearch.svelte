<script lang="ts">
import { invoke } from "@tauri-apps/api/core"
import Fuse from "fuse.js"
import { toast } from "svelte-sonner"

import AlertError from "$lib/components/AlertError.svelte"
import Pagination from "$lib/components/Pagination.svelte"
import ResourceCards from "$lib/components/ResourceCards.svelte"
import { Button } from "$lib/components/ui/button"
import { Input } from "$lib/components/ui/input"
import { Skeleton } from "$lib/components/ui/skeleton"
import { Slider } from "$lib/components/ui/slider"
import { Resource } from "$lib/types"

let query = ""
let filterQuery = ""
let promise: Promise<void>
let page: number
let perPage = 20
let resources: Resource[] = []

$: min = resources.reduce(
  (min, resource) => (resource.year_published && resource.year_published < min ? resource.year_published : min),
  Number.POSITIVE_INFINITY,
)
$: max = resources.reduce(
  (max, resource) => (resource.year_published && resource.year_published > max ? resource.year_published : max),
  Number.NEGATIVE_INFINITY,
)
$: yearPublishedRange = [min, max] as [number, number]
$: filteredResources = filterResources(resources, filterQuery, yearPublishedRange)
$: pageResources = filteredResources.slice(perPage * (page - 1), perPage * page)

async function searchBggThings() {
  const [_resources, errors] = await invoke<[Resource[], string[]]>("search_bgg_things", { query })

  // Render any errors as toasts
  for (const error of errors) {
    toast.custom(AlertError, { componentProps: { error } })
  }

  // Match with resources already tracked in database
  const db_resources = await invoke<Resource[]>("list_resources", { ids: _resources.map(resource => resource.bgg_id) })
  const bgg_to_db_id = db_resources.reduce((acc, resource) => acc.set(resource.bgg_id, resource.id), new Map())
  resources = _resources.map(resource => ({ ...resource, id: bgg_to_db_id.get(resource.bgg_id) }))
}

function filterResources(resources: Resource[], filterQuery: string, yearPublishedRange: [number, number]): Resource[] {
  let resourcesFiltered = resources
  if (filterQuery !== "") {
    const options = {
      includeScore: true,
      ignoreLocation: true,
      ignoreFieldNorm: true,
      useExtendedSearch: true,
      keys: [
        { name: "name", weight: 0.99 },
        { name: "description", weight: 0.01 },
      ],
    }
    const fuse = new Fuse(resources, options)
    resourcesFiltered = fuse.search(filterQuery).map(result => result.item)
  }
  return resourcesFiltered.filter(
    result =>
      result.year_published &&
      result.year_published >= yearPublishedRange[0] &&
      result.year_published <= yearPublishedRange[1],
  )
}
</script>

<div class="flex flex-col items-center">
  <h2>BoardGameGeek</h2>
  <form class="flex gap-4" on:submit={() => promise = searchBggThings()}>
    <Input bind:value={query} placeholder="Enter a query ..." />
    <Button type="submit">Search</Button>
  </form>
  <Input bind:value={filterQuery} placeholder="Filter results by name, description" />
  {#if promise}
    {#await promise}
      <Skeleton class="h-4 w-[200px]" />
    {:then}
      <Slider bind:value={yearPublishedRange} {min} {max} />
      <Pagination count={filteredResources.length} {perPage} bind:page />
      <ResourceCards resources={pageResources} />
    {:catch error}
      <p>Error: {error}</p>
    {/await}
  {/if}
</div>
