<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import mpegts from 'mpegts.js';

	export let src: string;
	let videoId = `video-${Math.random().toString(36).substr(2, 9)}`;
	let player: mpegts.Player | null = null;
	let currentSrc: string | null = null;
	let retryCount = 0;
	let maxRetries = 3;
	let stallTimeout: NodeJS.Timeout | null = null;

	function destroyPlayer() {
		if (player) {
			try {
				player.unload();
				player.detachMediaElement();
				player.destroy();
				player = null;
			} catch (error) {
				console.error('Error destroying player:', error);
			}
		}
	}

	function initializePlayer() {
		try {
			const element = document.getElementById(videoId) as HTMLMediaElement;
			if (!element) {
				console.error('Video element not found:', videoId);
				return;
			}

			if (!mpegts.getFeatureList().mseLivePlayback) {
				console.error('MSE live playback not supported in this browser');
				return;
			}

			player = mpegts.createPlayer({
				type: 'mse',
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
			player.attachMediaElement(element);
			player.load();
			player.play();
		} catch (error) {
			console.error('Error initializing player:', error);
		}
	}

	onMount(() => {
		setTimeout(initializePlayer, 0);
	});

	onDestroy(() => {
		if (stallTimeout) {
			clearTimeout(stallTimeout);
		}
		destroyPlayer();
	});

	// Update source when it changes
	$: if (src !== currentSrc) {
		currentSrc = src;
		retryCount = 0;
		if (stallTimeout) {
			clearTimeout(stallTimeout);
			stallTimeout = null;
		}
		destroyPlayer();
		setTimeout(initializePlayer, 100);
	}
</script>

<div class="video-container w-full h-full bg-black">
	<video
		id={videoId}
		controls
		autoplay
		class="w-full h-full"
	>
		<track kind="captions" src="" label="Captions" />
		<p>
			Your browser does not support HTML5 video playback.
			Live captions are not available for this stream.
		</p>
	</video>
</div>

<style>
	.video-container {
		position: relative;
		overflow: hidden;
	}
</style>
