<script lang="ts">
	import { cn } from '$utils/cn';
	import Button from "$elements/Button";

	export let disabled: boolean = false;

	function convert() {
		disabled = !disabled;
	}
</script>

<div class="flex h-20 w-40 flex-col bg-secondary">
	<input
		type="text"
		class={cn('h-20 w-20 font-bold', $$props.class, { 'bg-gray-200 font-extralight': disabled })}
		placeholder="some text"
	/>
	<button on:click={convert}>Simple Button</button>
	<Button class="rounded-3xl" on:click={convert}>Cva Button</Button>
</div>
