<script lang="ts">
import { cn } from "$lib/utils.js";
import type { HTMLAttributes } from "svelte/elements";
import { type Variant, alertVariants } from "./index.js";

type $$Props = HTMLAttributes<HTMLDivElement> & {
	variant?: Variant;
};

const className: $$Props["class"] = undefined;
export const variant: $$Props["variant"] = "default";
export { className as class };
</script>

<div class={cn(alertVariants({ variant }), className)} {...$$restProps} role="alert">
	<slot />
</div>
