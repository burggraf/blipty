export function getPlatform() {
    // Check if running in Tauri
    const isTauri = !!window.__TAURI__;

    // Check if running on Android TV
    const isAndroidTV = isTauri && navigator.userAgent.includes('Android');

    // Check if running on mobile
    const isMobile = /iPhone|iPad|iPod|Android/i.test(navigator.userAgent);

    // Check if device supports touch
    const hasTouch = 'ontouchstart' in window || navigator.maxTouchPoints > 0;

    return {
        isTauri,
        isAndroidTV,
        isMobile,
        hasTouch,
        isDesktop: !isMobile && !isAndroidTV
    };
}