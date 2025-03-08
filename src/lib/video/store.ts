import { writable } from 'svelte/store';
import type { PlayerState, StreamQuality, PlayerEvent, StreamInfo } from './types';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';

// Initial state
const initialState: PlayerState = {
    state: 'stopped',
    position: 0,
    volume: 1,
    isLive: true
};

function createVideoStore() {
    const { subscribe, set, update } = writable<PlayerState>(initialState);
    let streamInfo: StreamInfo | null = null;
    let currentQuality: StreamQuality | null = null;

    // Initialize event listener for player events from Rust
    listen<PlayerEvent>('player-event', (event) => {
        const { type, data } = event.payload;

        switch (type) {
            case 'stateChanged':
                update(state => ({ ...state, state: data.state as PlayerState['state'] }));
                break;
            case 'bufferingProgress':
                // Could update a buffering progress indicator if needed
                break;
            case 'qualityChanged':
                if (data.quality) {
                    currentQuality = data.quality;
                }
                break;
            case 'error':
                update(state => ({ ...state, state: 'error' }));
                break;
            case 'endOfStream':
                update(state => ({ ...state, state: 'stopped' }));
                break;
        }
    });

    return {
        subscribe,

        async play(url: string) {
            await invoke('play_video', { uri: url });
            update(state => ({ ...state, state: 'playing' }));
        },

        async pause() {
            await invoke('pause_video');
            update(state => ({ ...state, state: 'paused' }));
        },

        async resume() {
            await invoke('resume_video');
            update(state => ({ ...state, state: 'playing' }));
        },

        async stop() {
            await invoke('stop_video');
            update(state => ({ ...state, state: 'stopped', position: 0 }));
        },

        async setVolume(volume: number) {
            await invoke('set_volume', { volume });
            update(state => ({ ...state, volume }));
        },

        async seek(position: number) {
            if (!streamInfo?.duration) return; // Can't seek in live streams
            await invoke('seek', { position });
            update(state => ({ ...state, position }));
        },

        async updateStreamInfo() {
            streamInfo = await invoke<StreamInfo>('get_stream_info');
            if (streamInfo) {
                update(state => ({
                    ...state,
                    duration: streamInfo.duration,
                    position: streamInfo.position
                }));
            }
        },

        getCurrentQuality() {
            return currentQuality;
        },

        getStreamInfo() {
            return streamInfo;
        },

        async setQuality(quality: StreamQuality) {
            await invoke('set_quality', { quality });
            currentQuality = quality;
            update(state => ({ ...state }));
        },

        get currentQuality() {
            return currentQuality;
        },

        get availableQualities(): StreamQuality[] {
            return streamInfo?.qualities || [];
        },

        reset() {
            set(initialState);
            streamInfo = null;
            currentQuality = null;
        }
    };
}

export const videoStore = createVideoStore();