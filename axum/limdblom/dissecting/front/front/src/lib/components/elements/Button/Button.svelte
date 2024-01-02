<script lang="ts">
    import type { HTMLButtonAttributes } from "svelte/elements";
    import { cva, type VariantProps } from "class-variance-authority";
    import {cn} from "$utils/cn"
    import {createEventDispatcher} from "svelte";

    const button = cva("button", {
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

    interface $$Props extends HTMLButtonAttributes, VariantProps<typeof button> {}

    /**
     * For Svelte components, we recommend setting your defaultVariants within
     * Svelte props (which are `undefined` by default)
     */
    export let intent: $$Props["intent"] = "primary";
    export let size: $$Props["size"] = "medium";
</script>

<button {...$$props} class={cn(button({ intent, size, class: $$props.class }))} on:click={handleClick}>
    <slot />
</button>

<style>

</style>