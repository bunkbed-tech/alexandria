<script lang="ts">
import Fuse from "fuse.js";
import { ArrowDownAZ, ArrowDownWideNarrow, ArrowUpAZ } from "lucide-svelte";

import Pagination from "$lib/components/Pagination.svelte";
import ResourceCards from "$lib/components/ResourceCards.svelte";
import { Button } from "$lib/components/ui/button";
import * as Dialog from "$lib/components/ui/dialog";
import { Input } from "$lib/components/ui/input";
import { Label } from "$lib/components/ui/label";
import * as Select from "$lib/components/ui/select";
import { Skeleton } from "$lib/components/ui/skeleton";
import { Slider } from "$lib/components/ui/slider";
import * as Tabs from "$lib/components/ui/tabs";
import type { Resource } from "$lib/types";

const perPage = 20;
const sorters = ["default", "alphabetical", "year", "tracked"] as const;
type Sorter = (typeof sorters)[number];
const sorterCompareFns: Record<Sorter, (a: Resource, b: Resource) => number> = {
	default: (_a, _b) => 0,
	alphabetical: (a, b) => {
		// Uppercase everything to be case-insensitive
		const titleA = a.title.toUpperCase();
		const titleB = b.title.toUpperCase();
		if (titleA < titleB) return -1;
		if (titleA > titleB) return 1;
		return 0;
	},
	year: (a, b) =>
		(a.year_published || Number.POSITIVE_INFINITY) -
		(b.year_published || Number.POSITIVE_INFINITY),
	tracked: (a, b) => +!!b.id - +!!a.id,
};

export const filterPageResources: (_: Resource[]) => Resource[] = (a) => a;
export const searchOnMount = false;
export let searcher: () => Promise<Resource[]>;
export const query = "";
export let title: string;

const promise: Promise<void> | null = searchOnMount ? searchAndFilter() : null;
const page = 1;
let resources: Resource[] = [];
let filteredResources: Resource[] = [];
const sort = { value: "default" as Sorter };
const reverseOrder = false;

$: min = resources.reduce(
	(min, resource) =>
		resource.year_published && resource.year_published < min
			? resource.year_published
			: min,
	Number.POSITIVE_INFINITY,
);
$: max = resources.reduce(
	(max, resource) =>
		resource.year_published && resource.year_published > max
			? resource.year_published
			: max,
	Number.NEGATIVE_INFINITY,
);
$: yearPublishedRange = [min, max] as [number, number];
$: {
	filteredResources = resources
		// Year published within slider range (keep all nulls)
		.filter(
			(result) =>
				result.year_published == null ||
				(result.year_published >= yearPublishedRange[0] &&
					result.year_published <= yearPublishedRange[1]),
		)
		// Sort the results by the specified feature
		.sort(sorterCompareFns[sort.value]);
	filteredResources = reverseOrder
		? filteredResources.reverse()
		: filteredResources;
}
$: pageResources = filteredResources.slice(
	perPage * (page - 1),
	perPage * page,
);

async function searchAndFilter() {
	const endpoint_resources = await searcher();

	if (query === "") {
		resources = endpoint_resources;
	} else {
		// Fuzzy search
		const options = {
			includeScore: true,
			keys: [
				{
					name: "title",
					weight: 0.9,
				},
				{
					name: "description",
					weight: 0.1,
				},
			],
		};
		const fuse = new Fuse(endpoint_resources, options);
		resources = fuse.search(query).map((result) => result.item);
	}
}

function onToggleResource() {
	resources = [
		...resources.slice(0, perPage * (page - 1)),
		...filterPageResources(pageResources),
		...resources.slice(perPage * page),
	];
}
</script>

<Dialog.Root>
  <Tabs.Root value="board-games">
    <div class="flex justify-between items-center">
      <Tabs.List class="bg-primary text-white">
        <Tabs.Trigger value="board-games" class="data-[state=active]:bg-secondary data-[state=active]:text-white data-[state=active]:hover:text-brown data-[state=active]:hover:bg-white hover:text-brown hover:bg-white">Board Games</Tabs.Trigger>
        <Tabs.Trigger value="video-games" class="data-[state=active]:bg-secondary data-[state=active]:text-white hover:text-brown hover:bg-white">Video Games</Tabs.Trigger>
        <Tabs.Trigger value="books" class="data-[state=active]:bg-secondary data-[state=active]:text-white hover:text-brown hover:bg-white">Books</Tabs.Trigger>
      </Tabs.List>
      <p class="text-4xl">{title}</p>
      <div class="flex gap-2">
        <Dialog.Trigger><Button size="icon" class="text-white hover:bg-secondary" disabled={resources.length == 0}><ArrowDownWideNarrow /></Button></Dialog.Trigger>
        <form class="flex gap-2" on:submit={() => promise = searchAndFilter()}>
          <Input bind:value={query} placeholder="Enter a query ..." />
          <Button type="submit" class="text-white hover:bg-secondary">Search</Button>
        </form>
      </div>
    </div>
    <Tabs.Content value="board-games">
      {#if promise}
        {#await promise}
          <Skeleton class="h-4 w-[200px]" />
        {:then}
          {#if resources.length != 0}
            <Dialog.Content>
              <Dialog.Header>
                <Dialog.Title>Refine Results</Dialog.Title>
                <Dialog.Description>Filter and sort resources by different attributes.</Dialog.Description>
              </Dialog.Header>
              <Select.Root bind:selected={sort}>
                <div class="flex items-center justify-between">
                  <Label for="select">Sorting Algorithm</Label>
                  <div class="flex items-center gap-2">
                    <Button size="icon" class="text-white hover:bg-secondary" on:click={() => reverseOrder = !reverseOrder}>
                      {#if reverseOrder}
                        <ArrowUpAZ />
                      {:else}
                        <ArrowDownAZ />
                      {/if}
                    </Button>
                    <Select.Trigger class="w-[180px]">
                      <Select.Value id="select" placeholder="Default" />
                    </Select.Trigger>
                  </div>
                </div>
                <Select.Content>
                  <Select.Item value="default" label="Default" />
                  <Select.Item value="alphabetical" label="Alphabetical" />
                  <Select.Item value="year" label="Year Published" />
                  <Select.Item value="tracked" label="Tracking Status" />
                </Select.Content>
              </Select.Root>
              <Label for="slider">Year Published</Label>
              <Slider id="slider" bind:value={yearPublishedRange} {min} {max} step={Math.round((max - min) / 10)} />
            </Dialog.Content>
          {/if}
          <Pagination count={filteredResources.length} {perPage} bind:page />
          <ResourceCards bind:resources={pageResources} on:toggle={onToggleResource} />
        {:catch error}
          <p>Error: {error}</p>
        {/await}
      {/if}
    </Tabs.Content>
    <Tabs.Content value="tab-2">Welcome to Tab 2</Tabs.Content>
    <Tabs.Content value="tab-3">Welcome to Tab 3</Tabs.Content>
  </Tabs.Root>
</Dialog.Root>
