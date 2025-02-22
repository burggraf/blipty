<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import * as Card from '$lib/components/ui/card';
	import PlaylistForm from '$lib/components/playlist-form.svelte';
	import { onMount } from 'svelte';
	import { initializeDatabase } from '$lib/commands';

	let showForm = false;
	let error = '';

	onMount(async () => {
		try {
			await initializeDatabase();
		} catch (e: any) {
			error = e.message || 'Failed to initialize database';
			console.error('Database initialization error:', e);
		}
	});

	function toggleForm() {
		showForm = !showForm;
	}
</script>

<div
	class="min-h-screen flex items-center justify-center p-4 bg-gradient-to-br from-indigo-500 via-purple-500 to-pink-500 transition-all duration-500"
>
	{#if error}
		<Card.Root class="w-[380px] backdrop-blur-sm bg-white/90 dark:bg-gray-800/90">
			<Card.Content class="p-6">
				<p class="text-red-500 text-center">{error}</p>
			</Card.Content>
		</Card.Root>
	{:else if !showForm}
		<Card.Root
			class="w-[380px] backdrop-blur-sm bg-white/90 dark:bg-gray-800/90 shadow-xl hover:shadow-2xl transition-all duration-300 rounded-xl [--ring:267_100%_60%]"
		>
			<Card.Header class="space-y-2">
				<Card.Title
					class="text-3xl font-bold text-center bg-gradient-to-r from-indigo-500 to-pink-500 bg-clip-text text-transparent"
				>
					IPTV Playlists
				</Card.Title>
			</Card.Header>
			<Card.Content class="p-6">
				<Button
					onclick={toggleForm}
					class="w-full bg-gradient-to-r from-indigo-500 to-pink-500 hover:opacity-90 transition-opacity duration-200 py-6 text-lg font-semibold"
				>
					<svg
						xmlns="http://www.w3.org/2000/svg"
						class="h-6 w-6 mr-2 inline-block"
						fill="none"
						viewBox="0 0 24 24"
						stroke="currentColor"
					>
						<path
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="2"
							d="M12 4v16m8-8H4"
						/>
					</svg>
					Add New Playlist
				</Button>
			</Card.Content>
		</Card.Root>
	{:else}
		<div class="relative">
			<PlaylistForm />
			<Button
				onclick={toggleForm}
				class="absolute -top-4 -right-4 rounded-full w-8 h-8 p-0 bg-gray-800 hover:bg-gray-700 transition-colors"
				variant="secondary"
			>
				✕
			</Button>
		</div>
	{/if}
</div>
