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