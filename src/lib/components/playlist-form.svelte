<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import * as Card from '$lib/components/ui/card';
	import { addPlaylist } from '$lib/commands';

	let name = '';
	let serverUrl = '';
	let username = '';
	let password = '';
	let loading = false;
	let error = '';

	async function handleSubmit() {
		if (!name || !serverUrl || !username || !password) {
			error = 'All fields are required';
			return;
		}

		loading = true;
		error = '';

		try {
			await addPlaylist({
				name,
				server_url: serverUrl,
				username,
				password,
				is_active: true
			});

			// Reset form
			name = '';
			serverUrl = '';
			username = '';
			password = '';
			error = '';

			// TODO: Add success notification or redirect
		} catch (e: any) {
			error = e.message || 'Failed to add playlist';
		} finally {
			loading = false;
		}
	}
</script>

<Card.Root class="w-[380px]">
	<Card.Header>
		<Card.Title class="text-2xl font-bold">Add New Playlist</Card.Title>
		<Card.Description>Enter your IPTV provider details below</Card.Description>
	</Card.Header>

	<Card.Content>
		<form on:submit|preventDefault={handleSubmit} class="space-y-4">
			<div class="space-y-2">
				<label for="name" class="text-sm font-medium">Provider Name</label>
				<Input
					id="name"
					type="text"
					bind:value={name}
					placeholder="Enter provider name"
					class="w-full"
				/>
			</div>

			<div class="space-y-2">
				<label for="serverUrl" class="text-sm font-medium">Server URL</label>
				<Input
					id="serverUrl"
					type="url"
					bind:value={serverUrl}
					placeholder="https://example.com/playlist"
					class="w-full"
				/>
			</div>

			<div class="space-y-2">
				<label for="username" class="text-sm font-medium">Username</label>
				<Input
					id="username"
					type="text"
					bind:value={username}
					placeholder="Enter username"
					class="w-full"
				/>
			</div>

			<div class="space-y-2">
				<label for="password" class="text-sm font-medium">Password</label>
				<Input
					id="password"
					type="password"
					bind:value={password}
					placeholder="Enter password"
					class="w-full"
				/>
			</div>

			{#if error}
				<p class="text-red-500 text-sm">{error}</p>
			{/if}

			<Button
				type="submit"
				disabled={loading}
				class="w-full bg-gradient-to-r from-indigo-500 to-pink-500 hover:opacity-90 transition-opacity duration-200"
			>
				{loading ? 'Adding...' : 'Add Playlist'}
			</Button>
		</form>
	</Card.Content>
</Card.Root>
