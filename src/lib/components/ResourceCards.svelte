<script lang="ts">
import ResourceCard from "$lib/components/ResourceCard.svelte"
import { Skeleton } from "$lib/components/ui/skeleton"
import { Resource } from "$lib/types"

export let promise: Promise<Resource[]>
</script>

{#if promise}
    {#await promise}
        <Skeleton class="h-4 w-[200px]" />
    {:then resources}
        <div class="resource-grid flex flex-wrap justify-between gap-3">
            {#each resources as resource}
                <ResourceCard {resource} on:toggle />
            {:else}
                <p>No results found<p>
            {/each}
        </div>
    {:catch error}
        <p>Error: {error}</p>
    {/await}
{/if}

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
