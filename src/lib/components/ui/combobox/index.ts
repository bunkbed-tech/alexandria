import { Combobox as ComboboxPrimitive } from "bits-ui"

import Content from "./combobox-content.svelte"

const Root = ComboboxPrimitive.Root
const Item = ComboboxPrimitive.Item
const Input = ComboboxPrimitive.Input
const Label = ComboboxPrimitive.Label
const HiddenInput = ComboboxPrimitive.HiddenInput
const Arrow = ComboboxPrimitive.Arrow
const ItemIndicator = ComboboxPrimitive.ItemIndicator
const Separator = ComboboxPrimitive.Separator

export {
    Root,
    Content,
    Item,
    Input,
    Label,
    HiddenInput,
    Arrow,
    ItemIndicator,
    //
    Root as ComboboxRoot,
    Content as ComboboxContent,
    Item as ComboboxItem,
    Input as ComboboxInput,
    Label as ComboboxLabel,
    HiddenInput as ComboboxHiddenInput,
    Arrow as ComboboxArrow,
    ItemIndicator as ComboboxItemIndicator,
    Separator as ComboboxSeparator,
}
