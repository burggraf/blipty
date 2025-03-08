<script lang="ts">
	import { onMount } from 'svelte';
	import * as Accordion from '$lib/components/ui/accordion';
	import type { Channel, Playlist } from '$lib/commands';
	import { setSelectedChannel, getSelectedChannel, getPlaylists } from '$lib/commands';
	import { selectedPlaylist, selectedChannel as selectedChannelStore } from '$lib/stores';
	import VideoPlayer from './video-player.svelte';

	interface CategoryGroup {
		name: string;
		channels: Channel[];
	}

	interface ContentTypeGroup {
		name: string;
		categories: CategoryGroup[];
	}

	const { channels, playlist_id } = $props<{
		channels: Channel[];
		playlist_id: number;
	}>();

	let currentPlaylist = $state<Playlist | null>(null);
	let selectedChannel = $state<Channel | null>(null);
	let storeValue = $state<Channel | null>(null);
	let allChannels = $state<Channel[]>([]);

	// Function to get all channels in a flat array for easy iteration
	function getAllChannels() {
		const flatChannels: Channel[] = [];
		contentTypes.forEach((contentType) => {
			contentType.categories.forEach((category) => {
				flatChannels.push(...category.channels);
			});
		});
		return flatChannels;
	}

	async function switchToChannel(channel: Channel) {
		try {
			if (channel.stream_url) {
				channel.authenticated_stream_url = getAuthenticatedStreamUrl(channel.stream_url);
			}
			selectedChannel = channel;
			selectedChannelStore.set(channel);
			await setSelectedChannel(playlist_id, channel.id!);
		} catch (error) {
			console.error('Error switching channel:', error);
		}
	}

	async function nextChannel() {
		const channels = getAllChannels();
		const currentIndex = channels.findIndex((ch) => ch.id === selectedChannel?.id);
		if (currentIndex > -1 && currentIndex < channels.length - 1) {
			await switchToChannel(channels[currentIndex + 1]);
		}
	}

	async function previousChannel() {
		const channels = getAllChannels();
		const currentIndex = channels.findIndex((ch) => ch.id === selectedChannel?.id);
		if (currentIndex > 0) {
			await switchToChannel(channels[currentIndex - 1]);
		}
	}

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
		// Update allChannels when channels prop changes
		allChannels = getAllChannels();
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
			const channel = await getSelectedChannel(playlist_id);
			if (channel) {
				selectedChannel = channel;
				selectedChannelStore.set(channel);
			}
		} catch (error) {
			console.error('Error loading selected channel:', error);
		}
	}

	// Group channels by content_type and then by category
	let channelsByContentType = $derived(
		channels.reduce((acc: Map<string, Map<string, CategoryGroup>>, channel: Channel) => {
			const contentType = channel.stream_type || 'live';
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
		}, new Map<string, Map<string, CategoryGroup>>())
	);

	// Convert to array and sort
	let contentTypes = $derived(
		(Array.from(channelsByContentType.entries()) as [string, Map<string, CategoryGroup>][])
			.map(([contentType, categories]: [string, Map<string, CategoryGroup>]) => ({
				name: contentType.charAt(0).toUpperCase() + contentType.slice(1),
				categories: Array.from(categories.entries())
					.map(([id, category]: [string, CategoryGroup]) => ({
						id,
						name: category.name,
						channels: category.channels.sort((a: Channel, b: Channel) =>
							a.name.localeCompare(b.name)
						)
					}))
					.sort((a: { name: string }, b: { name: string }) => a.name.localeCompare(b.name))
			}))
			.sort((a: { name: string }, b: { name: string }) => a.name.localeCompare(b.name))
	);

	// Modify handleChannelClick to use switchToChannel
	async function handleChannelClick(channel: Channel) {
		await switchToChannel(channel);
	}

	// Add keyboard event listener for channel switching
	onMount(() => {
		const handleKeydown = async (event: KeyboardEvent) => {
			if (event.key === 'ArrowUp') {
				await previousChannel();
				event.preventDefault();
			} else if (event.key === 'ArrowDown') {
				await nextChannel();
				event.preventDefault();
			}
		};

		document.addEventListener('keydown', handleKeydown);
		return () => {
			document.removeEventListener('keydown', handleKeydown);
		};
	});
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
