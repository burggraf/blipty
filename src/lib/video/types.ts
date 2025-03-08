export interface PlayerState {
    state: 'playing' | 'paused' | 'stopped' | 'error';
    duration?: number;
    position: number;
    volume: number;
    isLive: boolean;
}

export interface StreamQuality {
    width: number;
    height: number;
    framerate?: number;
    bitrate?: number;
}

export interface PlayerEvent {
    type: 'stateChanged' | 'bufferingProgress' | 'qualityChanged' | 'error' | 'endOfStream';
    data: {
        state?: string;
        percent?: number;
        quality?: StreamQuality;
        error?: {
            code: number;
            message: string;
        };
    };
}

export interface PlayerCommands {
    play: (url: string) => Promise<void>;
    pause: () => Promise<void>;
    resume: () => Promise<void>;
    stop: () => Promise<void>;
    seek: (position: number) => Promise<void>;
    setVolume: (volume: number) => Promise<void>;
    getStreamInfo: () => Promise<StreamInfo>;
    getState: () => Promise<PlayerState>;
}

export interface StreamInfo {
    duration?: number;
    position: number;
    videoCodec?: string;
    audioCodec?: string;
    resolution?: string;
    bitrate?: number;
}