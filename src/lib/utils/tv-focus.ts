import { getPlatform } from './platform';

const FOCUSABLE_ELEMENTS = '[data-tv-focus]:not([disabled]):not([aria-hidden=true])';

export function setupTVFocus() {
    const { isAndroidTV } = getPlatform();
    if (!isAndroidTV) return;

    let currentFocusIndex = 0;

    function getAllFocusableElements(): HTMLElement[] {
        return Array.from(document.querySelectorAll(FOCUSABLE_ELEMENTS));
    }

    function handleKeydown(event: KeyboardEvent) {
        const focusableElements = getAllFocusableElements();
        if (!focusableElements.length) return;

        switch (event.keyCode) {
            case 37: // Left
                event.preventDefault();
                currentFocusIndex = Math.max(0, currentFocusIndex - 1);
                focusableElements[currentFocusIndex]?.focus();
                break;

            case 39: // Right
                event.preventDefault();
                currentFocusIndex = Math.min(focusableElements.length - 1, currentFocusIndex + 1);
                focusableElements[currentFocusIndex]?.focus();
                break;

            case 38: // Up
            case 40: // Down
                // Only handle up/down if we're on a slider
                const activeElement = document.activeElement;
                if (activeElement?.classList.contains('volume-slider') ||
                    activeElement?.classList.contains('progress-bar')) {
                    // Let the keyboard controls handle these cases
                    return;
                }
                event.preventDefault();
                break;
        }
    }

    // Set initial focus
    function initializeFocus() {
        const focusableElements = getAllFocusableElements();
        if (focusableElements.length) {
            currentFocusIndex = 0;
            focusableElements[0].focus();
        }
    }

    document.addEventListener('keydown', handleKeydown);

    // Initialize focus when controls become visible
    const observer = new MutationObserver((mutations) => {
        mutations.forEach((mutation) => {
            if (mutation.target.classList.contains('video-controls') &&
                !mutation.target.classList.contains('opacity-0')) {
                initializeFocus();
            }
        });
    });

    const controls = document.querySelector('.video-controls');
    if (controls) {
        observer.observe(controls, {
            attributes: true,
            attributeFilter: ['class']
        });
    }

    return {
        destroy() {
            document.removeEventListener('keydown', handleKeydown);
            observer.disconnect();
        }
    };
}