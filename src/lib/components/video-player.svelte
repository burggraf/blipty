<script lang="ts">
	import { onMount } from 'svelte';
	import { useSidebar } from '$lib/components/ui/sidebar/context.svelte';
	import { videoStore } from '$lib/video/store';
	import { VideoService } from '$lib/video/service';
	import VideoControls from './video/controls/video-controls.svelte';
	import PlatformStyles from './video/controls/platform-styles.svelte';
	import { getPlatform } from '$lib/utils/platform';
	import 'video.js/dist/video-js.css';

	let { src, channelName = '' } = $props<{ src: string; channelName?: string }>();
	let videoId = $state(`video-${Math.random().toString(36).substring(2, 9)}`);
	let videoService = $state<VideoService | null>(null);
	let isError = $state(false);
	let errorMessage = $state('');
	let lastTouchY = $state(0);
	let volumeBeforeDrag = $state(0);

	const sidebar = useSidebar();
	const platform = getPlatform();
	const playerState = videoStore;

	onMount(() => {
		initializePlayer();
		setupTouchHandlers();

		return () => {
			videoService?.destroy();
			cleanupTouchHandlers();
		};
	});

	async function initializePlayer() {
		try {
			videoService = new VideoService(videoId);
			await videoService.initialize();

			if (src) {
				await handleSourceChange(src);
			}
		} catch (error) {
			console.error('Error initializing player:', error);
			isError = true;
			errorMessage = error instanceof Error ? error.message : 'Failed to initialize video player';
		}
	}

	async function handleSourceChange(newSrc: string) {
		if (!videoService) return;

		try {
			isError = false;
			errorMessage = '';
			await videoService.load(newSrc);
		} catch (error) {
			console.error('Error loading source:', error);
			isError = true;
			errorMessage = error instanceof Error ? error.message : 'Failed to load video';
		}
	}

	function setupTouchHandlers() {
		if (!platform.hasTouch) return;

		const videoElement = document.getElementById(videoId);
		if (!videoElement) return;

		videoElement.addEventListener('touchstart', handleTouchStart);
		videoElement.addEventListener('touchmove', handleTouchMove);
		videoElement.addEventListener('touchend', handleTouchEnd);
	}

	function cleanupTouchHandlers() {
		const videoElement = document.getElementById(videoId);
		if (!videoElement) return;

		videoElement.removeEventListener('touchstart', handleTouchStart);
		videoElement.removeEventListener('touchmove', handleTouchMove);
		videoElement.removeEventListener('touchend', handleTouchEnd);
	}

	function handleTouchStart(event: TouchEvent) {
		lastTouchY = event.touches[0].clientY;
		volumeBeforeDrag = videoStore.getState().volume;
	}

	function handleTouchMove(event: TouchEvent) {
		const currentY = event.touches[0].clientY;
		const deltaY = lastTouchY - currentY;
		const volumeChange = (deltaY / window.innerHeight) * 2;
		const newVolume = Math.max(0, Math.min(1, volumeBeforeDrag + volumeChange));

		videoStore.setVolume(newVolume);
	}

	function handleTouchEnd() {
		lastTouchY = 0;
		volumeBeforeDrag = 0;
	}

	$effect(() => {
		if (src && videoService) {
			handleSourceChange(src);
		}
	});
</script>

<div class={`video-wrapper ${sidebar.open ? 'sidebaropen' : 'sidebarclosed'}`}>
	<PlatformStyles />
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
		class="video-js vjs-big-play-button-centered vjs-fluid vjs-default-skin"
		playsinline
	>
		<track kind="captions" src="" label="Captions" />
		<p class="vjs-no-js">
			To view this video please enable JavaScript, and consider upgrading to a web browser that
			supports HTML5 video
		</p>
	</video>
	<VideoControls />
</div>

<style>
	.video-wrapper {
		position: absolute;
		top: 2rem;
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
		width: 100% !important;
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

	@media (max-width: 640px) {
		.sidebaropen {
			left: 0;
		}
	}
</style>
