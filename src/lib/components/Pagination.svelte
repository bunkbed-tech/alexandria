<script lang="ts">
import * as Pagination from "$lib/components/ui/pagination"

export let count: number
export let perPage: number
export let page: number

$: count = count === 0 ? 1 : count
</script>

<Pagination.Root {count} {perPage} let:pages let:currentPage bind:page>
  <Pagination.Content>
    <Pagination.Item>
      <Pagination.PrevButton />
    </Pagination.Item>
    {#each pages as page (page.key)}
      {#if page.type === "ellipsis"}
        <Pagination.Item>
          <Pagination.Ellipsis />
        </Pagination.Item>
      {:else}
        <Pagination.Item>
          <Pagination.Link {page} isActive={currentPage == page.value}>
            {page.value}
          </Pagination.Link>
        </Pagination.Item>
      {/if}
    {/each}
    <Pagination.Item>
      <Pagination.NextButton />
    </Pagination.Item>
  </Pagination.Content>
</Pagination.Root>
