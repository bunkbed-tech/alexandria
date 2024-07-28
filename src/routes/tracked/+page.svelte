<script lang="ts">
import { invoke } from "@tauri-apps/api/core"
import { onMount } from "svelte"

import Pagination from "$lib/components/Pagination.svelte"
import ResourceCards from "$lib/components/ResourceCards.svelte"
import { Skeleton } from "$lib/components/ui/skeleton"
import * as Tabs from "$lib/components/ui/tabs"
import { Resource } from "$lib/types"

let perPage = 50

let promise: Promise<Resource[]>
let page: number

function queryTrackedResources() {
  promise = invoke<Resource[]>("list_resources", { ids: null })
}

onMount(queryTrackedResources)
</script>

<Tabs.Root value="board-games">
  <Tabs.List>
    <Tabs.Trigger value="board-games">Board Games</Tabs.Trigger>
    <Tabs.Trigger value="tab-2">Tab 2</Tabs.Trigger>
    <Tabs.Trigger value="tab-3">Tab 3</Tabs.Trigger>
  </Tabs.List>
  <Tabs.Content value="board-games">
    {#if promise}
      {#await promise}
        <Skeleton class="h-4 w-[200px]" />
      {:then resources}
        <Pagination count={resources.length} {perPage} bind:page />
        <ResourceCards
          resources={resources.slice(perPage * (page - 1), perPage * page)}
          on:toggle={queryTrackedResources}
        />
      {:catch error}
        <p>Error: {error}</p>
      {/await}
    {/if}
  </Tabs.Content>
  <Tabs.Content value="tab-2">Welcome to Tab 2</Tabs.Content>
  <Tabs.Content value="tab-3">Welcome to Tab 3</Tabs.Content>
</Tabs.Root>
