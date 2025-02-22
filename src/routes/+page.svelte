<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import * as Card from '$lib/components/ui/card';
	import PlaylistForm from '$lib/components/playlist-form.svelte';
	import PlaylistEditForm from '$lib/components/playlist-edit-form.svelte';
	import { onMount } from 'svelte';
	import { initializeDatabase, getPlaylists, deletePlaylist, fetchChannels } from '$lib/commands';
	import type { Playlist } from '$lib/commands';
	import { buttonVariants } from '$lib/components/ui/button/button.svelte';
	import { cn } from '$lib/utils';
	import { Pencil, Trash2 } from 'lucide-svelte';

	let providers: Playlist[] = [];
	let error = '';
	let loading = false;
	let editingProvider: Playlist | null = null;
	let loadingProviders = new Set<number>();

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
		if (loadingProviders.has(provider.id!)) return;

		try {
			loadingProviders.add(provider.id!);
			error = '';
			console.log(`Fetching channels for provider: ${provider.name} (ID: ${provider.id})`);

			const results = await fetchChannels(provider.id!);
			console.log('Channel results:', results);
		} catch (e: any) {
			console.error('Failed to fetch channels:', e);
			error = e.message || 'Failed to fetch channels';
		} finally {
			loadingProviders.delete(provider.id!);
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
</script>

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
	{:else if providers.length > 0}
		<Card.Root class="w-[380px] backdrop-blur-sm bg-white/90 dark:bg-gray-800/90">
			<Card.Header class="space-y-2">
				<Card.Title
					class="text-3xl font-bold text-center bg-gradient-to-r from-indigo-500 to-pink-500 bg-clip-text text-transparent"
				>
					IPTV Providers
				</Card.Title>
			</Card.Header>
			<Card.Content class="p-6 space-y-4">
				{#each providers as provider}
					<div
						class="border rounded-lg p-4 flex items-center justify-between bg-white/50 dark:bg-gray-700/50"
					>
						<button
							class="font-semibold text-lg text-left hover:text-indigo-600 transition-colors duration-200"
							on:click={() => handleProviderClick(provider)}
							disabled={loadingProviders.has(provider.id!)}
						>
							{provider.name}
							{#if loadingProviders.has(provider.id!)}
								<span class="text-sm text-gray-500 ml-2">Loading...</span>
							{/if}
						</button>
						<div class="flex gap-2">
							<button
								class={cn(buttonVariants({ variant: 'outline', size: 'icon' }))}
								title="Edit {provider.name}"
								on:click={() => handleEdit(provider)}
							>
								<Pencil class="h-4 w-4" />
							</button>
							<button
								class={cn(buttonVariants({ variant: 'destructive', size: 'icon' }))}
								title="Delete {provider.name}"
								on:click={() => handleDelete(provider)}
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
					on:click={() => {
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
