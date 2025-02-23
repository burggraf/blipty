<script lang="ts">
	import * as Accordion from '$lib/components/ui/accordion';
	import type { Channel } from '$lib/commands';
	import { setSelectedChannel, getSelectedChannel } from '$lib/commands';
	import VideoPlayer from './video-player.svelte';
	import { onMount } from 'svelte';

	export let channels: Channel[];
	export let playlist_id: number;

	// Currently selected channel
	let selectedChannel: Channel | null = null;

	onMount(() => {
		if (playlist_id) {
			loadSelectedChannel();
		}
	});

	// Load selected channel
	async function loadSelectedChannel() {
		try {
			console.log('Loading selected channel for playlist:', playlist_id);
			selectedChannel = await getSelectedChannel(playlist_id);
			console.log('Selected channel:', selectedChannel);
		} catch (error) {
			console.error('Error loading selected channel:', error);
		}
	}

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

	async function handleChannelClick(channel: Channel) {
		if (channel.id) {
			try {
				console.log('Setting selected channel:', channel.id, 'for playlist:', playlist_id);
				await setSelectedChannel(playlist_id, channel.id);
				selectedChannel = channel;
			} catch (error) {
				console.error('Error setting selected channel:', error);
			}
		}
	}
</script>

<div class="w-full max-w-3xl mx-auto space-y-4">
	{#if selectedChannel}
		<div class="w-full">
			<VideoPlayer src={selectedChannel.stream_url} />
			<div class="mt-2 text-lg font-semibold">{selectedChannel.name}</div>
		</div>
	{/if}

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
							<button
								class="w-full text-left border rounded-lg p-3 bg-white/50 dark:bg-gray-700/50 hover:bg-white/70 dark:hover:bg-gray-600/50 transition-colors"
								class:ring-2={selectedChannel?.id === channel.id}
								class:ring-primary={selectedChannel?.id === channel.id}
								on:click={() => handleChannelClick(channel)}
							>
								<div class="font-medium">{channel.name}</div>
							</button>
						{/each}
					</div>
				</Accordion.Content>
			</Accordion.Item>
		{/each}
	</Accordion.Root>
</div>
