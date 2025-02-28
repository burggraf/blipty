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
	let selectedChannel = $state<Channel | null>(null);
	let storeValue = $state<Channel | null>(null);

	$effect(() => {
		const unsubscribe = selectedChannelStore.subscribe((value) => {
			storeValue = value;
		});
		return unsubscribe;
	});

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
			currentPlaylist = playlists.find((p) => p.id === playlist_id) || null;
		} catch (error) {
			console.error('Error loading playlist info:', error);
		}
	}

	function getAuthenticatedStreamUrl(streamUrl: string): string {
		if (!currentPlaylist) return streamUrl;

		try {
			const url = new URL(streamUrl);
			if (url.searchParams.has('username') || url.searchParams.has('password')) {
				return streamUrl;
			}
			url.searchParams.set('username', currentPlaylist.username);
			url.searchParams.set('password', currentPlaylist.password);
			return url.toString();
		} catch (error) {
			console.error('Error adding authentication to URL:', error);
			return streamUrl;
		}
	}

	async function loadSelectedChannel() {
		try {
			selectedChannel = await getSelectedChannel(playlist_id);
		} catch (error) {
			console.error('Error loading selected channel:', error);
		}
	}

	// Group channels by content_type and then by category
	let channelsByContentType = $derived(
		channels.reduce((acc, channel) => {
			const contentType = channel.content_type || 'live';
			const categoryId = channel.category_id || 'uncategorized';
			
			// Get or create the content type group
			if (!acc.has(contentType)) {
				acc.set(contentType, new Map());
			}
			const contentTypeMap = acc.get(contentType)!;

			// Get or create the category in the content type group
			if (!contentTypeMap.has(categoryId)) {
				contentTypeMap.set(categoryId, {
					name: channel.category_name || 'Uncategorized',
					channels: []
				});
			}

			// Add the channel to its category
			contentTypeMap.get(categoryId)!.channels.push(channel);
			return acc;
		}, new Map<string, Map<string, { name: string; channels: Channel[] }>>())
	);

	// Convert to array and sort
	let contentTypes = $derived(
		Array.from(channelsByContentType.entries())
			.map(([contentType, categories]) => ({
				name: contentType.charAt(0).toUpperCase() + contentType.slice(1),
				categories: Array.from(categories.entries())
					.map(([id, category]) => ({
						id,
						name: category.name,
						channels: category.channels.sort((a, b) => a.name.localeCompare(b.name))
					}))
					.sort((a, b) => a.name.localeCompare(b.name))
			}))
			.sort((a, b) => a.name.localeCompare(b.name))
	);

	async function handleChannelClick(channel: Channel) {
		try {
			if (channel.stream_url) {
				channel.authenticated_stream_url = getAuthenticatedStreamUrl(channel.stream_url);
			}
			selectedChannel = channel;
			selectedChannelStore.set(channel);
			await setSelectedChannel(playlist_id, channel.id!);
		} catch (error) {
			console.error('Error selecting channel:', error);
		}
	}
</script>

<div class="space-y-4 max-h-[60vh] overflow-y-auto pr-2">
	{#each contentTypes as contentType}
		<Accordion.Root type="single">
			<Accordion.Item value={contentType.name}>
				<Accordion.Trigger class="text-lg font-medium">
					{contentType.name}
				</Accordion.Trigger>
				<Accordion.Content>
					<div class="space-y-2 mt-2">
						{#each contentType.categories as category}
							<Accordion.Root type="single">
								<Accordion.Item value={category.id}>
									<Accordion.Trigger class="text-md font-medium pl-4">
										{category.name} ({category.channels.length})
									</Accordion.Trigger>
									<Accordion.Content>
										<div class="space-y-2 mt-2 pl-8">
											{#each category.channels as channel}
												<button
													class="w-full text-left p-2 rounded-md hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors duration-200 flex items-center space-x-2 {selectedChannel?.id ===
													channel.id
														? 'bg-indigo-100 dark:bg-indigo-900/30'
														: ''}"
													onclick={() => handleChannelClick(channel)}
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
				</Accordion.Content>
			</Accordion.Item>
		</Accordion.Root>
	{/each}
</div>
