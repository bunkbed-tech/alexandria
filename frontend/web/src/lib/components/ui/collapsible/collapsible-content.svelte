<script lang="ts">
import type { Collapsible as CollapsiblePrimitive } from "bits-ui";
import { slide } from "svelte/transition";

type $$Props = CollapsiblePrimitive.ContentProps;

export const transition: $$Props["transition"] = slide;
export const transitionConfig: $$Props["transitionConfig"] = {
	duration: 150,
};
</script>

<CollapsiblePrimitive.Content {transition} {transitionConfig} {...$$restProps}>
	<slot />
</CollapsiblePrimitive.Content>
