<script lang="ts">
import { toast } from "svelte-sonner"

import AlertError from "$lib/components/AlertError.svelte"
import MediaGrid from "$lib/components/MediaGrid.svelte"
import { BggThingsSearch, Resource } from "$lib/types"

const query = ""

async function searchBggThings() {
  const bgg_response = await fetch(
    `${import.meta.env.BASE_API_URL}/bgg/things/search?${new URLSearchParams({
      query,
    })}`,
  )
  const { resources: api_resources, errors } = BggThingsSearch.create(JSON.parse(await bgg_response.json()))

  // Render any errors as toasts
  for (const error of errors) {
    toast.custom(AlertError, { componentProps: { error } })
  }

  // Match with resources already tracked in database
  const ids = api_resources.map(resource => resource.api_id)
  const db_response = await fetch(`${import.meta.env.BASE_API_URL}/resource?${new URLSearchParams({ ids })}`)
  if (!db_response.ok) {
    toast.custom(AlertError, {
      componentProps: {
        error: `${db_response.status} ${db_response.statusText}: ${await db_response.text()}`,
      },
    })
  }
  const db_resources = JSON.parse(await db_response.json()).map(Resource.create)
  const bgg_to_db_id = db_resources.reduce((acc, resource) => acc.set(resource.api_id, resource.id), new Map())
  const matched_resources = api_resources.map(resource => ({
    ...resource,
    id: bgg_to_db_id.get(resource.api_id),
  }))
  return matched_resources
}
</script>

<MediaGrid searcher={searchBggThings} title="Search" bind:query />
