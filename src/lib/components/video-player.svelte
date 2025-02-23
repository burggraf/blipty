<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import mpegts from 'mpegts.js';

	export let src: string;
	let videoId = `video-${Math.random().toString(36).substr(2, 9)}`;
	let player: mpegts.Player | null = null;

	function initializePlayer() {
		try {
			const element = document.getElementById(videoId) as HTMLMediaElement;
			if (!element) {
				console.error('Video element not found:', videoId);
				return;
			}

			console.log('Initializing mpegts.js player with source:', src);

			if (mpegts.getFeatureList().mseLivePlayback) {
				player = mpegts.createPlayer({
					type: 'mse',  // 'mse' works better for live streams
					isLive: true,
					url: src,
					fetchOptions: {
						cors: true,
						credentials: 'include'
					},
					configs: {
						enableStashBuffer: false,  // Reduce latency
						liveBufferLatencyChasing: true,
						liveSync: true,
						lazyLoad: false
					}
				});

				// Error handling
				player.on(mpegts.Events.ERROR, (errorType, errorDetail, errorInfo) => {
					console.error('mpegts.js error:', {
						type: errorType,
						detail: errorDetail,
						info: errorInfo
					});
				});

				// Log events
				player.on(mpegts.Events.MEDIA_INFO, (mediaInfo) => {
					console.log('Media info:', mediaInfo);
				});

				player.on(mpegts.Events.STATISTICS_INFO, (stats) => {
					console.log('Statistics:', stats);
				});

				// Attach player to video element
				player.attachMediaElement(element);
				player.load();
				player.play();
			} else {
				console.error('MSE live playback not supported in this browser');
			}
		} catch (error) {
			console.error('Error initializing player:', error);
		}
	}

	onMount(() => {
		console.log('Component mounted');
		console.log('Video element ID:', videoId);
		console.log('Source URL:', src);
		console.log('mpegts.js features:', mpegts.getFeatureList());
		
		// Wait for next tick to ensure element is in DOM
		setTimeout(initializePlayer, 0);
	});

	onDestroy(() => {
		if (player) {
			player.destroy();
		}
	});

	// Update source when it changes
	$: if (player && src) {
		console.log('Source changed, updating player source:', src);
		player.unload();
		player.detachMediaElement();
		player.destroy();
		initializePlayer();
	}
</script>

<div class="video-container w-full aspect-video bg-black">
	<video
		id={videoId}
		controls
		autoplay
		class="w-full h-full"
	>
		<p>
			Your browser does not support HTML5 video playback.
		</p>
	</video>
</div>

<style>
	.video-container {
		position: relative;
		overflow: hidden;
	}
</style>
