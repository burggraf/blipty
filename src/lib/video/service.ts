import mpegts from 'mpegts.js';
import videojs from 'video.js';
import { videoStore } from './store';

export class VideoService {
    private mpegtsPlayer: mpegts.Player | null = null;
    private vjsPlayer: any | null = null;
    private videoElement: HTMLVideoElement | null = null;
    private retryCount = 0;
    private maxRetries = 3;
    private stallTimeout: NodeJS.Timeout | null = null;

    constructor(private videoId: string) { }

    async initialize() {
        this.videoElement = document.getElementById(this.videoId) as HTMLVideoElement;
        if (!this.videoElement) {
            throw new Error('Video element not found');
        }

        if (!mpegts.getFeatureList().mseLivePlayback) {
            throw new Error('MSE live playback not supported in this browser');
        }

        // Initialize Video.js
        this.vjsPlayer = videojs(this.videoId, {
            autoplay: true,
            controls: true,
            preload: 'auto',
            fluid: true,
            controlBar: {
                playToggle: true,
                volumePanel: true,
                currentTimeDisplay: true,
                timeDivider: true,
                durationDisplay: true,
                progressControl: true,
                remainingTimeDisplay: true,
                customControlSpacer: true,
                fullscreenToggle: true,
                pictureInPictureToggle: true,
                liveDisplay: true
            },
            liveui: true,
            liveTracker: true,
            html5: {
                vhs: {
                    overrideNative: true
                },
                nativeAudioTracks: false,
                nativeVideoTracks: false
            }
        });

        this.vjsPlayer.addClass('vjs-live');
        this.vjsPlayer.addClass('vjs-show-controls');
    }

    private calculateBufferSize(): number {
        const memoryGB = navigator.deviceMemory || 4;
        return Math.min(
            Math.max(memoryGB * 2 * 1024 * 1024, 4 * 1024 * 1024),
            64 * 1024 * 1024
        );
    }

    async load(url: string) {
        if (!this.vjsPlayer) {
            throw new Error('Player not initialized');
        }

        // Create mpegts player
        this.mpegtsPlayer = mpegts.createPlayer({
            type: 'flv',
            isLive: true,
            url,
            fetchOptions: {
                cors: true,
                credentials: 'include'
            },
            configs: {
                enableStashBuffer: false,
                liveBufferLatencyChasing: true,
                liveSync: true,
                lazyLoad: false,
                stashInitialSize: this.calculateBufferSize()
            }
        });

        // Handle errors
        this.mpegtsPlayer.on(mpegts.Events.ERROR, (errorType, errorDetail) => {
            console.error('Player error:', { type: errorType, detail: errorDetail });
            if (this.retryCount < this.maxRetries) {
                this.retryCount++;
                setTimeout(() => this.load(url), 1000 * Math.pow(2, this.retryCount));
            } else {
                this.destroy();
                throw new Error('Failed to play stream after multiple attempts');
            }
        });

        // Handle stall detection
        this.mpegtsPlayer.on(mpegts.Events.STATISTICS_INFO, (stats) => {
            if (stats.speed === 0) {
                if (!this.stallTimeout) {
                    this.stallTimeout = setTimeout(() => {
                        this.stallTimeout = null;
                        this.load(url);
                    }, Math.min(1000 * Math.pow(2, this.retryCount), 30000));
                }
            } else if (this.stallTimeout) {
                clearTimeout(this.stallTimeout);
                this.stallTimeout = null;
                this.retryCount = 0;
            }
        });

        // Attach and start playback
        this.mpegtsPlayer.attachMediaElement(this.vjsPlayer.tech().el());
        await this.mpegtsPlayer.load();
        await this.mpegtsPlayer.play();
        await videoStore.play(url);
    }

    destroy() {
        if (this.stallTimeout) {
            clearTimeout(this.stallTimeout);
            this.stallTimeout = null;
        }

        if (this.mpegtsPlayer) {
            this.mpegtsPlayer.pause();
            this.mpegtsPlayer.unload();
            this.mpegtsPlayer.detachMediaElement();
            this.mpegtsPlayer.destroy();
            this.mpegtsPlayer = null;
        }

        if (this.vjsPlayer) {
            this.vjsPlayer.pause();
            this.vjsPlayer.reset();
            this.vjsPlayer.dispose();
            this.vjsPlayer = null;
        }

        videoStore.reset();
    }
}