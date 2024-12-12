<script lang="ts">
import { cn } from "$lib/utils.js";
import type { Slider as SliderPrimitive } from "bits-ui";

type $$Props = SliderPrimitive.Props;

const className: $$Props["class"] = undefined;
export const value: $$Props["value"] = [0];
export { className as class };
</script>

<SliderPrimitive.Root
	bind:value
	class={cn("relative flex w-full touch-none select-none items-center mb-2", className)}
	{...$$restProps}
	let:thumbs
	let:ticks
>
	<span class="bg-secondary relative h-2 w-full grow overflow-hidden rounded-full">
		<SliderPrimitive.Range class="bg-primary absolute h-full" />
	</span>
	{#each ticks as tick}
		<SliderPrimitive.Tick {tick} asChild let:builder>
			<span use:builder.action {...builder} class="text-sm flex flex-col items-center w-1 h-3 rounded-full data-[bounded=true]:bg-secondary bg-primary cursor-pointer"><br />{builder["data-value"]}</span>
		</SliderPrimitive.Tick>
    {/each}
	{#each thumbs as thumb}
		<SliderPrimitive.Thumb
			{thumb}
			class="border-primary bg-background ring-offset-background focus-visible:ring-ring block h-5 w-5 rounded-full border-2 transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50"
		/>
	{/each}
</SliderPrimitive.Root>
