<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import mpegts from 'mpegts.js';
	import videojs from 'video.js';
	import 'video.js/dist/video-js.css';

	export let src: string;
	let videoId = `video-${Math.random().toString(36).substr(2, 9)}`;
	let player: mpegts.Player | null = null;
	let vjsPlayer: any = null;
	let currentSrc: string | null = null;
	let retryCount = 0;
	let maxRetries = 3;
	let stallTimeout: NodeJS.Timeout | null = null;

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

	async function initializePlayer() {
		try {
			console.log('Initializing player for video ID:', videoId);
			const element = document.getElementById(videoId);
			if (!element) {
				console.error('Video element not found:', videoId);
				return;
			}
			console.log('Video element found');

			if (!mpegts.getFeatureList().mseLivePlayback) {
				console.error('MSE live playback not supported in this browser');
				return;
			}

			// Wait for any previous player instances to be fully destroyed
			await new Promise(resolve => setTimeout(resolve, 100));

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
					stashInitialSize: 1024 * 1024 * 1, // 1MB initial buffer
				}
			});

			// Handle errors
			player.on(mpegts.Events.ERROR, (errorType, errorDetail) => {
				console.error('Player error:', errorType, errorDetail);
				if (retryCount < maxRetries) {
					retryCount++;
					console.log(`Retrying playback (${retryCount}/${maxRetries})...`);
					destroyPlayer();
					setTimeout(initializePlayer, 1000);
				}
			});

			// Handle stall detection
			player.on(mpegts.Events.STATISTICS_INFO, (stats) => {
				if (stats.speed === 0) {
					if (!stallTimeout) {
						stallTimeout = setTimeout(() => {
							console.log('Playback stalled, attempting recovery...');
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
		}
	}

	onMount(() => {
		// Initialize on mount if we have a source
		if (src) {
			handleSourceChange(src);
		}
	});

	onDestroy(() => {
		if (stallTimeout) {
			clearTimeout(stallTimeout);
		}
		destroyPlayer();
	});

	// Update source when it changes
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
		await new Promise(resolve => setTimeout(resolve, 100));
		
		// Create a new video element to ensure clean state
		let wrapper = document.querySelector('.video-wrapper');
		let attempts = 0;
		const maxAttempts = 5;

		// Try to find the wrapper
		while (!wrapper && attempts < maxAttempts) {
			await new Promise(resolve => setTimeout(resolve, 100));
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
		newVideo.className = 'video-js vjs-big-play-button-centered vjs-fluid vjs-default-skin vjs-controls-enabled';
		newVideo.setAttribute('playsinline', '');
		newVideo.setAttribute('controls', '');
		
		// Add track and fallback content
		const track = document.createElement('track');
		track.kind = 'captions';
		track.label = 'Captions';
		newVideo.appendChild(track);

		const fallback = document.createElement('p');
		fallback.className = 'vjs-no-js';
		fallback.textContent = 'To view this video please enable JavaScript, and consider upgrading to a web browser that supports HTML5 video';
		newVideo.appendChild(fallback);
		
		// Add to DOM
		wrapper.appendChild(newVideo);
		console.log('New video element added to DOM');

		// Wait for the video element to be properly added to the DOM
		await new Promise(resolve => setTimeout(resolve, 100));

		// Verify the element exists and initialize
		const element = document.getElementById(videoId);
		if (element) {
			console.log('Starting player initialization...');
			await initializePlayer();
		} else {
			console.error('Video element not found after creation');
		}
	}

	$: handleSourceChange(src);
</script>

<div class="video-wrapper">
	<video
		id={videoId}
		class="video-js vjs-big-play-button-centered vjs-fluid vjs-default-skin vjs-controls-enabled"
		playsinline
		controls
	>
		<track kind="captions" src="" label="Captions" />
		<p class="vjs-no-js">
			To view this video please enable JavaScript, and consider upgrading to a
			web browser that supports HTML5 video
		</p>
	</video>
</div>

<style>
	.video-wrapper {
		position: absolute;
		top: 2rem;
		left: 24rem;
		right: 0;
		bottom: 0;

		background: red;
		display: flex;
		align-items: center;
		justify-content: center;
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
</style>
