<script lang="ts">
	import { dbService } from '../lib/stores';

	let sql = '';
	let results: any[] = [];
	let error = '';
	let tableStructure = '';

	async function executeQuery() {
		try {
			if ($dbService) {
				results = await $dbService.query(sql);
				error = '';
			}
		} catch (err) {
			error = err instanceof Error ? err.message : 'Error executing query';
			results = [];
		}
	}

	async function showTables() {
		sql = "SELECT name FROM sqlite_master WHERE type='table'";
		await executeQuery();
	}

	async function getTableInfo(tableName: string) {
		sql = `PRAGMA table_info('${tableName}')`;
		await executeQuery();
	}
</script>

<div class="sql-query-container">
	<h2>SQL Query Interface</h2>

	<div class="button-group">
		<button on:click={showTables}>Show Tables</button>
	</div>

	<div class="query-area">
		<textarea bind:value={sql} placeholder="Enter your SQL query here..." rows="4"></textarea>
		<button on:click={executeQuery}>Execute Query</button>
	</div>

	{#if error}
		<div class="error">{error}</div>
	{/if}

	{#if results.length > 0}
		<div class="results">
			<table>
				<thead>
					<tr>
						{#each Object.keys(results[0]) as header}
							<th>{header}</th>
						{/each}
					</tr>
				</thead>
				<tbody>
					{#each results as row}
						<tr>
							{#each Object.values(row) as cell}
								<td>{cell}</td>
							{/each}
						</tr>
					{/each}
				</tbody>
			</table>
		</div>
	{/if}
</div>

<style>
	.sql-query-container {
		padding: 1rem;
		max-width: 1200px;
		margin: 0 auto;
	}

	.query-area {
		margin: 1rem 0;
	}

	textarea {
		width: 100%;
		padding: 0.5rem;
		margin-bottom: 0.5rem;
		font-family: monospace;
	}

	button {
		background: #4caf50;
		color: white;
		padding: 0.5rem 1rem;
		border: none;
		border-radius: 4px;
		cursor: pointer;
		margin-right: 0.5rem;
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

	.results {
		margin-top: 1rem;
		overflow-x: auto;
	}

	table {
		width: 100%;
		border-collapse: collapse;
	}

	th,
	td {
		border: 1px solid #ddd;
		padding: 8px;
		text-align: left;
	}

	th {
		background-color: #f5f5f5;
	}

	tr:nth-child(even) {
		background-color: #f9f9f9;
	}

	.button-group {
		margin-bottom: 1rem;
	}
</style>
