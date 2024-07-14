<script lang="ts">
import { invoke } from "@tauri-apps/api/core"
 import { onMount } from "svelte"

import ResourceCards from "$lib/components/ResourceCards.svelte"
import * as Tabs from "$lib/components/ui/tabs"
import { Resource } from "$lib/types"

let promise: Promise<Resource[]>

function queryTrackedResources() {
   promise = invoke<Resource[]>("list_resources", { resources: null })
}

onMount(queryTrackedResources)
</script>

<Tabs.Root value="board-games">
  <Tabs.List>
    <Tabs.Trigger value="board-games">Board Games</Tabs.Trigger>
    <Tabs.Trigger value="tab-2">Tab 2</Tabs.Trigger>
    <Tabs.Trigger value="tab-3">Tab 3</Tabs.Trigger>
  </Tabs.List>
  <Tabs.Content value="board-games"><ResourceCards {promise} on:toggle={queryTrackedResources} /></Tabs.Content>
  <Tabs.Content value="tab-2">Welcome to Tab 2</Tabs.Content>
  <Tabs.Content value="tab-3">Welcome to Tab 3</Tabs.Content>
</Tabs.Root>
