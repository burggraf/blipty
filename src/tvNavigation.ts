/**
 * TV Navigation Helper for D-pad focus management
 */
export function setupTVNavigation(): void {
    // Get all focusable elements
    const focusableElements = document.querySelectorAll<HTMLElement>(
        'button, [href], input, select, textarea, [tabindex]:not([tabindex="-1"])'
    );

    // Add focus visual indicator
    focusableElements.forEach((el) => {
        el.addEventListener('focus', () => {
            el.classList.add('tv-focused');
        });

        el.addEventListener('blur', () => {
            el.classList.remove('tv-focused');
        });
    });

    // Initialize by focusing on the first element
    if (focusableElements.length > 0) {
        focusableElements[0].focus();
    }
}

// Handle remote control key events
document.addEventListener('keydown', (e: KeyboardEvent) => {
    switch (e.key) {
        case 'ArrowLeft':
        case 'ArrowRight':
        case 'Enter':
            // These events will be handled by the browser's default focus navigation
            break;
        case 'ArrowUp':
        case 'ArrowDown':
            // Let the channel-list component handle these for channel switching
            break;
        case 'PageUp':
        case 'ChannelUp':
            // Simulate ArrowUp for channel switching
            document.dispatchEvent(new KeyboardEvent('keydown', { key: 'ArrowUp' }));
            e.preventDefault();
            break;
        case 'PageDown':
        case 'ChannelDown':
            // Simulate ArrowDown for channel switching
            document.dispatchEvent(new KeyboardEvent('keydown', { key: 'ArrowDown' }));
            e.preventDefault();
            break;
        case 'Backspace':
            // Handle back button
            if (window.history.length > 1) {
                window.history.back();
                e.preventDefault();
            }
            break;
    }
});
