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
	let showAddForm = $state<boolean>(false);

	onMount(async () => {
		await loadProviders();
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
		loadProviders();
	}

	function handleEditCancel() {
		editingProvider = null;
	}

	function handleAddProviderClick() {
		showAddForm = true;
	}

	function handleProviderAdded() {
		showAddForm = false;
		// Refresh the providers list
		loadProviders();
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
										onclick={handleEditCancel}
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
										providersList={providers}
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
