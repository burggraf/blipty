// Import TV navigation
import { setupTVNavigation } from './tvNavigation';
import './styles/tv.css';

// Check if running on Android TV
const isTVPlatform = () => {
	if (window.tauri) {
		// You can use Tauri API to detect if running on Android TV
		// This is a simplified check - enhance as needed
		return window.__TAURI__.app.getType().then((type) => type.includes('android'));
	}
	return false;
};

// Initialize TV navigation if on TV platform
document.addEventListener('DOMContentLoaded', () => {
	isTVPlatform().then((isTV) => {
		if (isTV) {
			setupTVNavigation();
			document.body.classList.add('tv-mode');
		}
	});
});
