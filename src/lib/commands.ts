import { invoke } from "@tauri-apps/api/core";

export interface Playlist {
    id?: number;
    name: string;
    server_url: string;
    username: string;
    password: string;
    epg_url?: string;
    created_at?: string;
    updated_at?: string;
    last_updated?: string;
    is_active: boolean;
}

export async function initializeDatabase(): Promise<void> {
    await invoke('initialize_database');
}

export async function addPlaylist(playlist: Playlist): Promise<number> {
    return await invoke('add_playlist', { playlist });
}

export async function getPlaylists(): Promise<Playlist[]> {
    return await invoke('get_playlists');
}

export async function updatePlaylist(id: number, playlist: Playlist): Promise<void> {
    return await invoke('update_playlist', { id, playlist });
}

export async function deletePlaylist(id: number): Promise<void> {
    return await invoke('delete_playlist', { id });
}

export async function getDbPath(): Promise<string> {
    return await invoke('get_db_path');
}

export interface Channel {
    id?: number;
    playlist_id: number;
    category_id?: string;
    stream_id: string;
    name: string;
    stream_type: string;
    type_name?: string;
    stream_url: string;
    stream_icon?: string;
    epg_channel_id?: string;
    added?: string;
    series_no?: string;
    live?: string;
    container_extension?: string;
    custom_sid?: string;
    tv_archive?: number;
    direct_source?: string;
    tv_archive_duration?: number;
    num?: string;
    plot?: string;
    cast?: string;
    director?: string;
    genre?: string;
    release_date?: string;
    rating?: string;
    rating_5based?: number;
    backdrop_path?: string[];
    youtube_trailer?: string;
    episode_run_time?: string;
    cover?: string;
    created_at?: string;
    category_name?: string;
    content_type?: string;
    authenticated_stream_url?: string;
    is_selected?: number;
}

export async function fetchChannels(id: number): Promise<Channel[]> {
    // First check if we have any channels for this playlist
    const channels = await invoke('fetch_channels', { playlistId: id });

    // If no channels, fetch and populate data from the provider
    if (channels.length === 0) {
        console.log(`No channels found for playlist ${id}, fetching from provider...`);
        // Get the playlist details
        const playlists = await getPlaylists();
        const playlist = playlists.find(p => p.id === id);

        if (playlist) {
            try {
                console.log(`Fetching data for playlist: ${playlist.name}`);
                // Clean up the server URL to ensure it doesn't have trailing slashes
                let serverUrl = playlist.server_url.trim();
                if (serverUrl.endsWith('/')) {
                    serverUrl = serverUrl.slice(0, -1);
                }

                console.log(`Attempting to fetch data from: ${serverUrl}`);
                await invoke('fetch_and_populate_data', {
                    playlistId: id,
                    serverUrl: serverUrl,
                    username: playlist.username,
                    password: playlist.password
                });

                // Now fetch the channels again
                return await invoke('fetch_channels', { playlistId: id });
            } catch (error) {
                console.error('Error fetching data from provider:', error);
                throw error;
            }
        }
    }

    return channels;
}

export async function setSelectedChannel(playlist_id: number, channel_id: number): Promise<void> {
    return await invoke('set_selected_channel', { channel_id });
}

export async function getSelectedChannel(playlist_id: number): Promise<Channel | null> {
    return await invoke('get_selected_channel');
}
