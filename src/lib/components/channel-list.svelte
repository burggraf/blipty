<script lang="ts">
	import * as Accordion from '$lib/components/ui/accordion';
	import type { Channel, Playlist } from '$lib/commands';
	import { setSelectedChannel, getSelectedChannel, getPlaylists } from '$lib/commands';
import { selectedPlaylist, selectedChannel as selectedChannelStore } from '$lib/stores';
	import VideoPlayer from './video-player.svelte';
	import { onMount } from 'svelte';

	export let channels: Channel[];
	export let playlist_id: number;

	let currentPlaylist: Playlist | null = null;

	// Currently selected channel
	let selectedChannel: Channel | null = null;

	onMount(async () => {
		if (playlist_id) {
			await loadPlaylistInfo();
			await loadSelectedChannel();
		}
	});

	async function loadPlaylistInfo() {
		try {
			const playlists = await getPlaylists();
			console.log('All playlists:', playlists);
			console.log('Looking for playlist ID:', playlist_id);
			currentPlaylist = playlists.find(p => p.id === playlist_id) || null;
			console.log('Found playlist:', currentPlaylist);
		} catch (error) {
			console.error('Error loading playlist info:', error);
		}
	}

	function getAuthenticatedStreamUrl(streamUrl: string): string {
		console.log('Getting authenticated stream URL');
		console.log('Current playlist:', currentPlaylist);
		console.log('Original stream URL:', streamUrl);
		
		if (!currentPlaylist) {
			console.log('No playlist available, returning original URL');
			return streamUrl;
		}
		
		try {
			const url = new URL(streamUrl);
			
			// Check if URL already has username/password parameters
			if (url.searchParams.has('username') || url.searchParams.has('password')) {
				console.log('URL already has credentials, using as is');
				return streamUrl;
			}

			// Add credentials as query parameters
			url.searchParams.set('username', currentPlaylist.username);
			url.searchParams.set('password', currentPlaylist.password);
			
			const authenticatedUrl = url.toString();
			console.log('Authenticated URL created:', authenticatedUrl);
			return authenticatedUrl;
		} catch (error) {
			console.error('Error adding authentication to URL:', error);
			console.error('Error details:', error instanceof Error ? error.message : String(error));
			console.log('Falling back to original URL:', streamUrl);
			return streamUrl;
		}
	}

	// Load selected channel
	async function loadSelectedChannel() {
		try {
			console.log('Loading selected channel for playlist:', playlist_id);
			selectedChannel = await getSelectedChannel(playlist_id);
			console.log('Selected channel:', selectedChannel);
		} catch (error) {
			console.error('Error loading selected channel:', error);
		}
	}

	// Group channels by category
	$: channelsByCategory = channels.reduce((acc, channel) => {
		// Get or create the category in the map
		const categoryId = channel.category_id || 'uncategorized';
		const category = acc.get(categoryId) || {
			name: channel.category_name || 'Uncategorized',
			channels: []
		};
		category.channels.push(channel);
		acc.set(categoryId, category);
		return acc;
	}, new Map<string, { name: string; channels: Channel[] }>());

	// Convert to array and sort by category name
	$: categories = Array.from(channelsByCategory.entries())
		.map(([id, data]) => ({
			id,
			name: data.name,
			channels: data.channels.sort((a, b) => a.name.localeCompare(b.name))
		}))
		.sort((a, b) => {
			// Put Uncategorized at the end
			if (a.name === 'Uncategorized') return 1;
			if (b.name === 'Uncategorized') return -1;
			return a.name.localeCompare(b.name);
		});

	async function handleChannelClick(channel: Channel) {
		if (channel.id && $selectedPlaylist?.id) {
			try {
				console.log('Setting selected channel:', channel.id, 'for playlist:', $selectedPlaylist.id);
				console.log('Channel details:', JSON.stringify(channel, null, 2));
				await setSelectedChannel($selectedPlaylist.id, channel.id);
				
				// Add authenticated stream URL to the channel
				console.log('Generating authenticated stream URL for channel:', channel.name);
				const authenticatedUrl = getAuthenticatedStreamUrl(channel.stream_url);
				console.log('Final authenticated URL that will be used:', authenticatedUrl);
				
				const channelWithAuth = {
					...channel,
					authenticated_stream_url: authenticatedUrl
				};
				
				selectedChannel = channelWithAuth;
				selectedChannelStore.set(channelWithAuth);
				console.log('Channel with auth set in store:', channelWithAuth);
			} catch (error) {
				console.error('Error setting selected channel:', error);
				console.error('Error details:', error instanceof Error ? error.message : String(error));
			}
		}
	}
</script>

<div class="w-full max-w-3xl mx-auto space-y-4">
	<!--
	{#if false && selectedChannel}
		<div class="w-full">
			<VideoPlayer src={getAuthenticatedStreamUrl(selectedChannel.stream_url)} />
			<div class="mt-2 text-lg font-semibold">{selectedChannel.name}</div>
		</div>
	{/if}
	-->
	<Accordion.Root type="single">
		{#each categories as category (category.id)}
			<Accordion.Item value={category.id}>
				<Accordion.Trigger class="flex justify-between items-center w-full">
					<span class="text-lg font-semibold">{category.name}</span>
					<span class="text-sm text-muted-foreground">({category.channels.length})</span>
				</Accordion.Trigger>
				<Accordion.Content>
					<div class="space-y-2 p-4">
						{#each category.channels as channel (channel.stream_id)}
							<button
								class="w-full text-left border rounded-lg p-3 bg-white/50 dark:bg-gray-700/50 hover:bg-white/70 dark:hover:bg-gray-600/50 transition-colors"
								class:ring-2={selectedChannel?.id === channel.id}
								class:ring-primary={selectedChannel?.id === channel.id}
								on:click={() => handleChannelClick(channel)}
							>
								<div class="font-medium">{channel.name}</div>
							</button>
						{/each}
					</div>
				</Accordion.Content>
			</Accordion.Item>
		{/each}
	</Accordion.Root>
</div>
