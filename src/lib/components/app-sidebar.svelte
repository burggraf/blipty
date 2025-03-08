<script lang="ts">
	import * as Sidebar from '$lib/components/ui/sidebar/index.js';
	import { Button } from '$lib/components/ui/button';
	import * as Card from '$lib/components/ui/card';
	import PlaylistForm from '$lib/components/playlist-form.svelte';
	import PlaylistEditForm from '$lib/components/playlist-edit-form.svelte';
	import ChannelList from '$lib/components/channel-list.svelte';
	import { onMount } from 'svelte';
	import { getPlaylists, deletePlaylist, fetchChannels } from '$lib/commands';
	import type { Channel, Playlist } from '$lib/commands';
	import { buttonVariants } from '$lib/components/ui/button/button.svelte';
	import { cn } from '$lib/utils';
	import { Pencil, Trash2, ArrowLeft } from 'lucide-svelte';
	import { writable } from 'svelte/store';
	import { selectedPlaylist } from '$lib/stores';

	let providers = $state<Playlist[]>([]);
	let error = $state('');
	let loading = $state(false);
	let editingProvider = $state<Playlist | null>(null);
	let currentProvider = $state<Playlist | null>(null);
	let currentChannels = $state<Channel[]>([]);
	let allProviderChannels = $state<Map<Playlist, Channel[]>>(new Map());
	let showAccordion = $state<boolean>(true); // Changed to true by default
	let showAddForm = $state<boolean>(false);
	const loadingProviders = writable(new Set<number>());
	let loadingSet = $state<Set<number>>(new Set());

	$effect(() => {
		loadingSet = $loadingProviders;
	});

	onMount(async () => {
		await loadProviders();
		// Automatically load all provider channels after loading providers
		if (providers.length > 0) {
			await loadAllProviderChannels();
		}
	});

	async function loadProviders() {
		try {
			loading = true;
			providers = await getPlaylists();
			console.log('Loaded providers:', providers);
		} catch (e: any) {
			error = e.message || 'Failed to initialize database';
			console.error('Database initialization error:', e);
		} finally {
			loading = false;
		}
	}

	async function loadAllProviderChannels() {
		if (providers.length === 0) {
			return;
		}

		const providerMap = new Map<Playlist, Channel[]>();
		showAccordion = false;
		loading = true;
		error = '';

		try {
			// Load channels for each provider
			for (const provider of providers) {
				if (provider.id === undefined) continue;

				loadingProviders.update((set) => {
					set.add(provider.id!);
					return set;
				});

				try {
					const channels = await fetchChannels(provider.id);
					console.log(`Loaded ${channels.length} channels for ${provider.name}`);
					providerMap.set(provider, channels);
				} catch (e: any) {
					console.error(`Error loading channels for ${provider.name}:`, e);
					// Continue with other providers even if one fails
				} finally {
					loadingProviders.update((set) => {
						set.delete(provider.id!);
						return set;
					});
				}
			}

			allProviderChannels = providerMap;
			showAccordion = true;
		} catch (e: any) {
			error = e.message || 'Failed to load channel data';
			console.error('Error loading all provider data:', e);
		} finally {
			loading = false;
		}
	}

	async function handleProviderClick(provider: Playlist) {
		if ($loadingProviders.has(provider.id!)) return;

		try {
			loadingProviders.update((set) => {
				set.add(provider.id!);
				return set;
			});
			error = '';
			console.log(`Fetching channels for provider: ${provider.name} (ID: ${provider.id})`);

			const results = await fetchChannels(provider.id!);
			console.log('Channel results:', results);
			currentProvider = provider;
			currentChannels = results;
			selectedPlaylist.set(provider);
		} catch (e: any) {
			console.error('Failed to fetch channels:', e);
			error = e.message || 'Failed to fetch channels';
		} finally {
			loadingProviders.update((set) => {
				set.delete(provider.id!);
				return set;
			});
		}
	}

	function handleEdit(provider: Playlist) {
		editingProvider = provider;
	}

	function handleDelete(provider: Playlist) {
		console.log('Delete clicked for provider:', provider);
		try {
			console.log('Calling deletePlaylist with ID:', provider.id);
			deletePlaylist(provider.id!)
				.then(() => {
					console.log('Delete successful');
					providers = providers.filter((p) => p.id !== provider.id);
					if (currentProvider?.id === provider.id) {
						currentProvider = null;
						currentChannels = [];
					}

					// Also update the all providers map if needed
					if (showAccordion && allProviderChannels.has(provider)) {
						allProviderChannels.delete(provider);
						// Force UI update
						allProviderChannels = new Map(allProviderChannels);
					}

					console.log('Updated providers list:', providers);
				})
				.catch((e) => {
					console.error('Delete failed:', e);
					error = e.message || 'Failed to delete provider';
				});
		} catch (e) {
			console.error('Immediate error:', e);
			error = `Failed to delete provider: ${e}`;
		}
	}

	function handleEditSave() {
		editingProvider = null;
		// Refresh the providers list
		loadProviders().then(() => {
			// If we're showing the accordion, refresh provider data
			if (showAccordion) {
				loadAllProviderChannels();
			}
		});
	}

	function handleEditCancel() {
		editingProvider = null;
	}

	function handleBackToProviders() {
		currentProvider = null;
		currentChannels = [];
		showAccordion = false;
		showAddForm = false;
	}

	function handleViewAllProviders() {
		loadAllProviderChannels();
	}

	function handleAddProviderClick() {
		showAddForm = true;
	}

	function handleProviderAdded() {
		showAddForm = false;
		// After adding a provider, reload providers and show in accordion
		loadProviders().then(() => {
			loadAllProviderChannels();
		});
	}
</script>

<Sidebar.Root>
	<Sidebar.Content>
		<Sidebar.Group>
			<Sidebar.GroupLabel>Application</Sidebar.GroupLabel>
			<Sidebar.GroupContent>
				<Sidebar.Menu>
					menu stuff<br />
					<div
						class="w-full flex flex-col items-start p-2 bg-gradient-to-br from-indigo-500 via-purple-500 to-pink-500 transition-all duration-500"
					>
						{#if loading}
							<Card.Root class="w-full backdrop-blur-sm bg-white/90 dark:bg-gray-800/90">
								<Card.Content class="p-6">
									<p class="text-center text-gray-500">Loading...</p>
								</Card.Content>
							</Card.Root>
						{:else if error}
							<Card.Root class="w-full backdrop-blur-sm bg-white/90 dark:bg-gray-800/90">
								<Card.Content class="p-6">
									<p class="text-red-500 text-center">{error}</p>
								</Card.Content>
							</Card.Root>
						{:else if showAddForm}
							<Card.Root class="w-full backdrop-blur-sm bg-white/90 dark:bg-gray-800/90">
								<Card.Header class="flex items-center space-x-4">
									<button
										class={cn(buttonVariants({ variant: 'outline', size: 'icon' }))}
										onclick={() => {
											showAddForm = false;
											showAccordion = true;
										}}
									>
										<ArrowLeft class="h-4 w-4" />
									</button>
									<Card.Title
										class="text-2xl font-bold bg-gradient-to-r from-indigo-500 to-pink-500 bg-clip-text text-transparent"
									>
										Add Provider
									</Card.Title>
								</Card.Header>
								<Card.Content class="p-6">
									<PlaylistForm onSaved={handleProviderAdded} />
								</Card.Content>
							</Card.Root>
						{:else if editingProvider}
							<Card.Root class="w-full backdrop-blur-sm bg-white/90 dark:bg-gray-800/90">
								<Card.Header class="flex items-center space-x-4">
									<button
										class={cn(buttonVariants({ variant: 'outline', size: 'icon' }))}
										onclick={() => {
											editingProvider = null;
											showAccordion = true;
										}}
									>
										<ArrowLeft class="h-4 w-4" />
									</button>
									<Card.Title
										class="text-2xl font-bold bg-gradient-to-r from-indigo-500 to-pink-500 bg-clip-text text-transparent"
									>
										Edit Provider
									</Card.Title>
								</Card.Header>
								<Card.Content class="p-6">
									<PlaylistEditForm
										provider={editingProvider}
										onSaved={handleEditSave}
										onCancel={handleEditCancel}
									/>
								</Card.Content>
							</Card.Root>
						{:else if providers.length === 0}
							<PlaylistForm onSaved={handleProviderAdded} />
						{:else}
							<Card.Root class="w-full backdrop-blur-sm bg-white/90 dark:bg-gray-800/90">
								<Card.Header class="flex items-center space-x-4">
									<Card.Title
										class="text-3xl font-bold bg-gradient-to-r from-indigo-500 to-pink-500 bg-clip-text text-transparent"
									>
										All Channels
									</Card.Title>
								</Card.Header>
								<Card.Content class="p-6">
									<ChannelList
										providers={allProviderChannels}
										onEditProvider={handleEdit}
										onDeleteProvider={handleDelete}
										onAddProvider={handleAddProviderClick}
									/>
								</Card.Content>
							</Card.Root>
						{/if}
					</div>
				</Sidebar.Menu>
			</Sidebar.GroupContent>
		</Sidebar.Group>
	</Sidebar.Content>
</Sidebar.Root>
