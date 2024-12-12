<script lang="ts">
import "../app.css";
import { page } from "$app/stores";
import * as Collapsible from "$lib/components/ui/collapsible";
import { Toaster } from "$lib/components/ui/sonner";
import { ChevronDown, ChevronUp, Search } from "lucide-svelte";

const open = false;
</script>

<Toaster />
<div class="h-screen w-screen flex">
  <aside class="w-[20%] border-r-[3px] bg-primary text-white font-bold text-xl">
    <a href="/" class="rounded-none flex w-full items-center justify-between p-3 hover:bg-white hover:text-brown {$page.url.pathname === "/" ? "bg-secondary" : ""}">
      <p>Search</p>
      <Search />
    </a>
    <Collapsible.Root bind:open>
      <Collapsible.Trigger class="flex w-full justify-between">
        <a href="/tracked" class="rounded-none flex w-full justify-between p-3 hover:bg-white hover:text-brown {$page.url.pathname === "/tracked" ? "bg-secondary" : ""}">
          <p>Tracked</p>
          {#if open}
            <ChevronUp />
          {:else}
            <ChevronDown />
          {/if}
        </a>
      </Collapsible.Trigger>
      <Collapsible.Content class="text-lg">
        <a href="/tracked2" class="pl-6 rounded-none flex w-full justify-between p-3 hover:bg-white hover:text-brown {$page.url.pathname === "/tracked2" ? "bg-secondary" : ""}">
          <p>Stuff</p>
        </a>
      </Collapsible.Content>
    </Collapsible.Root>
  </aside>
  <main class="w-full h-full overflow-y-scroll p-2">
    <slot></slot>
  </main>
</div>
