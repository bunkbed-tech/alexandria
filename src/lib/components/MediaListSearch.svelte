<script lang="ts">
import { invoke } from "@tauri-apps/api/core"
import Fuse from "fuse.js"
import { toast } from "svelte-sonner"

import AlertError from "$lib/components/AlertError.svelte"
import Pagination from "$lib/components/Pagination.svelte"
import ResourceCards from "$lib/components/ResourceCards.svelte"
import { Button } from "$lib/components/ui/button"
import { Input } from "$lib/components/ui/input"
import * as Select from "$lib/components/ui/select"
import { Skeleton } from "$lib/components/ui/skeleton"
import { Slider } from "$lib/components/ui/slider"
import { Resource } from "$lib/types"

const perPage = 20
const sorters = ["default", "alphabetical", "year", "tracked"] as const
type Sorter = (typeof sorters)[number]
const sorterCompareFns: Record<Sorter, (a: Resource, b: Resource) => number> = {
  default: (_a, _b) => 0,
  alphabetical: (a, b) => {
    // Uppercase everything to be case-insensitive
    const titleA = a.title.toUpperCase()
    const titleB = b.title.toUpperCase()
    if (titleA < titleB) return -1
    if (titleA > titleB) return 1
    return 0
  },
  year: (a, b) => (a.year_published || Number.POSITIVE_INFINITY) - (b.year_published || Number.POSITIVE_INFINITY),
  tracked: (a, b) => +!!b.id - +!!a.id,
}

let query = ""
let promise: Promise<void> | null = null
let page = 1
let resources: Resource[] = []
let sort = { value: "default" as Sorter }

$: min = resources.reduce(
  (min, resource) => (resource.year_published && resource.year_published < min ? resource.year_published : min),
  Number.POSITIVE_INFINITY,
)
$: max = resources.reduce(
  (max, resource) => (resource.year_published && resource.year_published > max ? resource.year_published : max),
  Number.NEGATIVE_INFINITY,
)
$: yearPublishedRange = [min, max] as [number, number]
$: filteredResources = resources
  // Year published within slider range (keep all nulls)
  .filter(
    result =>
      result.year_published == null ||
      (result.year_published >= yearPublishedRange[0] && result.year_published <= yearPublishedRange[1]),
  )
  // Sort the results by the specified feature
  .sort(sorterCompareFns[sort.value])
$: pageResources = filteredResources.slice(perPage * (page - 1), perPage * page)

async function searchBggThings() {
  const [api_resources, errors] = await invoke<[Resource[], string[]]>("search_bgg_things", { query })

  // Render any errors as toasts
  for (const error of errors) {
    toast.custom(AlertError, { componentProps: { error } })
  }

  // Match with resources already tracked in database
  const db_resources = await invoke<Resource[]>("list_resources", {
    ids: api_resources.map(resource => resource.bgg_id),
  })
  const bgg_to_db_id = db_resources.reduce((acc, resource) => acc.set(resource.bgg_id, resource.id), new Map())
  const matched_resources = api_resources.map(resource => ({ ...resource, id: bgg_to_db_id.get(resource.bgg_id) }))

  // Fuzzy search
  const options = {
    includeScore: true,
    keys: [
      {
        name: "title",
        weight: 0.9,
      },
      {
        name: "description",
        weight: 0.1,
      },
    ],
  }
  const fuse = new Fuse(matched_resources, options)
  // TODO The app freezes if this is an empty array... WTF
  resources = fuse.search(query).map(result => result.item)
}

function onToggleResource() {
  console.log("Updating resources on toggle")
  resources = [...resources.slice(0, perPage * (page - 1)), ...pageResources, ...resources.slice(perPage * page)]
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
    {:then}
      {#if resources.length != 0}
        <Slider bind:value={yearPublishedRange} {min} {max} />
        <Select.Root bind:selected={sort}>
          <Select.Trigger class="w-[180px]">
            <Select.Value placeholder="Theme" />
          </Select.Trigger>
          <Select.Content>
            <Select.Item value="default" label="Default" />
            <Select.Item value="alphabetical" label="Alphabetical" />
            <Select.Item value="year" label="Year Published" />
            <Select.Item value="tracked" label="Tracking Status" />
          </Select.Content>
        </Select.Root>
        <Pagination count={filteredResources.length} {perPage} bind:page />
      {/if}
      <ResourceCards bind:resources={pageResources} on:toggle={onToggleResource} />
    {:catch error}
      <p>Error: {error}</p>
    {/await}
  {/if}
</div>
