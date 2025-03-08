<!-- VideoControls.svelte -->
<script lang="ts">
	import { onMount } from 'svelte';
	import { Button } from '$lib/components/ui/button';
	import { Slider } from '$lib/components/ui/slider';
	import {
		Play,
		Pause,
		Volume2,
		VolumeX,
		Maximize,
		Minimize,
		Settings,
		HelpCircle
	} from 'lucide-svelte';
	import { videoStore } from '$lib/video/store';
	import { formatDuration } from '$lib/video/utils';
	import { getPlatform } from '$lib/utils/platform';
	import { setupTVFocus } from '$lib/utils/tv-focus';
	import QualitySelector from './quality-selector.svelte';
	import ShortcutHelp from './shortcut-help.svelte';
	import { setupKeyboardControls } from './keyboard-controls';

	let isFullscreen = $state(false);
	let isMuted = $state(false);
	let showQualityMenu = $state(false);
	let controlsVisible = $state(true);
	let controlsTimeout: NodeJS.Timeout | null = null;
	let showShortcutHelp = $state(false);

	const { isAndroidTV } = getPlatform();

	// Hide controls after 3 seconds of inactivity
	function resetControlsTimer() {
		if (isAndroidTV) return; // Keep controls visible on Android TV
		if (controlsTimeout) clearTimeout(controlsTimeout);
		controlsVisible = true;
		controlsTimeout = setTimeout(() => {
			controlsVisible = false;
		}, 3000);
	}

	function togglePlay() {
		if ($videoStore.state === 'playing') {
			videoStore.pause();
		} else {
			videoStore.resume();
		}
	}

	function toggleMute() {
		if (isMuted) {
			videoStore.setVolume($videoStore.volume || 1);
		} else {
			videoStore.setVolume(0);
		}
		isMuted = !isMuted;
	}

	function handleVolumeChange(value: number) {
		videoStore.setVolume(value);
		isMuted = value === 0;
	}

	async function toggleFullscreen() {
		const wrapper = document.querySelector('.video-wrapper');
		if (!wrapper) return;

		if (!isFullscreen) {
			try {
				await wrapper.requestFullscreen();
				isFullscreen = true;
			} catch (error) {
				console.error('Failed to enter fullscreen:', error);
			}
		} else {
			try {
				await document.exitFullscreen();
				isFullscreen = false;
			} catch (error) {
				console.error('Failed to exit fullscreen:', error);
			}
		}
	}

	function handleSeek(value: number) {
		if ($videoStore.duration) {
			videoStore.seek(value * $videoStore.duration);
		}
	}

	// Update fullscreen state on change and setup keyboard controls
	onMount(() => {
		const cleanups = [];

		// Setup keyboard controls
		const keyboardControls = setupKeyboardControls({
			onFullscreenToggle: toggleFullscreen,
			onMuteToggle: toggleMute,
			onHelpToggle: () => (showShortcutHelp = !showShortcutHelp),
			onQualityMenuToggle: () => (showQualityMenu = !showQualityMenu)
		});
		cleanups.push(() => keyboardControls.destroy());

		// Setup TV focus management
		if (isAndroidTV) {
			const tvFocusManager = setupTVFocus();
			cleanups.push(() => tvFocusManager.destroy());
		}

		// Setup fullscreen change handler
		const handleFullscreenChange = () => {
			isFullscreen = !!document.fullscreenElement;
		};
		document.addEventListener('fullscreenchange', handleFullscreenChange);
		cleanups.push(() => document.removeEventListener('fullscreenchange', handleFullscreenChange));

		return () => {
			cleanups.forEach((cleanup) => cleanup());
		};
	});

	// Initialize volume from store
	$effect(() => {
		isMuted = $videoStore.volume === 0;
	});
</script>

<div
	class="video-controls absolute bottom-0 left-0 right-0 flex flex-col gap-2 bg-gradient-to-t from-black/80 to-transparent p-4 transition-opacity duration-300"
	class:opacity-0={!controlsVisible}
	class:opacity-100={isAndroidTV}
	on:mousemove={resetControlsTimer}
	on:mouseleave={() => {
		if (!isAndroidTV && controlsTimeout) {
			clearTimeout(controlsTimeout);
			controlsVisible = false;
		}
	}}
>
	<!-- Progress bar -->
	{#if $videoStore.duration}
		<Slider
			value={[$videoStore.position / $videoStore.duration]}
			onValueChange={([value]) => handleSeek(value)}
			max={1}
			step={0.001}
			class="progress-bar"
			data-tv-focus="progress"
			tabindex="0"
		/>
	{/if}

	<div class="flex items-center justify-between">
		<div class="flex items-center gap-2">
			<!-- Play/Pause -->
			<Button
				variant="ghost"
				size="icon"
				on:click={togglePlay}
				aria-label={$videoStore.state === 'playing' ? 'Pause' : 'Play'}
				data-tv-focus="play-pause"
				tabindex="0"
			>
				{#if $videoStore.state === 'playing'}
					<Pause class="h-6 w-6 text-white" />
				{:else}
					<Play class="h-6 w-6 text-white" />
				{/if}
			</Button>

			<!-- Volume -->
			<div class="flex items-center">
				<Button
					variant="ghost"
					size="icon"
					on:click={toggleMute}
					aria-label={isMuted ? 'Unmute' : 'Mute'}
					data-tv-focus="volume"
					tabindex="0"
				>
					{#if isMuted || $videoStore.volume === 0}
						<VolumeX class="h-6 w-6 text-white" />
					{:else}
						<Volume2 class="h-6 w-6 text-white" />
					{/if}
				</Button>
				<Slider
					value={[isMuted ? 0 : $videoStore.volume]}
					onValueChange={([value]) => handleVolumeChange(value)}
					max={1}
					step={0.01}
					class="w-24 volume-slider"
					aria-label="Volume"
					data-tv-focus="volume-slider"
					tabindex="0"
				/>
			</div>

			<!-- Time -->
			{#if $videoStore.duration}
				<span class="text-sm text-white">
					{formatDuration($videoStore.position)} / {formatDuration($videoStore.duration)}
				</span>
			{:else}
				<span class="text-sm text-white">LIVE</span>
			{/if}
		</div>

		<div class="flex items-center gap-2">
			<!-- Help -->
			<Button
				variant="ghost"
				size="icon"
				on:click={() => (showShortcutHelp = true)}
				aria-label="Keyboard shortcuts"
				data-tv-focus="help"
				tabindex="0"
			>
				<HelpCircle class="h-6 w-6 text-white" />
			</Button>

			<!-- Quality selector -->
			<div class="relative">
				<Button
					variant="ghost"
					size="icon"
					on:click={() => (showQualityMenu = !showQualityMenu)}
					aria-label="Quality settings"
					data-tv-focus="quality"
					tabindex="0"
				>
					<Settings class="h-6 w-6 text-white" />
				</Button>
				{#if showQualityMenu}
					<QualitySelector on:close={() => (showQualityMenu = false)} />
				{/if}
			</div>

			<!-- Fullscreen -->
			<Button
				variant="ghost"
				size="icon"
				on:click={toggleFullscreen}
				aria-label={isFullscreen ? 'Exit fullscreen' : 'Enter fullscreen'}
				data-tv-focus="fullscreen"
				tabindex="0"
			>
				{#if isFullscreen}
					<Minimize class="h-6 w-6 text-white" />
				{:else}
					<Maximize class="h-6 w-6 text-white" />
				{/if}
			</Button>
		</div>
	</div>
</div>

<ShortcutHelp bind:open={showShortcutHelp} />

<style>
	.video-controls {
		z-index: 50;
	}

	.progress-bar {
		height: 4px;
		cursor: pointer;
	}

	.progress-bar :global(.slider-track) {
		background-color: rgba(255, 255, 255, 0.2);
	}

	.progress-bar :global(.slider-range) {
		background-color: rgb(239, 68, 68);
	}

	.progress-bar :global(.slider-thumb) {
		width: 12px;
		height: 12px;
		background-color: rgb(239, 68, 68);
		opacity: 0;
		transition: opacity 0.2s;
	}

	.progress-bar:hover :global(.slider-thumb) {
		opacity: 1;
	}

	/* TV-specific focus styles */
	:global([data-tv-focus]:focus) {
		outline: 2px solid white;
		outline-offset: 2px;
	}

	:global([data-tv-focus]:focus:not(:focus-visible)) {
		outline: none;
	}

	/* Make controls more touch-friendly on mobile */
	@media (max-width: 640px) {
		.video-controls {
			padding: 1rem;
		}

		:global(.video-controls button) {
			min-height: 44px;
			min-width: 44px;
		}

		.progress-bar {
			height: 6px;
		}

		.progress-bar :global(.slider-thumb) {
			width: 16px;
			height: 16px;
			opacity: 1;
		}
	}
</style>
