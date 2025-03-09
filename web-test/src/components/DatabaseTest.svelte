<script lang="ts">
	import { dbService } from '../lib/stores';
	import type { Playlist } from '../lib/services/db-service';

	let playlists: Playlist[] = [];
	let name = '';
	let serverUrl = '';
	let username = '';
	let password = '';
	let error = '';
	let success = '';

	async function loadPlaylists() {
		try {
			if ($dbService) {
				playlists = await $dbService.getPlaylists();
			}
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to load playlists';
		}
	}

	async function handleSubmit() {
		try {
			if ($dbService) {
				await $dbService.addPlaylist({
					name,
					server_url: serverUrl,
					username,
					password,
					is_active: 1
				});
				success = 'Playlist added successfully';
				name = serverUrl = username = password = '';
				await loadPlaylists();
			}
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to add playlist';
		}
	}

	async function handleDelete(id: number) {
		try {
			if ($dbService) {
				await $dbService.deletePlaylist(id);
				success = 'Playlist deleted successfully';
				await loadPlaylists();
			}
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to delete playlist';
		}
	}

	$: {
		if ($dbService) {
			loadPlaylists();
		}
	}
</script>

<div class="container">
	<h2>Add Playlist</h2>

	{#if error}
		<div class="error">{error}</div>
	{/if}

	{#if success}
		<div class="success">{success}</div>
	{/if}

	<form on:submit|preventDefault={handleSubmit}>
		<div class="form-group">
			<label for="name">Name:</label>
			<input type="text" id="name" bind:value={name} required />
		</div>

		<div class="form-group">
			<label for="serverUrl">Server URL:</label>
			<input type="url" id="serverUrl" bind:value={serverUrl} required />
		</div>

		<div class="form-group">
			<label for="username">Username:</label>
			<input type="text" id="username" bind:value={username} required />
		</div>

		<div class="form-group">
			<label for="password">Password:</label>
			<input type="password" id="password" bind:value={password} required />
		</div>

		<button type="submit">Add Playlist</button>
	</form>

	<h2>Playlists</h2>
	{#if playlists.length === 0}
		<p>No playlists added yet.</p>
	{:else}
		<ul>
			{#each playlists as playlist}
				<li>
					{playlist.name} ({playlist.server_url})
					<button on:click={() => handleDelete(playlist.id!)}>Delete</button>
				</li>
			{/each}
		</ul>
	{/if}
</div>

<style>
	.container {
		max-width: 600px;
		margin: 0 auto;
		padding: 1rem;
	}

	.form-group {
		margin-bottom: 1rem;
	}

	.form-group label {
		display: block;
		margin-bottom: 0.5rem;
	}

	.form-group input {
		width: 100%;
		padding: 0.5rem;
		border: 1px solid #ccc;
		border-radius: 4px;
	}

	button {
		background: #4caf50;
		color: white;
		padding: 0.5rem 1rem;
		border: none;
		border-radius: 4px;
		cursor: pointer;
	}

	button:hover {
		background: #45a049;
	}

	.error {
		color: red;
		padding: 1rem;
		margin: 1rem 0;
		border: 1px solid red;
		border-radius: 4px;
	}

	.success {
		color: green;
		padding: 1rem;
		margin: 1rem 0;
		border: 1px solid green;
		border-radius: 4px;
	}

	ul {
		list-style: none;
		padding: 0;
	}

	li {
		padding: 1rem;
		border: 1px solid #ccc;
		margin-bottom: 0.5rem;
		border-radius: 4px;
		display: flex;
		justify-content: space-between;
		align-items: center;
	}

	li button {
		background: #f44336;
	}

	li button:hover {
		background: #da190b;
	}
</style>
