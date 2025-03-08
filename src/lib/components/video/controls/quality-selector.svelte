<!-- QualitySelector.svelte -->
<script lang="ts">
	import { clickOutside } from '$lib/hooks/click-outside';
	import { createEventDispatcher } from 'svelte';
	import { videoStore } from '$lib/video/store';
	import { getQualityLabel } from '$lib/video/utils';
	import { Check } from 'lucide-svelte';
	import * as Card from '$lib/components/ui/card';
	import { getPlatform } from '$lib/utils/platform';

	interface StreamQuality {
		height: number;
		width: number;
		bitrate?: number;
	}

	const dispatch = createEventDispatcher();
	const { isAndroidTV } = getPlatform();

	let selectedQuality = $state(videoStore.currentQuality);
	let qualities = $state(videoStore.availableQualities);
	let currentFocusIndex = $state(0);

	function handleKeydown(event: KeyboardEvent) {
		if (!isAndroidTV) return;

		switch (event.keyCode) {
			case 38: // Up
				event.preventDefault();
				currentFocusIndex = Math.max(0, currentFocusIndex - 1);
				focusCurrentItem();
				break;
			case 40: // Down
				event.preventDefault();
				currentFocusIndex = Math.min(qualities.length - 1, currentFocusIndex + 1);
				focusCurrentItem();
				break;
			case 13: // Enter
				event.preventDefault();
				handleQualitySelect(qualities[currentFocusIndex]);
				break;
			case 27: // Escape
				event.preventDefault();
				dispatch('close');
				break;
		}
	}

	function focusCurrentItem() {
		const items = document.querySelectorAll('[data-quality-item]');
		(items[currentFocusIndex] as HTMLElement)?.focus();
	}

	function handleQualitySelect(quality: StreamQuality) {
		videoStore.setQuality(quality);
		selectedQuality = quality;
		dispatch('close');
	}

	// Initialize focus on first item when menu opens
	$effect(() => {
		if (isAndroidTV) {
			setTimeout(() => {
				focusCurrentItem();
			}, 0);
		}
	});
</script>

<svelte:window onkeydown={handleKeydown} />

<div use:clickOutside={() => dispatch('close')}>
	<div
		class="absolute bottom-full right-0 mb-2 w-48 bg-black/90 text-white p-2 rounded-lg border border-border"
	>
		<div class="flex flex-col gap-1">
			{#each qualities as quality, i}
				<button
					class="flex items-center justify-between rounded px-3 py-2 text-sm hover:bg-white/10 focus:bg-white/20 focus:outline-none"
					class:selected={selectedQuality?.height === quality.height}
					onclick={() => handleQualitySelect(quality)}
					data-quality-item
					data-tv-focus="quality-item"
					tabindex={isAndroidTV ? (i === currentFocusIndex ? 0 : -1) : 0}
				>
					<span>{getQualityLabel(quality)}</span>
					{#if selectedQuality?.height === quality.height}
						<Check class="h-4 w-4" />
					{/if}
				</button>
			{/each}
		</div>
	</div>
</div>

<style>
	.selected {
		background-color: rgb(255 255 255 / 0.1);
	}

	[data-tv-focus='quality-item']:focus {
		outline: 2px solid white;
		outline-offset: -2px;
		border-radius: 4px;
	}
</style>
