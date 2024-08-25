<script lang="ts">
import { invoke } from "@tauri-apps/api/core"
import { toast } from "svelte-sonner"

import AlertError from "$lib/components/AlertError.svelte"
import MediaGrid from "$lib/components/MediaGrid.svelte"
import { Resource } from "$lib/types"

async function listResources() {
  const maybe_resources = await invoke<Resource[] | string>("list_resources", { ids: null })

  // Render error as toast
  if (typeof maybe_resources == "string") {
    toast.custom(AlertError, { componentProps: { error: maybe_resources } })
    return []
  }
  return maybe_resources
}

// This allows us to filter out the resource that was untracked for dynamic display
function filterPageResources(pageResources: Resource[]) {
  return pageResources.filter(resource => resource.id != undefined)
}
</script>

<MediaGrid searcher={listResources} searchOnMount {filterPageResources} />
