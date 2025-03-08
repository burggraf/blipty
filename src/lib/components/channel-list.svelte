<script lang="ts">
	import { onMount } from 'svelte';
	import * as Accordion from '$lib/components/ui/accordion';
	import type { Channel, Playlist } from '$lib/commands';
	import {
		setSelectedChannel,
		getSelectedChannel,
		getPlaylists,
		deletePlaylist,
		fetchChannels,
		addToFavorites,
		removeFromFavorites,
		getFavorites
	} from '$lib/commands';
	import { selectedPlaylist, selectedChannel as selectedChannelStore } from '$lib/stores';
	import VideoPlayer from './video-player.svelte';
	import { buttonVariants } from '$lib/components/ui/button/button.svelte';
	import { cn } from '$lib/utils';
	import { Pencil, Trash2, PlusCircle, Loader2, Heart } from 'lucide-svelte';

	interface CategoryGroup {
		id: string;
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

	const { playlist_id, onEditProvider, onDeleteProvider, onAddProvider, providersList } = $props<{
		playlist_id?: number;
		onEditProvider?: (provider: Playlist) => void;
		onDeleteProvider?: (provider: Playlist) => void;
		onAddProvider?: () => void;
		providersList: Playlist[];
	}>();

	let currentPlaylist = $state<Playlist | null>(null);
	let selectedChannel = $state<Channel | null>(null);
	let storeValue = $state<Channel | null>(null);
	let allPlaylists = $state<Playlist[]>([]);
	let loadedProviders = $state<Map<number, Channel[]>>(new Map());
	let loadingProviders = $state<Set<number>>(new Set());
	let expandedContentTypes = $state<Set<string>>(new Set());
	let expandedCategories = $state<Set<string>>(new Set());
	let favorites = $state<Set<string>>(new Set());

	$effect(() => {
		const unsubscribe = selectedChannelStore.subscribe((value) => {
			storeValue = value;
		});
		return unsubscribe;
	});

	onMount(async () => {
		// Load favorites
		try {
			const favList = await getFavorites();
			favorites = new Set(favList.map(([_, streamId]) => streamId));
		} catch (error) {
			console.error('Error loading favorites:', error);
		}
	});

	async function loadProviderChannels(provider: Playlist) {
		if (loadedProviders.has(provider.id!) || loadingProviders.has(provider.id!)) {
			return;
		}

		try {
			loadingProviders.add(provider.id!);
			const channels = await fetchChannels(provider.id!);
			loadedProviders.set(provider.id!, channels);
			loadedProviders = new Map(loadedProviders);
		} catch (error) {
			console.error(`Error loading channels for provider ${provider.name}:`, error);
		} finally {
			loadingProviders.delete(provider.id!);
			loadingProviders = new Set(loadingProviders);
		}
	}

	function getProviderChannels(providerId: number): Channel[] {
		return loadedProviders.get(providerId) || [];
	}

	function processProviderChannels(channelsList: Channel[]): ContentTypeGroup[] {
		const channelsByContentType = channelsList.reduce(
			(acc: Map<string, Map<string, CategoryGroup>>, channel: Channel) => {
				const contentType = channel.stream_type || 'live';
				const categoryId = channel.category_id || 'uncategorized';

				if (!acc.has(contentType)) {
					acc.set(contentType, new Map());
					// Add favorites category for this content type
					acc.get(contentType)!.set('favorites', {
						id: 'favorites',
						name: 'Favorites',
						channels: []
					});
				}
				const contentTypeMap = acc.get(contentType)!;

				// If this channel is a favorite, add it to the favorites category
				if (favorites.has(channel.stream_id)) {
					const favoritesCategory = contentTypeMap.get('favorites')!;
					favoritesCategory.channels.push(channel);
				}

				if (!contentTypeMap.has(categoryId)) {
					contentTypeMap.set(categoryId, {
						id: categoryId,
						name: channel.category_name || 'Uncategorized',
						channels: []
					});
				}

				contentTypeMap.get(categoryId)!.channels.push(channel);
				return acc;
			},
			new Map<string, Map<string, CategoryGroup>>()
		);

		return Array.from(channelsByContentType.entries())
			.map(([contentType, categories]) => {
				const categoriesArray = Array.from(categories.values());
				// Filter out empty favorites categories
				const nonEmptyCategories = categoriesArray.filter(
					(cat) => cat.id !== 'favorites' || cat.channels.length > 0
				);
				// Sort all categories except favorites
				const sortedCategories = nonEmptyCategories.sort((a, b) => {
					if (a.id === 'favorites') return -1;
					if (b.id === 'favorites') return 1;
					return a.name.localeCompare(b.name);
				});

				return {
					name: contentType.charAt(0).toUpperCase() + contentType.slice(1),
					categories: sortedCategories.map((category) => ({
						...category,
						channels: category.channels.sort((a, b) => a.name.localeCompare(b.name))
					}))
				};
			})
			.sort((a, b) => a.name.localeCompare(b.name));
	}

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

	function handleEdit(event: Event, provider: Playlist) {
		event.stopPropagation();
		if (onEditProvider) {
			onEditProvider(provider);
		}
	}

	function handleDelete(event: Event, provider: Playlist) {
		event.stopPropagation();
		if (onDeleteProvider) {
			onDeleteProvider(provider);
		} else {
			if (confirm(`Are you sure you want to delete provider "${provider.name}"?`)) {
				deletePlaylist(provider.id!)
					.then(() => window.location.reload())
					.catch((error) => {
						console.error('Error deleting provider:', error);
						alert('Failed to delete provider: ' + error.message);
					});
			}
		}
	}

	async function handleAccordionChange(value: string | undefined) {
		if (!value) return;

		const provider = providersList.find((p: Playlist) => p.id!.toString() === value);
		if (provider) {
			await loadProviderChannels(provider);
		}
	}

	function handleContentTypeClick(providerId: number, contentTypeName: string) {
		const key = `${providerId}-${contentTypeName}`;
		if (expandedContentTypes.has(key)) {
			expandedContentTypes.delete(key);
		} else {
			expandedContentTypes.add(key);
		}
		expandedContentTypes = new Set(expandedContentTypes);
	}

	function handleCategoryClick(providerId: number, contentTypeName: string, categoryId: string) {
		const key = `${providerId}-${contentTypeName}-${categoryId}`;
		if (expandedCategories.has(key)) {
			expandedCategories.delete(key);
		} else {
			expandedCategories.add(key);
		}
		expandedCategories = new Set(expandedCategories);
	}

	function handleAddProvider(event: Event) {
		event.preventDefault();
		event.stopPropagation();
		if (onAddProvider) {
			onAddProvider();
		}
	}

	async function toggleFavorite(event: Event, channel: Channel) {
		event.stopPropagation();
		try {
			if (favorites.has(channel.stream_id)) {
				await removeFromFavorites(channel.playlist_id, channel.stream_id);
				favorites.delete(channel.stream_id);
			} else {
				await addToFavorites(channel.playlist_id, channel.stream_id, channel.stream_type);
				favorites.add(channel.stream_id);
			}
			favorites = new Set(favorites); // Trigger reactivity
		} catch (error) {
			console.error('Error toggling favorite:', error);
		}
	}
</script>

<div class="space-y-4 max-h-[60vh] overflow-y-auto pr-2">
	{#if providersList}
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

		<Accordion.Root type="single" onValueChange={handleAccordionChange}>
			{#each providersList as provider}
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
									onclick={(e) => handleEdit(e, provider)}
								>
									<Pencil class="h-4 w-4" />
								</button>
								<button
									class={cn(buttonVariants({ variant: 'destructive', size: 'icon' }))}
									title="Delete {provider.name}"
									onclick={(e) => handleDelete(e, provider)}
								>
									<Trash2 class="h-4 w-4" />
								</button>
							{/if}
						</div>
					</div>
					<Accordion.Content>
						<div class="space-y-2 mt-2">
							{#if loadingProviders.has(provider.id!)}
								<div class="flex items-center justify-center p-4">
									<Loader2 class="h-6 w-6 animate-spin" />
									<span class="ml-2">Loading channels...</span>
								</div>
							{:else if loadedProviders.has(provider.id!)}
								{#each processProviderChannels(getProviderChannels(provider.id!)) as contentType}
									<div class="mb-4">
										<button
											class="w-full text-left text-lg font-medium pl-4 py-2 hover:bg-gray-100 dark:hover:bg-gray-700 rounded-md"
											onclick={() => handleContentTypeClick(provider.id!, contentType.name)}
										>
											{contentType.name}
										</button>
										{#if expandedContentTypes.has(`${provider.id}-${contentType.name}`)}
											<div class="mt-2 space-y-2">
												{#each contentType.categories as category}
													<div class="ml-4">
														<button
															class="w-full text-left text-md font-medium pl-4 py-1 hover:bg-gray-100 dark:hover:bg-gray-700 rounded-md"
															onclick={() =>
																handleCategoryClick(provider.id!, contentType.name, category.id)}
														>
															{category.name} ({category.channels.length})
														</button>
														{#if expandedCategories.has(`${provider.id}-${contentType.name}-${category.id}`)}
															<div class="space-y-1 pl-8 mt-1">
																{#each category.channels as channel}
																	<div
																		class="w-full text-left p-2 rounded-md hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors duration-200 flex items-center justify-between cursor-pointer {selectedChannel?.id ===
																		channel.id
																			? 'bg-indigo-100 dark:bg-indigo-900/30'
																			: ''}"
																		onclick={() => handleChannelClick(channel, provider)}
																	>
																		<span class="truncate">{channel.name}</span>
																		<button
																			class="p-1 hover:bg-gray-200 dark:hover:bg-gray-600 rounded-full"
																			onclick={(e) => toggleFavorite(e, channel)}
																		>
																			<Heart
																				class="h-4 w-4 {favorites.has(channel.stream_id)
																					? 'fill-current text-red-500'
																					: 'text-gray-500'}"
																			/>
																		</button>
																	</div>
																{/each}
															</div>
														{/if}
													</div>
												{/each}
											</div>
										{/if}
									</div>
								{/each}
							{:else}
								<div class="text-center text-muted-foreground p-4">
									No channels found for this provider
								</div>
							{/if}
						</div>
					</Accordion.Content>
				</Accordion.Item>
			{/each}
		</Accordion.Root>
	{:else}
		<p class="text-center text-muted-foreground">No providers available</p>
	{/if}
</div>
