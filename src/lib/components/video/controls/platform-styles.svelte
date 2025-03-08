<!-- PlatformStyles.svelte -->
<script lang="ts">
	import { getPlatform } from '$lib/utils/platform';

	// Update styles when platform changes
	const { isAndroidTV, isMobile, hasTouch } = getPlatform();
</script>

<svelte:head>
	{#if isAndroidTV}
		<style>
			/* Larger touch targets and focus indicators for TV */
			[data-tv-focus] {
				min-height: 48px !important;
				min-width: 48px !important;
			}

			[data-tv-focus]:focus {
				outline: 3px solid white !important;
				outline-offset: 4px !important;
				transform: scale(1.1);
				transition: transform 0.2s ease;
			}

			/* Ensure controls are always visible on TV */
			.video-controls {
				opacity: 1 !important;
			}

			/* Larger progress bar for TV remote control */
			.progress-bar {
				height: 8px !important;
			}

			.progress-bar :global(.slider-thumb) {
				width: 20px !important;
				height: 20px !important;
				opacity: 1 !important;
			}
		</style>
	{:else if isMobile || hasTouch}
		<style>
			/* Larger touch targets for mobile */
			.video-controls button {
				min-height: 44px !important;
				min-width: 44px !important;
			}

			/* Larger progress bar for touch */
			.progress-bar {
				height: 6px !important;
			}

			.progress-bar :global(.slider-thumb) {
				width: 16px !important;
				height: 16px !important;
				opacity: 1 !important;
			}

			/* Ensure volume slider is usable on touch */
			.volume-slider {
				min-width: 100px !important;
			}
		</style>
	{/if}
</svelte:head>
