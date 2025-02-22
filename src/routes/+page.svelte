<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import * as Card from '$lib/components/ui/card';
	import PlaylistForm from '$lib/components/playlist-form.svelte';
	import { onMount } from 'svelte';
	import { initializeDatabase, getPlaylists, deletePlaylist } from '$lib/commands';
	import type { Playlist } from '$lib/commands';

	let providers: Playlist[] = [];
	let error = '';
	let loading = true;

	onMount(async () => {
		try {
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

	function handleDelete(provider: Playlist) {
		// First debug log
		console.log('1. Delete clicked for provider:', provider);

		// Immediate alert to confirm the function is running
		alert(`About to process delete for: ${provider.name}`);

		try {
			// Log before deletion
			console.log('2. Calling deletePlaylist with ID:', provider.id);

			// Actually call the delete function
			deletePlaylist(provider.id!)
				.then(() => {
					// Log success
					console.log('3. Delete successful');

					// Update UI
					providers = providers.filter((p) => p.id !== provider.id);
					console.log('4. Updated providers list:', providers);

					// Show success message
					alert('Provider deleted successfully');
				})
				.catch((e) => {
					// Log error with full details
					console.error('Delete failed:', e);
					console.error('Full error object:', JSON.stringify(e, null, 2));
					alert(`Failed to delete provider: ${e.message || 'Unknown error'}`);
				});
		} catch (e) {
			// Log any immediate errors
			console.error('Immediate error:', e);
			alert(`Immediate error: ${e}`);
		}
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
						<h3 class="font-semibold text-lg">{provider.name}</h3>
						<div class="flex gap-2">
							<button
								class="bg-red-500 text-white px-4 py-2 rounded"
								on:click={() => handleDelete(provider)}
							>
								Delete {provider.name}
							</button>
						</div>
					</div>
				{/each}

				<button
					class="w-full bg-blue-500 text-white px-4 py-2 rounded"
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
