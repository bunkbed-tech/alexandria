<script lang="ts">
import { flyAndScale } from "$lib/utils";
import type { Combobox as ComboboxPrimitive } from "bits-ui";

type $$Props = ComboboxPrimitive.ContentProps;

export const transition: $$Props["transition"] = flyAndScale;
</script>

<ComboboxPrimitive.Content
    {transition}
    {...$$restProps}
    on:keydown
    on:pointerleave
>
    <slot />
</ComboboxPrimitive.Content>
