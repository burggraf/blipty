<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import * as Card from '$lib/components/ui/card';
	import { updatePlaylist } from '$lib/commands';
	import type { Playlist } from '$lib/commands';

	const { provider, onSaved, onCancel } = $props<{
		provider: Playlist;
		onSaved?: () => void;
		onCancel?: () => void;
	}>();

	let name = $state(provider.name);
	let serverUrl = $state(provider.server_url);
	let username = $state(provider.username);
	let password = $state(provider.password);
	let loading = $state(false);
	let error = $state('');

	async function handleSubmit(event: SubmitEvent) {
		event.preventDefault();

		if (!name || !serverUrl || !username || !password) {
			error = 'All fields are required';
			return;
		}

		loading = true;
		error = '';

		const updatedPlaylist: Playlist = {
			id: provider.id,
			name,
			server_url: serverUrl,
			username,
			password,
			is_active: provider.is_active,
			last_updated: provider.last_updated
		};

		try {
			console.log('Submitting update for provider:', updatedPlaylist);

			await updatePlaylist(provider.id!, updatedPlaylist);

			console.log('Update successful');
			onSaved?.();
		} catch (e: any) {
			console.error('Update failed:', e);
			console.error('Full error object:', JSON.stringify(e, null, 2));
			error = e.message || 'Failed to update provider';
		} finally {
			loading = false;
		}
	}

	function handleCancel() {
		onCancel?.();
	}
</script>

<Card.Root class="w-[380px] backdrop-blur-sm bg-white/90 dark:bg-gray-800/90">
	<Card.Header>
		<Card.Title class="text-2xl font-bold">Edit Provider</Card.Title>
		<Card.Description>Update your Blipty provider details</Card.Description>
	</Card.Header>

	<Card.Content>
		<form onsubmit={handleSubmit} class="space-y-4">
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

			<div class="flex gap-2">
				<Button
					type="submit"
					disabled={loading}
					class="flex-1 bg-gradient-to-r from-indigo-500 to-pink-500 hover:opacity-90 transition-opacity duration-200"
				>
					{loading ? 'Saving...' : 'Save Changes'}
				</Button>
				<Button type="button" variant="outline" onclick={handleCancel} class="flex-1">
					Cancel
				</Button>
			</div>
		</form>
	</Card.Content>
</Card.Root>
