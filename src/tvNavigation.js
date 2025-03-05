/**
 * TV Navigation Helper for D-pad focus management
 */
export function setupTVNavigation() {
	// Get all focusable elements
	const focusableElements = document.querySelectorAll(
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
document.addEventListener('keydown', (e) => {
	switch (e.key) {
		case 'ArrowUp':
		case 'ArrowDown':
		case 'ArrowLeft':
		case 'ArrowRight':
		case 'Enter':
			// These events will be handled by the browser's default focus navigation
			// Just make sure we have proper focus indicators
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
