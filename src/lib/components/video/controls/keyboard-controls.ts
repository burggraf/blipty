import { videoStore } from '$lib/video/store';
import { getPlatform } from '$lib/utils/platform';

interface KeyboardOptions {
    onFullscreenToggle: () => void;
    onMuteToggle: () => void;
    onHelpToggle: () => void;
    onQualityMenuToggle: () => void;
}

export function setupKeyboardControls(options: KeyboardOptions) {
    const { isAndroidTV } = getPlatform();

    function handleKeyDown(event: KeyboardEvent) {
        // Don't handle events if user is typing in an input
        if (event.target instanceof HTMLInputElement ||
            event.target instanceof HTMLTextAreaElement) {
            return;
        }

        // Handle Android TV remote control keys
        if (isAndroidTV) {
            switch (event.keyCode) {
                // Android TV remote Select/Enter button
                case 13: // Enter
                    event.preventDefault();
                    if (document.activeElement?.getAttribute('data-tv-focus') === 'play-pause') {
                        if (videoStore.getState().state === 'playing') {
                            videoStore.pause();
                        } else {
                            videoStore.resume();
                        }
                    }
                    break;

                // D-pad navigation is handled by the system
                case 37: // Left
                case 39: // Right
                    event.preventDefault();
                    // Only seek if we're focused on the progress bar
                    if (document.activeElement?.classList.contains('progress-bar')) {
                        const delta = event.keyCode === 37 ? -10 : 10;
                        if (!videoStore.getState().isLive) {
                            videoStore.seek(Math.max(0, videoStore.getState().position + delta));
                        }
                    }
                    break;

                // Volume control with up/down on remote
                case 38: // Up
                case 40: // Down
                    event.preventDefault();
                    const delta = event.keyCode === 38 ? 0.1 : -0.1;
                    const newVolume = Math.max(0, Math.min(1, videoStore.getState().volume + delta));
                    videoStore.setVolume(newVolume);
                    break;

                // Back button
                case 27: // Escape
                    event.preventDefault();
                    if (document.fullscreenElement) {
                        options.onFullscreenToggle();
                    }
                    break;
            }
        }

        // Regular keyboard controls for desktop
        switch (event.key.toLowerCase()) {
            case ' ':
            case 'k':
                event.preventDefault();
                if (videoStore.getState().state === 'playing') {
                    videoStore.pause();
                } else {
                    videoStore.resume();
                }
                break;

            case 'm':
                event.preventDefault();
                options.onMuteToggle();
                break;

            case 'f':
                event.preventDefault();
                options.onFullscreenToggle();
                break;

            case 'h':
                event.preventDefault();
                options.onHelpToggle();
                break;

            case 'q':
                event.preventDefault();
                options.onQualityMenuToggle();
                break;

            case 'arrowleft':
                if (!isAndroidTV) {
                    event.preventDefault();
                    if (!videoStore.getState().isLive) {
                        videoStore.seek(Math.max(0, videoStore.getState().position - 10));
                    }
                }
                break;

            case 'arrowright':
                if (!isAndroidTV) {
                    event.preventDefault();
                    if (!videoStore.getState().isLive) {
                        const duration = videoStore.getState().duration || 0;
                        videoStore.seek(Math.min(duration, videoStore.getState().position + 10));
                    }
                }
                break;

            case 'arrowup':
                if (!isAndroidTV) {
                    event.preventDefault();
                    const newVolumeUp = Math.min(1, videoStore.getState().volume + 0.1);
                    videoStore.setVolume(newVolumeUp);
                }
                break;

            case 'arrowdown':
                if (!isAndroidTV) {
                    event.preventDefault();
                    const newVolumeDown = Math.max(0, videoStore.getState().volume - 0.1);
                    videoStore.setVolume(newVolumeDown);
                }
                break;
        }
    }

    document.addEventListener('keydown', handleKeyDown);

    return {
        destroy() {
            document.removeEventListener('keydown', handleKeyDown);
        }
    };
}