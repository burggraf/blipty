<script lang="ts">
	import svelteLogo from './assets/svelte.svg';
	import viteLogo from '/vite.svg';
	import Counter from './lib/Counter.svelte';
	import VideoPlayer from './components/VideoPlayer.svelte';

	// Read the sample URLs from the parent directory
	const sampleUrls = `
  `
		.trim()
		.split('\n');

	let selectedUrl = $state(sampleUrls[0]);
</script>

<main>
	<div>
		<a href="https://vite.dev" target="_blank" rel="noreferrer">
			<img src={viteLogo} class="logo" alt="Vite Logo" />
		</a>
		<a href="https://svelte.dev" target="_blank" rel="noreferrer">
			<img src={svelteLogo} class="logo svelte" alt="Svelte Logo" />
		</a>
	</div>
	<h1>Vite + Svelte</h1>

	<div class="card">
		<Counter />
	</div>

	<p>
		Check out <a href="https://github.com/sveltejs/kit#readme" target="_blank" rel="noreferrer"
			>SvelteKit</a
		>, the official Svelte app framework powered by Vite!
	</p>

	<p class="read-the-docs">Click on the Vite and Svelte logos to learn more</p>

	<div class="controls">
		<select bind:value={selectedUrl}>
			{#each sampleUrls as url}
				<option value={url}>{url}</option>
			{/each}
		</select>
	</div>

	<div class="player-container">
		<VideoPlayer src={selectedUrl} />
	</div>
</main>

<style>
	.logo {
		height: 6em;
		padding: 1.5em;
		will-change: filter;
		transition: filter 300ms;
	}
	.logo:hover {
		filter: drop-shadow(0 0 2em #646cffaa);
	}
	.logo.svelte:hover {
		filter: drop-shadow(0 0 2em #ff3e00aa);
	}
	.read-the-docs {
		color: #888;
	}

	main {
		padding: 1rem;
	}

	.controls {
		margin-bottom: 1rem;
	}

	select {
		width: 100%;
		padding: 0.5rem;
	}

	.player-container {
		width: 100%;
		aspect-ratio: 16/9;
		background: #000;
	}
</style>
