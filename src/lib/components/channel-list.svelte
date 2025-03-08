<script lang="ts">
	import * as Accordion from '$lib/components/ui/accordion';
	import type { Channel, Playlist } from '$lib/commands';
	import {
		setSelectedChannel,
		getSelectedChannel,
		getPlaylists,
		deletePlaylist
	} from '$lib/commands';
	import { selectedPlaylist, selectedChannel as selectedChannelStore } from '$lib/stores';
	import VideoPlayer from './video-player.svelte';
	import { buttonVariants } from '$lib/components/ui/button/button.svelte';
	import { cn } from '$lib/utils';
	import { Pencil, Trash2, PlusCircle } from 'lucide-svelte';

	interface CategoryGroup {
		name: string;
		channels: Channel[];
	}

	interface ContentTypeGroup {
		name: string;
		categories: CategoryGroup[];
	}

	interface ProviderGroup {
		id: number;
		name: string;
		contentTypes: ContentTypeGroup[];
	}

	// Accept either channels for a single provider or a providers map
	const { channels, playlist_id, providers, onEditProvider, onDeleteProvider, onAddProvider } =
		$props<{
			channels?: Channel[];
			playlist_id?: number;
			providers?: Map<Playlist, Channel[]>;
			onEditProvider?: (provider: Playlist) => void;
			onDeleteProvider?: (provider: Playlist) => void;
			onAddProvider?: () => void;
		}>();

	let currentPlaylist = $state<Playlist | null>(null);
	let selectedChannel = $state<Channel | null>(null);
	let storeValue = $state<Channel | null>(null);
	let allPlaylists = $state<Playlist[]>([]);

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
			allPlaylists = playlists;
			currentPlaylist = playlists.find((p) => p.id === playlist_id) || null;
		} catch (error) {
			console.error('Error loading playlist info:', error);
		}
	}

	function getAuthenticatedStreamUrl(streamUrl: string, playlist: Playlist): string {
		try {
			const url = new URL(streamUrl);
			if (url.searchParams.has('username') || url.searchParams.has('password')) {
				return streamUrl;
			}
			url.searchParams.set('username', playlist.username);
			url.searchParams.set('password', playlist.password);
			return url.toString();
		} catch (error) {
			console.error('Error adding authentication to URL:', error);
			return streamUrl;
		}
	}

	async function loadSelectedChannel() {
		try {
			if (!playlist_id) return;

			const channel = await getSelectedChannel(playlist_id);
			if (channel) {
				selectedChannel = channel;
				selectedChannelStore.set(channel);
			}
		} catch (error) {
			console.error('Error loading selected channel:', error);
		}
	}

	// Process channels for one provider
	function processProviderChannels(channelsList: Channel[]): ContentTypeGroup[] {
		// Group channels by content_type and then by category
		const channelsByContentType = channelsList.reduce(
			(acc: Map<string, Map<string, CategoryGroup>>, channel: Channel) => {
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
			},
			new Map<string, Map<string, CategoryGroup>>()
		);

		// Convert to array and sort
		return (Array.from(channelsByContentType.entries()) as [string, Map<string, CategoryGroup>][])
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
			.sort((a: { name: string }, b: { name: string }) => a.name.localeCompare(b.name));
	}

	// For single provider mode
	let providerContentTypes = $derived(channels ? processProviderChannels(channels) : []);

	// For multi-provider mode
	let providerGroups = $derived(
		providers
			? Array.from(providers.entries()).map(([provider, providerChannels]) => ({
					id: provider.id!,
					name: provider.name,
					contentTypes: processProviderChannels(providerChannels)
				}))
			: []
	);

	async function handleChannelClick(channel: Channel, playlist: Playlist) {
		try {
			if (channel.stream_url) {
				channel.authenticated_stream_url = getAuthenticatedStreamUrl(channel.stream_url, playlist);
			}
			selectedChannel = channel;
			selectedChannelStore.set(channel);
			await setSelectedChannel(playlist.id!, channel.id!);
		} catch (error) {
			console.error('Error selecting channel:', error);
		}
	}

	function handleEdit(event: Event, provider: Playlist) {
		event.stopPropagation(); // Prevent accordion from toggling
		if (onEditProvider) {
			onEditProvider(provider);
		}
	}

	function handleDelete(event: Event, provider: Playlist) {
		event.stopPropagation(); // Prevent accordion from toggling
		if (onDeleteProvider) {
			onDeleteProvider(provider);
		} else {
			// Default delete implementation if no callback is provided
			if (confirm(`Are you sure you want to delete provider "${provider.name}"?`)) {
				deletePlaylist(provider.id!)
					.then(() => {
						// If successful, refresh the entire view
						window.location.reload();
					})
					.catch((error) => {
						console.error('Error deleting provider:', error);
						alert('Failed to delete provider: ' + error.message);
					});
			}
		}
	}

	function handleAddProvider(event: Event) {
		event.preventDefault();
		event.stopPropagation();
		if (onAddProvider) {
			onAddProvider();
		}
	}
</script>

<div class="space-y-4 max-h-[60vh] overflow-y-auto pr-2">
	{#if providers}
		<!-- Add Provider Button -->
		{#if onAddProvider}
			<button
				class={cn(
					buttonVariants({ variant: 'outline' }),
					'w-full flex items-center justify-center space-x-2 mb-2 bg-white/50 dark:bg-gray-700/50 hover:bg-indigo-100 dark:hover:bg-indigo-900/30'
				)}
				onclick={handleAddProvider}
			>
				<PlusCircle class="h-4 w-4 mr-2" />
				<span>Add New Provider</span>
			</button>
		{/if}

		<!-- Multi-provider mode -->
		{#each providerGroups as provider}
			<Accordion.Root type="single">
				<Accordion.Item value={provider.id.toString()}>
					<div class="flex items-center justify-between">
						<Accordion.Trigger class="text-xl font-bold flex-1">
							{provider.name}
						</Accordion.Trigger>
						<div class="flex gap-2 pr-2">
							{#if onEditProvider || onDeleteProvider}
								<button
									class={cn(buttonVariants({ variant: 'outline', size: 'icon' }))}
									title="Edit {provider.name}"
									onclick={(e) => {
										const providerObj = Array.from(providers.keys()).find(
											(p) => p.id === provider.id
										);
										if (providerObj) handleEdit(e, providerObj);
									}}
								>
									<Pencil class="h-4 w-4" />
								</button>
								<button
									class={cn(buttonVariants({ variant: 'destructive', size: 'icon' }))}
									title="Delete {provider.name}"
									onclick={(e) => {
										const providerObj = Array.from(providers.keys()).find(
											(p) => p.id === provider.id
										);
										if (providerObj) handleDelete(e, providerObj);
									}}
								>
									<Trash2 class="h-4 w-4" />
								</button>
							{/if}
						</div>
					</div>
					<Accordion.Content>
						<div class="space-y-2 mt-2">
							<!-- Content Type level -->
							{#each provider.contentTypes as contentType}
								<Accordion.Root type="single">
									<Accordion.Item value={`${provider.id}-${contentType.name}`}>
										<Accordion.Trigger class="text-lg font-medium pl-4">
											{contentType.name}
										</Accordion.Trigger>
										<Accordion.Content>
											<div class="space-y-2 mt-2 pl-4">
												<!-- Category level -->
												{#each contentType.categories as category}
													<Accordion.Root type="single">
														<Accordion.Item
															value={`${provider.id}-${contentType.name}-${category.id}`}
														>
															<Accordion.Trigger class="text-md font-medium pl-4">
																{category.name} ({category.channels.length})
															</Accordion.Trigger>
															<Accordion.Content>
																<div class="space-y-2 mt-2 pl-8">
																	<!-- Channel level -->
																	{#each category.channels as channel}
																		<button
																			class="w-full text-left p-2 rounded-md hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors duration-200 flex items-center space-x-2 {selectedChannel?.id ===
																			channel.id
																				? 'bg-indigo-100 dark:bg-indigo-900/30'
																				: ''}"
																			onclick={() => {
																				const providerObj = Array.from(providers.keys()).find(
																					(p) => p.id === provider.id
																				);
																				if (providerObj) handleChannelClick(channel, providerObj);
																			}}
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
					</Accordion.Content>
				</Accordion.Item>
			</Accordion.Root>
		{/each}
	{:else if channels}
		<!-- Single provider mode (legacy) -->
		<div class="space-y-2 mt-2">
			{#each providerContentTypes as contentType}
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
															onclick={() =>
																currentPlaylist && handleChannelClick(channel, currentPlaylist)}
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
	{:else}
		<p class="text-center text-muted-foreground">No channels available</p>
	{/if}
</div>
