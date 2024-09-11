<script lang="ts">
import { invoke } from "@tauri-apps/api/core"
import { Bookmark, Check, ChevronDown } from "lucide-svelte"
import { createEventDispatcher } from "svelte"

import { Button } from "$lib/components/ui/button"
import * as Combobox from "$lib/components/ui/combobox"
import { Resource } from "$lib/types"

export let resource: Resource

const dispatch = createEventDispatcher()

$: tracked = resource.id != undefined

async function toggleTrackResource() {
  const command = tracked ? "untrack_resource" : "track_resource"
  resource = await invoke<Resource>(command, { resource })
  dispatch("toggle", {})
}
</script>

<div class="flex flex-col gap-3 justify-end w-[200px] h-[300px]">
  <div class="rounded-sm flex bg-secondary size-full items-center justify-center p-1">
    <div class="rounded-sm bg-contain bg-center bg-no-repeat size-full" style="background-image: url('{resource.thumbnail || "default.webp"}')" />
  </div>
  <div class="flex justify-start gap-2 items-center h-[10%]">
    <div class="flex text-white">
      <!-- <Button on:click={toggleTrackResource} size="icon" class="absolute start-3 top-1/2 size-6 -translate-y-1/2 text-muted-foreground bg-transparent"> -->
      <Button on:click={toggleTrackResource} size="icon" class="border-4 border-r-1 border-primary rounded-r-none w-8 h-8 {tracked ? 'text-secondary' : ''}">
        <Bookmark fill={tracked ? "var(--secondary)" : "none"} class="hover:bg-white rounded-sm {tracked ? 'hover:text-white' : 'hover:text-brown'}" />
      </Button>
      <Button on:click={toggleTrackResource} size="icon" class="rounded-l-none w-4 h-8 hover:bg-secondary">
        <ChevronDown />
      </Button>
    </div>
    <p class="text-xs line-clamp-3 truncate-text hover:overflow-hidden hover:line-clamp-none hover:animate-[marquee_5s_linear_infinite]">{resource.title} ({resource.year_published})</p>
    <!-- <Combobox.Root multiple>
         <div class="relative">
         <Combobox.Input
         class="inline-flex h-input w-[296px] truncate rounded-9px border rounded-xl border-border-input bg-background px-11 py-3 text-sm transition-colors placeholder:text-foreground-alt/50 focus:outline-none focus:ring-2 focus:ring-foreground focus:ring-offset-2 focus:ring-offset-background"
         aria-label="Search lists"
         />
         <ChevronsUpDown class="absolute end-3 top-1/2 size-6 -translate-y-1/2 text-muted-foreground" />
         </div>
         <Combobox.Content sideOffset={8} class="w-full rounded-xl border border-muted bg-primary px-1 py-3 shadow-popover outline-none">
         <Combobox.Item class="flex h-10 w-full select-none items-center rounded-xl rounded-button py-3 pl-5 pr-1.5 text-sm capitalize outline-none transition-all duration-75 data-[highlighted]:bg-muted" on:click={() => tracked = !tracked}>
         {tracked ? "Tracked" : "Add to Collection"}
         <Combobox.ItemIndicator class="ml-auto" asChild={false}><Check /></Combobox.ItemIndicator>
         </Combobox.Item>
         </Combobox.Content>
         </Combobox.Root> -->
  </div>
</div>
