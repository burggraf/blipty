import type { StreamQuality, StreamInfo } from './types';

export function formatBitrate(bitrate: number | undefined): string {
    if (!bitrate) return 'N/A';
    return bitrate >= 1000000
        ? `${(bitrate / 1000000).toFixed(1)} Mbps`
        : `${(bitrate / 1000).toFixed(0)} Kbps`;
}

export function formatResolution(width: number, height: number): string {
    if (height >= 2160) return '4K';
    if (height >= 1080) return '1080p';
    if (height >= 720) return '720p';
    if (height >= 480) return '480p';
    return `${width}x${height}`;
}

export function getQualityLabel(quality: StreamQuality): string {
    const resolution = formatResolution(quality.width, quality.height);
    const fps = quality.framerate ? `${Math.round(quality.framerate)}fps` : '';
    const bitrate = quality.bitrate ? ` (${formatBitrate(quality.bitrate)})` : '';
    return `${resolution} ${fps}${bitrate}`.trim();
}

export function parseStreamInfo(info: StreamInfo) {
    return {
        quality: info.resolution ? info.resolution : 'Auto',
        codec: [info.videoCodec, info.audioCodec].filter(Boolean).join(' / '),
        bitrate: info.bitrate ? formatBitrate(info.bitrate) : 'N/A',
        duration: info.duration ? formatDuration(info.duration) : 'Live'
    };
}

export function formatDuration(seconds: number): string {
    const hours = Math.floor(seconds / 3600);
    const minutes = Math.floor((seconds % 3600) / 60);
    const secs = Math.floor(seconds % 60);

    if (hours > 0) {
        return `${hours}:${padZero(minutes)}:${padZero(secs)}`;
    }
    return `${minutes}:${padZero(secs)}`;
}

function padZero(num: number): string {
    return num.toString().padStart(2, '0');
}

export function calculateOptimalBufferSize(): number {
    // Base buffer size on available memory (if available)
    const memoryGB = navigator.deviceMemory || 4; // Default to 4GB if not available

    // Calculate buffer size: 2MB per GB of memory
    // Minimum: 4MB, Maximum: 64MB
    const baseSize = memoryGB * 2 * 1024 * 1024;
    const minSize = 4 * 1024 * 1024;
    const maxSize = 64 * 1024 * 1024;

    return Math.min(Math.max(baseSize, minSize), maxSize);
}

export function createRetryStrategy(maxRetries = 3, baseDelay = 1000) {
    let retryCount = 0;

    return {
        shouldRetry: () => retryCount < maxRetries,
        getDelay: () => Math.min(baseDelay * Math.pow(2, retryCount), 30000),
        increment: () => { retryCount++ },
        reset: () => { retryCount = 0 },
        getAttempts: () => retryCount
    };
}