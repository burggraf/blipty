import { invoke } from "@tauri-apps/api/core";

export interface Playlist {
    id?: number;
    name: string;
    server_url: string;
    username: string;
    password: string;
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

export async function fetchChannels(id: number): Promise<string> {
    return await invoke('fetch_channels', { id });
}