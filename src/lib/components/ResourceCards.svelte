<script lang="ts">
import ResourceCard from "$lib/components/ResourceCard.svelte"
import { Resource } from "$lib/types"

export let resources: Resource[]
</script>

<div class="resource-grid flex flex-wrap justify-between gap-3">
  {#each resources as resource}
    <ResourceCard bind:resource={resource} on:toggle />
  {:else}
    <p>No results found</p>
  {/each}
</div>

<style>
 .resource-grid::after {
     content: "";
     flex: auto;
 }
</style>
