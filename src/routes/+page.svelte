<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import * as Card from '$lib/components/ui/card';
	import PlaylistForm from '$lib/components/playlist-form.svelte';
	import PlaylistEditForm from '$lib/components/playlist-edit-form.svelte';
	import ChannelList from '$lib/components/channel-list.svelte';
	import {
		initializeDatabase,
		getSelectedChannel,
		getPlaylists,
		deletePlaylist,
		fetchChannels
	} from '$lib/commands';
	import { selectedPlaylist, selectedChannel } from '$lib/stores';
	import type { Channel, Playlist } from '$lib/commands';
	import { buttonVariants } from '$lib/components/ui/button/button.svelte';
	import { cn } from '$lib/utils';
	import { Pencil, Trash2, ArrowLeft } from 'lucide-svelte';
	import { writable } from 'svelte/store';
	import VideoPlayer from '$lib/components/video-player.svelte';

	let providers = $state<Playlist[]>([]);
	let error = $state('');
	let loading = $state(false);
	let editingProvider = $state<Playlist | null>(null);
	let currentProvider = $state<Playlist | null>(null);
	let currentChannels = $state<Channel[]>([]);
	let loadingSet = $state(new Set<number>());
	const { playlist_id } = $props<{ playlist_id: number }>();
	let currentPlaylist = $state<Playlist | null>(null);
	let selectedChannelValue = $state<Channel | null>(null);

	async function initializeApp() {
		try {
			loading = true;
			await initializeDatabase();
			providers = await getPlaylists();
			console.log('Loaded providers:', providers);
			if (playlist_id) {
				await loadPlaylistInfo();
				await loadSelectedChannel();
			}
		} catch (e: any) {
			error = e.message || 'Failed to initialize database';
			console.error('Database initialization error:', e);
		} finally {
			loading = false;
		}
	}

	$effect(() => {
		initializeApp();
	});

	async function loadPlaylistInfo() {
		try {
			const playlists = await getPlaylists();
			console.log('All playlists:', playlists);
			console.log('Looking for playlist ID:', playlist_id);
			currentPlaylist = playlists.find((p) => p.id === playlist_id) || null;
			console.log('Found playlist:', currentPlaylist);
		} catch (error) {
			console.error('Error loading playlist info:', error);
		}
	}

	async function loadSelectedChannel() {
		try {
			if (!playlist_id) {
				console.log('No playlist_id available, cannot load selected channel');
				selectedChannelValue = null;
				return;
			}
			console.log('Loading selected channel for playlist:', playlist_id);
			const channel = await getSelectedChannel(playlist_id);
			// Update both the store and local state
			if (channel) {
				selectedChannelValue = channel;
				console.log('Selected channel:', channel);
			} else {
				selectedChannelValue = null;
				console.log('No channel selected');
			}
		} catch (error) {
			console.error('Error loading selected channel:', error);
			selectedChannelValue = null;
		}
	}

	async function handleProviderClick(provider: Playlist) {
		if (loadingSet.has(provider.id!)) return;

		try {
			loadingSet.add(provider.id!);
			error = '';
			console.log(`Fetching channels for provider: ${provider.name} (ID: ${provider.id})`);

			const results = await fetchChannels(provider.id!);
			console.log('Channel results:', results);
			currentProvider = provider;
			currentChannels = results;
		} catch (e: any) {
			console.error('Failed to fetch channels:', e);
			error = e.message || 'Failed to fetch channels';
		} finally {
			loadingSet.delete(provider.id!);
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

	function handleEditCancel() {
		editingProvider = null;
	}

	function handleBackToProviders() {
		currentProvider = null;
		currentChannels = [];
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
			console.log('Authenticated URL:', authenticatedUrl);
			return authenticatedUrl;
		} catch (error) {
			console.error('Error adding authentication to URL:', error);
			return streamUrl;
		}
	}

	// Subscribe to the selectedChannel store to update local state
	$effect(() => {
		const unsubscribe = selectedChannel.subscribe((value) => {
			console.log('selectedChannel store updated:', value);
			if (value) {
				// If the channel has a stream URL, ensure it's authenticated
				if (value.stream_url && !value.authenticated_stream_url) {
					value.authenticated_stream_url = getAuthenticatedStreamUrl(value.stream_url);
					console.log(
						'Added authenticated URL to channel from store:',
						value.authenticated_stream_url
					);
				}
				selectedChannelValue = value;
			}
		});

		return unsubscribe;
	});
</script>

<main class="w-full h-screen p-4">
	{#if selectedChannelValue && selectedChannelValue.authenticated_stream_url}
		<div class="flex flex-col w-full h-full">
			<div class="flex-1 min-w-0 min-h-0">
				<VideoPlayer 
					src={selectedChannelValue.authenticated_stream_url} 
					channelName={selectedChannelValue.name} 
				/>
			</div>
			<div class="p-2 text-lg font-semibold">{selectedChannelValue.name}</div>
		</div>
	{/if}
</main>
