<!-- DialogFooter.svelte -->
<script lang="ts">
	import { cn } from '$lib/utils';
	import type { HTMLAttributes } from 'svelte/elements';

	type $$Props = HTMLAttributes<HTMLDivElement>;
	let { class: className, ...props } = $props();
</script>

<div
	class={cn('flex flex-col-reverse sm:flex-row sm:justify-end sm:space-x-2', className)}
	{...props}
>
	<slot />
</div>
