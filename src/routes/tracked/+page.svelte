<script lang="ts">
import { invoke } from "@tauri-apps/api/core"
import Fuse from "fuse.js"
 // import { onMount } from "svelte"
import { toast } from "svelte-sonner"

import AlertError from "$lib/components/AlertError.svelte"
import Pagination from "$lib/components/Pagination.svelte"
import ResourceCards from "$lib/components/ResourceCards.svelte"
import { Button } from "$lib/components/ui/button"
import { Input } from "$lib/components/ui/input"
import { Skeleton } from "$lib/components/ui/skeleton"
import * as Tabs from "$lib/components/ui/tabs"
import * as Select from "$lib/components/ui/select"
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
let promise: Promise<void> | null = listResources()
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

async function listResources() {
  const maybe_resources = await invoke<Resource[] | String>("list_resources", { ids: null })

  // Render error as toast
  if (maybe_resources instanceof String) {
    toast.custom(AlertError, { componentProps: { error: maybe_resources } })
    resources = []
    return
  }

  if (query !== "") {
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
    const fuse = new Fuse(maybe_resources, options)
    resources = fuse.search(query).map(result => result.item)
  } else {
    resources = maybe_resources
  }
}

function onToggleResource() {
  console.log("Updating resources on toggle")
  promise = listResources()
}
</script>

<Tabs.Root value="board-games">
  <Tabs.List>
    <Tabs.Trigger value="board-games">Board Games</Tabs.Trigger>
    <Tabs.Trigger value="tab-2">Tab 2</Tabs.Trigger>
    <Tabs.Trigger value="tab-3">Tab 3</Tabs.Trigger>
  </Tabs.List>
  <Tabs.Content value="board-games">
    <form class="flex gap-4" on:submit={() => promise = listResources()}>
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
              <Select.Value placeholder="Default" />
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
  </Tabs.Content>
  <Tabs.Content value="tab-2">Welcome to Tab 2</Tabs.Content>
  <Tabs.Content value="tab-3">Welcome to Tab 3</Tabs.Content>
</Tabs.Root>
