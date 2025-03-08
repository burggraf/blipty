<!-- Slider.svelte -->
<script lang="ts">
	import { Slider as SliderPrimitive } from 'bits-ui';
	import { cn } from '$lib/utils';

	type $$Props = SliderPrimitive.Props;

	let { class: className, ...props }: $$Props = $props();
</script>

<SliderPrimitive.Root
	class={cn('relative flex w-full touch-none select-none items-center', className)}
	{...props}
>
	<SliderPrimitive.Track class="relative h-1 w-full grow overflow-hidden rounded-full bg-black/20">
		<SliderPrimitive.Range class="absolute h-full bg-white/80" />
	</SliderPrimitive.Track>
	<SliderPrimitive.Thumb
		class="block h-3 w-3 rounded-full border border-white/80 bg-background shadow transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:pointer-events-none disabled:opacity-50"
	/>
</SliderPrimitive.Root>
