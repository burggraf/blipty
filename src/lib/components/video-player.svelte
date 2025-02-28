<script lang="ts">
	import mpegts from 'mpegts.js';
	import videojs from 'video.js';
	import 'video.js/dist/video-js.css';
	import { useSidebar } from '$lib/components/ui/sidebar/context.svelte';

	let { src, channelName = '' } = $props<{ src: string; channelName?: string }>();
	let videoId = $state(`video-${Math.random().toString(36).substring(2, 9)}`);
	let player = $state<mpegts.Player | null>(null);
	let vjsPlayer = $state<any>(null);
	let currentSrc = $state<string | null>(null);
	let retryCount = $state(0);
	let maxRetries = $state(3);
	let stallTimeout = $state<NodeJS.Timeout | null>(null);
	let isError = $state(false);
	let errorMessage = $state('');

	const sidebar = useSidebar();

	$effect(() => {
		console.log('Stream URL to load:', src);

		// Initialize on mount if we have a source
		if (src) {
			handleSourceChange(src);
		}

		return () => {
			if (stallTimeout) {
				clearTimeout(stallTimeout);
			}
			destroyPlayer();
		};
	});

	function destroyPlayer() {
		console.log('Destroying players...');
		if (player) {
			try {
				player.pause();
				player.unload();
				player.detachMediaElement();
				player.destroy();
				player = null;
				console.log('mpegts player destroyed');
			} catch (error) {
				console.error('Error destroying mpegts player:', error);
			}
		}
		if (vjsPlayer) {
			try {
				vjsPlayer.pause();
				vjsPlayer.reset();
				vjsPlayer.dispose();
				vjsPlayer = null;
				console.log('video.js player destroyed');
			} catch (error) {
				console.error('Error destroying Video.js player:', error);
			}
		}
	}

	// Variables already declared with $state above

	async function initializePlayer() {
		try {
			console.log('Initializing player for video ID:', videoId);
			const element = document.getElementById(videoId);
			if (!element) {
				console.error('Video element not found:', videoId);
				isError = true;
				errorMessage = 'Failed to initialize video player';
				return;
			}
			console.log('Video element found');

			if (!mpegts.getFeatureList().mseLivePlayback) {
				console.error('MSE live playback not supported in this browser');
				isError = true;
				errorMessage = 'Your browser does not support live video playback';
				return;
			}

			// Reset error state
			isError = false;
			errorMessage = '';

			// Wait for any previous player instances to be fully destroyed
			await new Promise((resolve) => setTimeout(resolve, 100));

			// Initialize Video.js first
			vjsPlayer = videojs(videoId, {
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

			// Add the live UI class
			vjsPlayer.addClass('vjs-live');
			vjsPlayer.addClass('vjs-show-controls');

			// Then initialize mpegts.js
			console.log('Creating mpegts player with URL:', src);
			player = mpegts.createPlayer({
				type: 'flv',
				isLive: true,
				url: src,
				fetchOptions: {
					cors: true,
					credentials: 'include'
				},
				configs: {
					enableStashBuffer: false,
					liveBufferLatencyChasing: true,
					liveSync: true,
					lazyLoad: false,
					stashInitialSize: 1024 * 1024 * 1 // 1MB initial buffer
				}
			});

			// Handle errors
			player.on(mpegts.Events.ERROR, (errorType, errorDetail) => {
				// Set error state and message
				isError = true;
				errorMessage = 'This video is not currently available';

				// Log error details for debugging
				console.debug('Player error details:', {
					type: errorType,
					detail: errorDetail,
					url: src
				});

				// Clean up resources
				destroyPlayer();
			});

			// Handle stall detection
			player.on(mpegts.Events.STATISTICS_INFO, (stats) => {
				if (stats.speed === 0) {
					if (!stallTimeout) {
						stallTimeout = setTimeout(() => {
							console.debug('Playback stalled, attempting recovery');
							destroyPlayer();
							initializePlayer();
						}, 5000);
					}
				} else if (stallTimeout) {
					clearTimeout(stallTimeout);
					stallTimeout = null;
				}
			});

			// Attach and start playback
			player.attachMediaElement(vjsPlayer.tech().el());
			player.load();
			player.play();
		} catch (error) {
			console.error('Error initializing player:', error);
			isError = true;
			errorMessage = 'Failed to initialize video player';
		}
	}

	async function handleSourceChange(newSrc: string) {
		if (newSrc === currentSrc) return;

		console.log('Source changed from', currentSrc, 'to', newSrc);
		currentSrc = newSrc;
		retryCount = 0;

		if (stallTimeout) {
			clearTimeout(stallTimeout);
			stallTimeout = null;
		}

		// Ensure complete cleanup
		destroyPlayer();

		// Wait for DOM to be ready
		await new Promise((resolve) => setTimeout(resolve, 100));

		// Create a new video element to ensure clean state
		let wrapper = document.querySelector('.video-wrapper');
		let attempts = 0;
		const maxAttempts = 5;

		// Try to find the wrapper
		while (!wrapper && attempts < maxAttempts) {
			await new Promise((resolve) => setTimeout(resolve, 100));
			wrapper = document.querySelector('.video-wrapper');
			attempts++;
			console.log(`Attempting to find video wrapper (${attempts}/${maxAttempts})`);
		}

		if (!wrapper) {
			console.error('Video wrapper not found after multiple attempts');
			return;
		}

		// Remove old video if it exists
		const oldVideo = document.getElementById(videoId);
		if (oldVideo) {
			console.log('Removing old video element');
			wrapper.removeChild(oldVideo);
		}

		// Generate a new unique ID for the video element
		videoId = `video-${Math.random().toString(36).substr(2, 9)}`;
		console.log('Creating new video element with ID:', videoId);

		// Create new video element
		const newVideo = document.createElement('video');
		newVideo.id = videoId;
		newVideo.className =
			'video-js vjs-big-play-button-centered vjs-fluid vjs-default-skin vjs-controls-enabled';
		newVideo.setAttribute('playsinline', '');
		newVideo.setAttribute('controls', '');

		// Add track and fallback content
		const track = document.createElement('track');
		track.kind = 'captions';
		track.label = 'Captions';
		newVideo.appendChild(track);

		const fallback = document.createElement('p');
		fallback.className = 'vjs-no-js';
		fallback.textContent =
			'To view this video please enable JavaScript, and consider upgrading to a web browser that supports HTML5 video';
		newVideo.appendChild(fallback);

		// Add to DOM
		wrapper.appendChild(newVideo);
		console.log('New video element added to DOM');

		// Wait for the video element to be properly added to the DOM
		await new Promise((resolve) => setTimeout(resolve, 100));

		// Verify the element exists and initialize
		const element = document.getElementById(videoId);
		if (element) {
			console.log('Starting player initialization...');
			await initializePlayer();
		} else {
			console.error('Video element not found after creation');
		}
	}

	$effect(() => {
		if (src !== currentSrc) {
			handleSourceChange(src);
		}
	});
</script>

<div class={`video-wrapper ${sidebar.open ? 'sidebaropen' : 'sidebarclosed'}`}>
	{#if isError}
		<div class="error-message">
			<p>{errorMessage}</p>
			{#if channelName}
				<p class="channel-name">{channelName}</p>
			{/if}
		</div>
	{/if}
	<video
		id={videoId}
		class="video-js vjs-big-play-button-centered vjs-fluid vjs-default-skin vjs-controls-enabled"
		playsinline
		controls
	>
		<track kind="captions" src="" label="Captions" />
		<p class="vjs-no-js">
			To view this video please enable JavaScript, and consider upgrading to a web browser that
			supports HTML5 video
		</p>
	</video>
</div>

<style>
	.video-wrapper {
		position: absolute;
		top: 2rem;
		/* left: 24rem;*/
		right: 0;
		bottom: 0;

		background: #000;
		display: flex;
		align-items: center;
		justify-content: center;
	}
	.sidebaropen {
		left: 24rem;
	}
	.sidebarclosed {
		left: 0;
	}

	:global(.video-js) {
		width: 100% /*calc(100% - 24rem)*/ !important;
		height: 100% !important;
		aspect-ratio: 16 / 9;
		max-height: 100vh;
	}

	:global(.video-js .vjs-tech) {
		width: 100%;
		height: 100%;
		object-fit: contain;
	}

	:global(.vjs-fluid:not(.vjs-audio-only-mode)) {
		padding-top: 0 !important;
	}

	.error-message {
		position: absolute;
		top: 50%;
		left: 50%;
		transform: translate(-50%, -50%);
		background: rgba(0, 0, 0, 0.8);
		padding: 1.5rem;
		border-radius: 0.5rem;
		text-align: center;
		color: white;
		z-index: 1000;
	}

	.error-message p {
		margin: 0 0 0.5rem 0;
		font-size: 1.1rem;
	}

	.error-message .channel-name {
		font-size: 0.9rem;
		color: #a1a1aa;
	}

	.retry-button {
		background: #4f46e5;
		color: white;
		border: none;
		padding: 0.5rem 1rem;
		border-radius: 0.25rem;
		cursor: pointer;
		transition: background-color 0.2s;
	}

	.retry-button:hover {
		background: #4338ca;
	}
</style>
