<script lang="ts">
import "../app.css"
import { Button } from "$lib/components/ui/button"
import * as Collapsible from "$lib/components/ui/collapsible"
import { ChevronDown, ChevronUp } from "lucide-svelte"
</script>

<div class="h-screen w-screen flex">
  <aside class="w-[20%] border-r-[3px]">
    <Collapsible.Root class="w-[350px] space-y-2">
      <div class="flex items-center justify-between space-x-4 px-4">
        <h4 class="text-sm font-semibold">Lists</h4>
        <Collapsible.Trigger asChild let:builder>
          <Button builders={[builder]} variant="ghost" size="sm" class="w-9 p-0">
            <ChevronDown class="h-4 w-4" />
            <span class="sr-only">Toggle</span>
          </Button>
        </Collapsible.Trigger>
      </div>
      <div class="rounded-md border px-4 py-3 font-mono text-sm">
        @huntabyte/bits-ui
      </div>
      <Collapsible.Content class="space-y-2">
        <div class="rounded-md border px-4 py-3 font-mono text-sm">
          @melt-ui/melt-ui
        </div>
        <div class="rounded-md border px-4 py-3 font-mono text-sm">
          @sveltejs/svelte
        </div>
      </Collapsible.Content>
    </Collapsible.Root>
  </aside>
  <slot></slot>
</div>
