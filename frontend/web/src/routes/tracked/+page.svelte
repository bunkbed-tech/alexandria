<script lang="ts">
import { toast } from "svelte-sonner"

import AlertError from "$lib/components/AlertError.svelte"
import MediaGrid from "$lib/components/MediaGrid.svelte"
import { Resource } from "$lib/types"

async function listResources() {
  const response = await fetch(`${import.meta.env.BASE_API_URL}/resource`)
  if (!response.ok) {
    toast.custom(AlertError, {
      componentProps: {
        error: `${response.status} ${response.statusText}: ${await response.text()}`,
      },
    })
  }
  return JSON.parse(await response.json()).map(Resource.create)
}

// This allows us to filter out the resource that was untracked for dynamic display
function filterPageResources(pageResources: Resource[]) {
  return pageResources.filter(resource => resource.id != undefined)
}
</script>

<MediaGrid searcher={listResources} title="Tracked" searchOnMount {filterPageResources} />
