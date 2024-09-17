<script lang="ts">
import { invoke } from "@tauri-apps/api/core"
import { toast } from "svelte-sonner"

import AlertError from "$lib/components/AlertError.svelte"
import MediaGrid from "$lib/components/MediaGrid.svelte"
import { Resource } from "$lib/types"

let query = ""

async function searchBggThings() {
  const [api_resources, errors] = await invoke<[Resource[], string[]]>("search_bgg_things", { query })

  // Render any errors as toasts
  for (const error of errors) {
    toast.custom(AlertError, { componentProps: { error } })
  }

  // Match with resources already tracked in database
  const db_resources = await invoke<Resource[]>("list_resources", {
    ids: api_resources.map(resource => resource.bgg_id),
  })
  const bgg_to_db_id = db_resources.reduce((acc, resource) => acc.set(resource.bgg_id, resource.id), new Map())
  const matched_resources = api_resources.map(resource => ({ ...resource, id: bgg_to_db_id.get(resource.bgg_id) }))
  return matched_resources
}
</script>

<MediaGrid searcher={searchBggThings} title="Search" bind:query />
