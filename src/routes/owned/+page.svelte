<script lang="ts">
import MediaListSearch from "$lib/components/MediaListSearch.svelte"
import * as Tabs from "$lib/components/ui/tabs"
</script>

<Tabs.Root value="board-games">
  <Tabs.List>
    <Tabs.Trigger value="board-games">Board Games</Tabs.Trigger>
    <Tabs.Trigger value="tab-2">Tab 2</Tabs.Trigger>
    <Tabs.Trigger value="tab-3">Tab 3</Tabs.Trigger>
  </Tabs.List>
  <Tabs.Content value="board-games"><MediaListSearch /></Tabs.Content>
  <Tabs.Content value="tab-2">Welcome to Tab 2</Tabs.Content>
  <Tabs.Content value="tab-3">Welcome to Tab 3</Tabs.Content>
</Tabs.Root>
