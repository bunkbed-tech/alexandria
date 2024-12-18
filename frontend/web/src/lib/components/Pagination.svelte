<script lang="ts">
import * as Pagination from "$lib/components/ui/pagination"

export let count: number
export let perPage: number
export let page: number

$: count = count === 0 ? 1 : count
</script>

<Pagination.Root {count} {perPage} let:pages let:currentPage bind:page>
  <Pagination.Content class="my-2">
    <Pagination.Item>
      <Pagination.PrevButton class="hover:bg-primary hover:text-white" />
    </Pagination.Item>
    {#each pages as page (page.key)}
      {#if page.type === "ellipsis"}
        <Pagination.Item>
          <Pagination.Ellipsis />
        </Pagination.Item>
      {:else}
        <Pagination.Item>
          {@const isActive = currentPage == page.value}
          <Pagination.Link {page} {isActive} class="hover:bg-primary hover:text-white {isActive ? 'bg-secondary text-white' : ''}">
            {page.value}
          </Pagination.Link>
        </Pagination.Item>
      {/if}
    {/each}
    <Pagination.Item>
      <Pagination.NextButton class="hover:bg-primary hover:text-white" />
    </Pagination.Item>
  </Pagination.Content>
</Pagination.Root>
