<script lang="ts">
import { invoke } from "@tauri-apps/api/core"
import { Bookmark, Check, ChevronsUpDown } from "lucide-svelte"

import { Button } from "$lib/components/ui/button"
import * as Combobox from "$lib/components/ui/combobox"
import { Resource } from "$lib/types"

export let resource: Resource

let wantToOwn = false
let wantToTry = false
$: tracked = resource.id != undefined

async function toggleTrackResource() {
  const command = tracked ? "untrack_resource" : "track_resource"
  resource = await invoke<Resource>(command, { resource })
}
</script>

<div class="flex flex-col gap-3 justify-end">
  <img src={resource.thumbnail} alt="thumbnail" />
  <p class="max-w-[200px] whitespace-nowrap overflow-hidden text-center text-ellipsis">{resource.title} ({resource.year_published})</p>
  <Combobox.Root multiple>
    <div class="relative">
      <!-- <Bookmark class="absolute start-3 top-1/2 size-6 -translate-y-1/2 text-muted-foreground" /> -->
      <Button on:click={toggleTrackResource} size="icon" class="absolute start-3 top-1/2 size-6 -translate-y-1/2 text-muted-foreground bg-transparent">
        <Bookmark fill={tracked ? "var(--foreground)" : "none"} />
      </Button>
      <Combobox.Input
        class="inline-flex h-input w-[296px] truncate rounded-9px border rounded-xl border-border-input bg-background px-11 py-3 text-sm transition-colors placeholder:text-foreground-alt/50 focus:outline-none focus:ring-2 focus:ring-foreground focus:ring-offset-2 focus:ring-offset-background"
        placeholder="Search lists"
        aria-label="Search lists"
      />
      <ChevronsUpDown class="absolute end-3 top-1/2 size-6 -translate-y-1/2 text-muted-foreground" />
    </div>
    <Combobox.Content sideOffset={8} class="w-full rounded-xl border border-muted bg-primary px-1 py-3 shadow-popover outline-none">
      <Combobox.Item class="flex h-10 w-full select-none items-center rounded-xl rounded-button py-3 pl-5 pr-1.5 text-sm capitalize outline-none transition-all duration-75 data-[highlighted]:bg-muted" on:click={() => tracked = !tracked}>
        {tracked ? "Tracked" : "Add to Collection"}
        <Combobox.ItemIndicator class="ml-auto" asChild={false}><Check /></Combobox.ItemIndicator>
      </Combobox.Item>
      <Combobox.Item class="flex h-10 w-full select-none items-center rounded-xl rounded-button py-3 pl-5 pr-1.5 text-sm capitalize outline-none transition-all duration-75 data-[highlighted]:bg-muted" on:click={() => wantToOwn = !wantToOwn}>
        {wantToOwn ? "Wishlist" : "Want to Own?"}
        <Combobox.ItemIndicator class="ml-auto" asChild={false}><Check /></Combobox.ItemIndicator>
      </Combobox.Item>
      <Combobox.Item class="flex h-10 w-full select-none items-center rounded-xl rounded-button py-3 pl-5 pr-1.5 text-sm capitalize outline-none transition-all duration-75 data-[highlighted]:bg-muted" on:click={() => wantToTry = !wantToTry}>
        {wantToTry ? "Want to Try" : "Want to Try?"}
        <Combobox.ItemIndicator class="ml-auto" asChild={false}><Check /></Combobox.ItemIndicator>
      </Combobox.Item>
    </Combobox.Content>
  </Combobox.Root>
</div>
