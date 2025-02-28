<script lang="ts">
	import Calendar from 'lucide-svelte/icons/calendar';
	import House from 'lucide-svelte/icons/house';
	import Inbox from 'lucide-svelte/icons/inbox';
	import Search from 'lucide-svelte/icons/search';
	import Settings from 'lucide-svelte/icons/settings';
	import * as Sidebar from '$lib/components/ui/sidebar/index.js';
	import { Button } from '$lib/components/ui/button';
	import * as Card from '$lib/components/ui/card';
	import PlaylistForm from '$lib/components/playlist-form.svelte';
	import PlaylistEditForm from '$lib/components/playlist-edit-form.svelte';
	import ChannelList from '$lib/components/channel-list.svelte';
	import { onMount } from 'svelte';
	import { initializeDatabase, getPlaylists, deletePlaylist, fetchChannels } from '$lib/commands';
	import type { Channel, Playlist } from '$lib/commands';
	import { buttonVariants } from '$lib/components/ui/button/button.svelte';
	import { cn } from '$lib/utils';
	import { Pencil, Trash2, ArrowLeft } from 'lucide-svelte';
	import { writable } from 'svelte/store';
	import { selectedPlaylist, selectedChannel } from '$lib/stores';
	let providers: Playlist[] = [];
	let error = '';
	let loading = false;
	let editingProvider: Playlist | null = null;
	let currentProvider: Playlist | null = null;
	let currentChannels: Channel[] = [];
	const loadingProviders = writable(new Set<number>());
	$: loadingSet = $loadingProviders;

	onMount(async () => {
		try {
			loading = true;
			await initializeDatabase();
			providers = await getPlaylists();
			console.log('Loaded providers:', providers);
		} catch (e: any) {
			error = e.message || 'Failed to initialize database';
			console.error('Database initialization error:', e);
		} finally {
			loading = false;
		}
	});

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
		getPlaylists().then((updatedProviders) => {
			providers = updatedProviders;
		});
	}

	function handleEditCancel() {
		editingProvider = null;
	}

	function handleBackToProviders() {
		currentProvider = null;
		currentChannels = [];
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
						class="min-h-screen flex items-center justify-center p-4 bg-gradient-to-br from-indigo-500 via-purple-500 to-pink-500 transition-all duration-500"
					>
						{#if loading}
							<Card.Root class="w-[380px] backdrop-blur-sm bg-white/90 dark:bg-gray-800/90">
								<Card.Content class="p-6">
									<p class="text-center text-gray-500">Loading...</p>
								</Card.Content>
							</Card.Root>
						{:else if error}
							<Card.Root class="w-[380px] backdrop-blur-sm bg-white/90 dark:bg-gray-800/90">
								<Card.Content class="p-6">
									<p class="text-red-500 text-center">{error}</p>
								</Card.Content>
							</Card.Root>
						{:else if editingProvider}
							<PlaylistEditForm
								provider={editingProvider}
								on:saved={handleEditSave}
								on:cancel={handleEditCancel}
							/>
						{:else if currentProvider && currentChannels.length > 0}
							<Card.Root class="w-[800px] backdrop-blur-sm bg-white/90 dark:bg-gray-800/90">
								<Card.Header class="flex items-center space-x-4">
									<button
										class={cn(buttonVariants({ variant: 'outline', size: 'icon' }))}
										onclick={handleBackToProviders}
									>
										<ArrowLeft class="h-4 w-4" />
									</button>
									<Card.Title
										class="text-3xl font-bold bg-gradient-to-r from-indigo-500 to-pink-500 bg-clip-text text-transparent"
									>
										{currentProvider.name} Channels
									</Card.Title>
								</Card.Header>
								<Card.Content class="p-6">
									<ChannelList channels={currentChannels} playlist_id={currentProvider.id!} />
								</Card.Content>
							</Card.Root>
						{:else if providers.length > 0}
							<Card.Root class="w-[380px] backdrop-blur-sm bg-white/90 dark:bg-gray-800/90">
								<Card.Header class="space-y-2">
									<div class="flex items-center justify-between">
										<h2 class="text-lg font-semibold tracking-tight">Blipty Providers</h2>
									</div>
								</Card.Header>
								<Card.Content class="p-6 space-y-4">
									{#each providers as provider}
										<div
											class="border rounded-lg p-4 flex items-center justify-between bg-white/50 dark:bg-gray-700/50"
										>
											<button
												class="font-semibold text-lg text-left hover:text-indigo-600 transition-colors duration-200"
												onclick={() => handleProviderClick(provider)}
												disabled={$loadingProviders.has(provider.id!)}
											>
												{provider.name}
												{#if $loadingProviders.has(provider.id!)}
													<span class="text-sm text-gray-500 ml-2">Loading...</span>
												{/if}
											</button>
											<div class="flex gap-2">
												<button
													class={cn(buttonVariants({ variant: 'outline', size: 'icon' }))}
													title="Edit {provider.name}"
													onclick={() => handleEdit(provider)}
												>
													<Pencil class="h-4 w-4" />
												</button>
												<button
													class={cn(buttonVariants({ variant: 'destructive', size: 'icon' }))}
													title="Delete {provider.name}"
													onclick={() => handleDelete(provider)}
												>
													<Trash2 class="h-4 w-4" />
												</button>
											</div>
										</div>
									{/each}

									<button
										class={cn(
											buttonVariants({ variant: 'default' }),
											'w-full bg-gradient-to-r from-indigo-500 to-pink-500 hover:opacity-90 transition-opacity duration-200'
										)}
										onclick={() => {
											console.log('Add clicked');
											providers = [];
										}}
									>
										Add Another Provider
									</button>
								</Card.Content>
							</Card.Root>
						{:else}
							<PlaylistForm />
						{/if}
					</div>

					<!--
        {#each items as item (item.title)}
         <Sidebar.MenuItem>
          <Sidebar.MenuButton>
           {#snippet child({ props })}
            <a href={item.url} {...props}>
             <item.icon />
             <span>{item.title}</span>
            </a>
           {/snippet}
          </Sidebar.MenuButton>
         </Sidebar.MenuItem>
        {/each}
        -->
				</Sidebar.Menu>
			</Sidebar.GroupContent>
		</Sidebar.Group>
	</Sidebar.Content>
</Sidebar.Root>
