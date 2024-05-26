<script lang="ts">
import { Check } from "lucide-svelte"

import * as Combobox from "$lib/components/ui/combobox"
import { Resource } from "$lib/types"

let owned = false
let wantToOwn = false
let wantToTry = false

export let resource: Resource
</script>

<div class="flex flex-col gap-3 justify-end">
    <img src={resource.thumbnail} alt="thumbnail" />
    <p class="max-w-[200px] whitespace-nowrap overflow-hidden text-center text-ellipsis">{resource.title} ({resource.year_published})</p>
    <Combobox.Root multiple>
      <Combobox.Input />
      <Combobox.Content>
        <Combobox.Item on:click={() => owned = !owned}>
          {owned ? "Owned" : "Add to Collection"}
          <Combobox.ItemIndicator><Check /></Combobox.ItemIndicator>
        </Combobox.Item>
        <Combobox.Item on:click={() => wantToOwn = !wantToOwn}>
          {wantToOwn ? "Wishlist" : "Want to Own?"}
          <Combobox.ItemIndicator><Check /></Combobox.ItemIndicator>
        </Combobox.Item>
        <Combobox.Item on:click={() => wantToTry = !wantToTry}>
          {wantToTry ? "Want to Try" : "Want to Try?"}
          <Combobox.ItemIndicator><Check /></Combobox.ItemIndicator>
        </Combobox.Item>
      </Combobox.Content>
    </Combobox.Root>

</div>
