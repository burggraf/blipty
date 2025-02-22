<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import * as Card from '$lib/components/ui/card';
	import PlaylistForm from '$lib/components/playlist-form.svelte';
	import { onMount } from 'svelte';
	import { initializeDatabase, getPlaylists } from '$lib/commands';
	import type { Playlist } from '$lib/commands';

	let providers: Playlist[] = [];
	let error = '';
	let loading = true;

	onMount(async () => {
		try {
			await initializeDatabase();
			providers = await getPlaylists();
		} catch (e: any) {
			error = e.message || 'Failed to initialize database';
			console.error('Database initialization error:', e);
		} finally {
			loading = false;
		}
	});
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
					<div class="border rounded-lg p-4 space-y-2 bg-white/50 dark:bg-gray-700/50">
						<div class="flex justify-between items-center">
							<h3 class="font-semibold text-lg">{provider.name}</h3>
							<div class="text-sm text-gray-500">{provider.server_url}</div>
						</div>
						<div class="text-sm text-gray-500">User: {provider.username}</div>
					</div>
				{/each}

				<Button
					onclick={() => (providers = [])}
					class="w-full bg-gradient-to-r from-indigo-500 to-pink-500 hover:opacity-90 transition-opacity duration-200"
				>
					Add Another Provider
				</Button>
			</Card.Content>
		</Card.Root>
	{:else}
		<PlaylistForm />
	{/if}
</div>
