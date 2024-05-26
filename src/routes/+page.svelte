<script lang="ts">
import { invoke } from "@tauri-apps/api/core"

import { Button } from "$lib/components/ui/button"
import { Input } from "$lib/components/ui/input"
import { Skeleton } from "$lib/components/ui/skeleton"
import { Resource } from "$lib/types"

import ResourceCard from "./ResourceCard.svelte"

let query = ""
let promise: Promise<Resource[]>

function fetchBggResources() {
  promise = invoke<Resource[]>("search_bgg", { query })
}
</script>

<h2>BoardGameGeek</h2>
<form class="flex gap-4" on:submit={fetchBggResources}>
    <Input bind:value={query} placeholder="Enter a query ..." />
    <Button type="submit">Search</Button>
</form>
{#if promise}
    {#await promise}
        <Skeleton>Loading...</Skeleton>
    {:then resources}
        <div class="grid grid-cols-3 gap-3">
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
