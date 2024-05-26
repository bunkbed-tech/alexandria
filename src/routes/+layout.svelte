<script lang="ts">
import "../app.css"
import * as Collapsible from "$lib/components/ui/collapsible"
import { ChevronDown, ChevronUp, Search } from "lucide-svelte"

let open = false
</script>

<div class="h-screen w-screen flex">
  <aside class="w-[20%] border-r-[3px] bg-primary text-secondary">
    <button class="flex w-full justify-between p-3">
      <a href="/">Search</a>
      <Search />
    </button>
    <Collapsible.Root bind:open>
      <Collapsible.Trigger class="flex w-full justify-between p-3">
        Lists
        {#if open}
          <ChevronUp />
        {:else}
          <ChevronDown />
        {/if}
      </Collapsible.Trigger>
      <Collapsible.Content>
        <div class="w-full p-3 pl-6"><a href="/owned">Owned</a></div>
        <div class="w-full p-3 pl-6"><a href="/owned">Owned</a></div>
      </Collapsible.Content>
    </Collapsible.Root>
  </aside>
  <slot></slot>
</div>
