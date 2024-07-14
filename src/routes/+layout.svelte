<script lang="ts">
import "../app.css"
import * as Collapsible from "$lib/components/ui/collapsible"
import { ChevronDown, ChevronUp, Search } from "lucide-svelte"

let open = false
</script>

<div class="h-screen w-screen flex">
  <aside class="w-[20%] border-r-[3px] bg-primary">
    <button class="flex w-full justify-between p-3">
      <a href="/">Search</a>
      <Search />
    </button>
    <Collapsible.Root bind:open>
      <Collapsible.Trigger class="flex w-full justify-between p-3">
        <a href="/tracked">Tracked</a>
        {#if open}
          <ChevronUp />
        {:else}
          <ChevronDown />
        {/if}
      </Collapsible.Trigger>
      <Collapsible.Content>
        <!-- <div class="w-full p-3 pl-6"><a href="/tracked">Tracked</a></div> -->
      </Collapsible.Content>
    </Collapsible.Root>
  </aside>
  <main class="w-full h-full overflow-y-scroll p-2">
    <slot></slot>
  </main>
</div>
