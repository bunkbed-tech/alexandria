<script lang="ts">
import * as Alert from "$lib/components/ui/alert"
import CircleAlert from "lucide-svelte/icons/circle-alert"

export let error: string
</script>

<Alert.Root variant="destructive">
  <CircleAlert class="h-4 w-4" />
  <Alert.Title>Error</Alert.Title>
  <Alert.Description>{error}</Alert.Description>
</Alert.Root>
