<script lang="ts">
	import * as Accordion from '$lib/components/ui/accordion';
	import type { Channel } from '$lib/commands';

	export let channels: Channel[];

	// Group channels by category
	$: channelsByCategory = channels.reduce((acc, channel) => {
		// Get or create the category in the map
		const categoryId = channel.category_id || 'uncategorized';
		const category = acc.get(categoryId) || {
			name: channel.category_name || 'Uncategorized',
			channels: []
		};
		category.channels.push(channel);
		acc.set(categoryId, category);
		return acc;
	}, new Map<string, { name: string; channels: Channel[] }>());

	// Convert to array and sort by category name
	$: categories = Array.from(channelsByCategory.entries())
		.map(([id, data]) => ({
			id,
			name: data.name,
			channels: data.channels.sort((a, b) => a.name.localeCompare(b.name))
		}))
		.sort((a, b) => {
			// Put Uncategorized at the end
			if (a.name === 'Uncategorized') return 1;
			if (b.name === 'Uncategorized') return -1;
			return a.name.localeCompare(b.name);
		});
</script>

<div class="w-full max-w-3xl mx-auto space-y-2">
	<Accordion.Root type="single">
		{#each categories as category (category.id)}
			<Accordion.Item value={category.id}>
				<Accordion.Trigger class="flex justify-between items-center w-full">
					<span class="text-lg font-semibold">{category.name}</span>
					<span class="text-sm text-muted-foreground">({category.channels.length})</span>
				</Accordion.Trigger>
				<Accordion.Content>
					<div class="space-y-2 p-4">
						{#each category.channels as channel (channel.stream_id)}
							<div
								class="border rounded-lg p-3 bg-white/50 dark:bg-gray-700/50 hover:bg-white/70 dark:hover:bg-gray-600/50 transition-colors"
							>
								<div class="font-medium">{channel.name}</div>
							</div>
						{/each}
					</div>
				</Accordion.Content>
			</Accordion.Item>
		{/each}
	</Accordion.Root>
</div>
