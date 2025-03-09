<script lang="ts">
  import svelteLogo from "./assets/svelte.svg";
  import viteLogo from "/vite.svg";
  import Counter from "./lib/Counter.svelte";
  import VideoPlayer from "./components/VideoPlayer.svelte";
  import DatabaseTest from "./components/DatabaseTest.svelte";
  import SqlQuery from "./components/SqlQuery.svelte";
  import { onMount } from "svelte";
  import { initDatabase, db } from "./lib/stores";
  import * as Sidebar from "$lib/components/ui/sidebar";
  import "./app.css";

  // Read the sample URLs from the parent directory
  const sampleUrls = `
  `
    .trim()
    .split("\n");

  let selectedUrl = $state(sampleUrls[0]);
  let dbInitError = $state("");

  onMount(async () => {
    try {
      await initDatabase();
    } catch (err) {
      dbInitError =
        err instanceof Error ? err.message : "Failed to initialize database";
      console.error("Database initialization error:", err);
    }
  });
</script>

<Sidebar.Provider>
  <Sidebar.Root>
    <Sidebar.Content>
      <Sidebar.Group>
        <Sidebar.GroupLabel>IPTV</Sidebar.GroupLabel>
        <Sidebar.GroupContent>
          <Sidebar.Menu>
            {#if dbInitError}
              <div class="p-4 text-red-500">
                {dbInitError}
              </div>
            {:else}
              <div
                class="w-full flex flex-col items-start p-2 bg-gradient-to-br from-indigo-500 via-purple-500 to-pink-500 transition-all duration-500"
              >
                <!-- Content will go here -->
              </div>
            {/if}
          </Sidebar.Menu>
        </Sidebar.GroupContent>
      </Sidebar.Group>
    </Sidebar.Content>
  </Sidebar.Root>
</Sidebar.Provider>

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

  {#if dbInitError}
    <div class="error">
      Database Error: {dbInitError}
    </div>
  {/if}

  <DatabaseTest />

  <SqlQuery />

  <div class="card">
    <Counter />
  </div>

  <p>
    Check out <a
      href="https://github.com/sveltejs/kit#readme"
      target="_blank"
      rel="noreferrer">SvelteKit</a
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

  .error {
    color: red;
    padding: 1rem;
    margin: 1rem 0;
    border: 1px solid red;
    border-radius: 4px;
  }
</style>
