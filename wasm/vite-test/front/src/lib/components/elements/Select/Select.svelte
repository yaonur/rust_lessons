<script lang="ts">
    import type { HTMLButtonAttributes } from "svelte/elements";
    import { cva, type VariantProps } from "class-variance-authority";
    import {cn} from "$utils/cn"
    import {createEventDispatcher} from "svelte";

    const select = cva("select", {
        variants: {
            intent: {
                primary: "bg-primary",
                secondary: "bg-secondary",
            },
            size: {
                small: "w-20 h-10",
                medium: "w-26 h-14  ",
            },
        },
        compoundVariants: [
            { intent: "primary", size: "medium", class: "primaryMedium" },
        ],
    });
    const dispatch = createEventDispatcher();

    function handleClick() {
        dispatch("click");
    }

    interface $$Props extends HTMLButtonAttributes, VariantProps<typeof select> {}

    export let intent: $$Props["intent"] = "primary";
    export let size: $$Props["size"] = "medium";
</script>

<select {...$$props} class={cn(select({ intent, size, class: $$props.class }))} on:click={handleClick}>
    <slot />
</select>

<style>

</style>