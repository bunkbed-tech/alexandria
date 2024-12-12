<script lang="ts">
import { type Props, buttonVariants } from "$lib/components/ui/button/index.js";
import { cn } from "$lib/utils.js";
import type { Pagination as PaginationPrimitive } from "bits-ui";

type $$Props = PaginationPrimitive.PageProps &
	Props & {
		isActive: boolean;
	};

type $$Events = PaginationPrimitive.PageEvents;

const className: $$Props["class"] = undefined;
export let page: $$Props["page"];
export const size: $$Props["size"] = "icon";
export const isActive: $$Props["isActive"] = false;

export { className as class };
</script>

<PaginationPrimitive.Page
	bind:page
	class={cn(
		buttonVariants({
			variant: isActive ? "outline" : "ghost",
			size,
		}),
		className
	)}
	{...$$restProps}
	on:click
>
	<slot>{page.value}</slot>
</PaginationPrimitive.Page>
