<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import * as Card from '$lib/components/ui/card';
	import PlaylistForm from '$lib/components/playlist-form.svelte';
	import { onMount } from 'svelte';
	import { initializeDatabase, getPlaylists, deletePlaylist } from '$lib/commands';
	import type { Playlist } from '$lib/commands';
	import { buttonVariants } from '$lib/components/ui/button/button.svelte';
	import { cn } from '$lib/utils';
	import { Pencil, Trash2 } from 'lucide-svelte';

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
		console.log('1. Delete clicked for provider:', provider);
		try {
			console.log('2. Calling deletePlaylist with ID:', provider.id);
			deletePlaylist(provider.id!)
				.then(() => {
					console.log('3. Delete successful');
					providers = providers.filter((p) => p.id !== provider.id);
					console.log('4. Updated providers list:', providers);
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

	function handleEdit(provider: Playlist) {
		// TODO: Implement edit functionality
		console.log('Edit clicked for provider:', provider.id);
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
