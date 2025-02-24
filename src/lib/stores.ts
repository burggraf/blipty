import { writable } from 'svelte/store';
import type { Channel, Playlist } from './commands';

export const selectedPlaylist = writable<Playlist | null>(null);
export const selectedChannel = writable<Channel | null>(null);
