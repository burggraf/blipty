<script lang="ts">
	import * as Accordion from '$lib/components/ui/accordion';
	import type { Channel, Playlist } from '$lib/commands';
	import { setSelectedChannel, getSelectedChannel, getPlaylists } from '$lib/commands';
	import { selectedPlaylist, selectedChannel as selectedChannelStore } from '$lib/stores';
	import VideoPlayer from './video-player.svelte';

	const { channels, playlist_id } = $props<{
		channels: Channel[];
		playlist_id: number;
	}>();

	let currentPlaylist = $state<Playlist | null>(null);

	// Currently selected channel
	let selectedChannel = $state<Channel | null>(null);
	// Track store value locally
	let storeValue = $state<Channel | null>(null);

	// Subscribe to the store and update local state
	$effect(() => {
		const unsubscribe = selectedChannelStore.subscribe((value) => {
			storeValue = value;
		});

		return unsubscribe;
	});

	// Load initial data when playlist_id is available
	function loadInitialData() {
		if (playlist_id) {
			loadPlaylistInfo()
				.then(() => {
					loadSelectedChannel().catch((error) => {
						console.error('Error loading selected channel:', error);
					});
				})
				.catch((error) => {
					console.error('Error loading playlist info:', error);
				});
		}
	}

	$effect(() => {
		loadInitialData();
	});

	async function loadPlaylistInfo() {
		try {
			const playlists = await getPlaylists();
			console.log('All playlists:', playlists);
			console.log('Looking for playlist ID:', playlist_id);
			currentPlaylist = playlists.find((p) => p.id === playlist_id) || null;
			console.log('Found playlist:', currentPlaylist);
		} catch (error) {
			console.error('Error loading playlist info:', error);
		}
	}

	function getAuthenticatedStreamUrl(streamUrl: string): string {
		console.log('Getting authenticated stream URL');
		console.log('Current playlist:', currentPlaylist);
		console.log('Original stream URL:', streamUrl);

		if (!currentPlaylist) {
			console.log('No playlist available, returning original URL');
			return streamUrl;
		}

		try {
			const url = new URL(streamUrl);

			// Check if URL already has username/password parameters
			if (url.searchParams.has('username') || url.searchParams.has('password')) {
				console.log('URL already has credentials, using as is');
				return streamUrl;
			}

			// Add credentials as query parameters
			url.searchParams.set('username', currentPlaylist.username);
			url.searchParams.set('password', currentPlaylist.password);

			const authenticatedUrl = url.toString();
			console.log('Authenticated URL created:', authenticatedUrl);
			return authenticatedUrl;
		} catch (error) {
			console.error('Error adding authentication to URL:', error);
			console.error('Error details:', error instanceof Error ? error.message : String(error));
			console.log('Falling back to original URL:', streamUrl);
			return streamUrl;
		}
	}

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
	let channelsByCategory = $derived(
		channels.reduce((acc: any, channel: any) => {
			// Get or create the category in the map
			const categoryId = channel.category_id || 'uncategorized';
			const category = acc.get(categoryId) || {
				name: channel.category_name || 'Uncategorized',
				channels: []
			};
			category.channels.push(channel);
			acc.set(categoryId, category);
			return acc;
		}, new Map<string, { name: string; channels: Channel[] }>())
	);

	// Convert to array and sort by category name
	let categories = $derived(
		Array.from(channelsByCategory.entries())
			.map((entry) => {
				const [id, category] = entry as [string, { name: string; channels: Channel[] }];
				return {
					id,
					name: category.name,
					channels: category.channels.sort((a, b) => a.name.localeCompare(b.name))
				};
			})
			.sort((a, b) => a.name.localeCompare(b.name))
	);

	async function handleChannelClick(channel: Channel) {
		try {
			console.log('Channel clicked:', channel);

			// Add authentication to stream URL
			if (channel.stream_url) {
				channel.authenticated_stream_url = getAuthenticatedStreamUrl(channel.stream_url);
				console.log('Authenticated URL set:', channel.authenticated_stream_url);
			}

			// Update local state
			selectedChannel = channel;

			// Update the store with the selected channel
			selectedChannelStore.set(channel);
			console.log('Updated selectedChannelStore with:', channel);

			// Save selection to backend
			await setSelectedChannel(playlist_id, channel.id!);
			console.log('Selected channel saved to backend');
		} catch (error) {
			console.error('Error selecting channel:', error);
		}
	}
</script>

<div class="space-y-4 max-h-[60vh] overflow-y-auto pr-2">
	{#each categories as category}
		<Accordion.Root type="single">
			<Accordion.Item value={category.id}>
				<Accordion.Trigger class="text-lg font-medium">
					{category.name} ({category.channels.length})
				</Accordion.Trigger>
				<Accordion.Content>
					<div class="space-y-2 mt-2">
						{#each category.channels as channel}
							<button
								class="w-full text-left p-2 rounded-md hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors duration-200 flex items-center space-x-2 {selectedChannel?.id ===
								channel.id
									? 'bg-indigo-100 dark:bg-indigo-900/30'
									: ''}"
								on:click={() => handleChannelClick(channel)}
							>
								<span class="truncate">{channel.name}</span>
							</button>
						{/each}
					</div>
				</Accordion.Content>
			</Accordion.Item>
		</Accordion.Root>
	{/each}
</div>
