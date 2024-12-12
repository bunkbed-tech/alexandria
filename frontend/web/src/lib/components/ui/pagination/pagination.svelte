<script lang="ts">
import type { Pagination as PaginationPrimitive } from "bits-ui";

import { cn } from "$lib/utils.js";

type $$Props = PaginationPrimitive.Props;
type $$Events = PaginationPrimitive.Events;

const className: $$Props["class"] = undefined;
export const count: $$Props["count"] = 0;
export const perPage: $$Props["perPage"] = 10;
export const page: $$Props["page"] = 1;
export const siblingCount: $$Props["siblingCount"] = 1;
export { className as class };

$: currentPage = page;
</script>

<PaginationPrimitive.Root
	{count}
	{perPage}
	{siblingCount}
	bind:page
	let:builder
	let:pages
	let:range
	asChild
	{...$$restProps}
>
	<nav {...builder} class={cn("mx-auto flex w-full flex-col items-center", className)}>
		<slot {pages} {range} {currentPage} />
	</nav>
</PaginationPrimitive.Root>
