<!-- ShortcutHelp.svelte -->
<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import * as Dialog from '$lib/components/ui/dialog';
	import { X } from 'lucide-svelte';
	import { cn } from '$lib/utils';

	const shortcuts = [
		{ key: 'Space/K', action: 'Play/Pause' },
		{ key: 'M', action: 'Mute/Unmute' },
		{ key: 'F', action: 'Toggle Fullscreen' },
		{ key: '←', action: 'Seek -10s' },
		{ key: '→', action: 'Seek +10s' },
		{ key: '↑', action: 'Volume +10%' },
		{ key: '↓', action: 'Volume -10%' },
		{ key: 'H', action: 'Show/Hide Help' }
	];

	let { open = $bindable(false) } = $props();
</script>

<Dialog.Root {open} on:change={(e) => (open = e.detail)} class={cn('')}>
	<Dialog.Content class={cn('sm:max-w-md')}>
		<Dialog.Header class="space-y-2">
			<Dialog.Title class="text-lg font-semibold">Keyboard Shortcuts</Dialog.Title>
			<Dialog.Description class="text-sm text-muted-foreground">
				These shortcuts are available when the video player is focused.
			</Dialog.Description>
		</Dialog.Header>
		<div class="grid grid-cols-2 gap-4">
			{#each shortcuts as { key, action }}
				<div class="flex items-center justify-between">
					<kbd class="bg-muted text-muted-foreground rounded px-2 py-1 text-sm">{key}</kbd>
					<span class="text-sm">{action}</span>
				</div>
			{/each}
		</div>
		<Dialog.Footer class="flex justify-end">
			<Button variant="outline" onclick={() => (open = false)}>Close</Button>
		</Dialog.Footer>
	</Dialog.Content>
</Dialog.Root>
