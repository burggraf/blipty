
// this file is generated — do not edit it


/// <reference types="@sveltejs/kit" />

/**
 * Environment variables [loaded by Vite](https://vitejs.dev/guide/env-and-mode.html#env-files) from `.env` files and `process.env`. Like [`$env/dynamic/private`](https://svelte.dev/docs/kit/$env-dynamic-private), this module cannot be imported into client-side code. This module only includes variables that _do not_ begin with [`config.kit.env.publicPrefix`](https://svelte.dev/docs/kit/configuration#env) _and do_ start with [`config.kit.env.privatePrefix`](https://svelte.dev/docs/kit/configuration#env) (if configured).
 * 
 * _Unlike_ [`$env/dynamic/private`](https://svelte.dev/docs/kit/$env-dynamic-private), the values exported from this module are statically injected into your bundle at build time, enabling optimisations like dead code elimination.
 * 
 * ```ts
 * import { API_KEY } from '$env/static/private';
 * ```
 * 
 * Note that all environment variables referenced in your code should be declared (for example in an `.env` file), even if they don't have a value until the app is deployed:
 * 
 * ```
 * MY_FEATURE_FLAG=""
 * ```
 * 
 * You can override `.env` values from the command line like so:
 * 
 * ```bash
 * MY_FEATURE_FLAG="enabled" npm run dev
 * ```
 */
declare module '$env/static/private' {
	export const SHELL: string;
	export const npm_command: string;
	export const COLORTERM: string;
	export const PYENV_SHELL: string;
	export const XPC_FLAGS: string;
	export const NVM_INC: string;
	export const TERM_PROGRAM_VERSION: string;
	export const CONDA_EXE: string;
	export const _CE_M: string;
	export const NODE: string;
	export const __CFBundleIdentifier: string;
	export const SSH_AUTH_SOCK: string;
	export const MallocNanoZone: string;
	export const npm_config_local_prefix: string;
	export const EDITOR: string;
	export const PWD: string;
	export const EMSDK_PYTHON: string;
	export const LOGNAME: string;
	export const CONDA_PREFIX: string;
	export const PNPM_HOME: string;
	export const _: string;
	export const VSCODE_GIT_ASKPASS_NODE: string;
	export const COMMAND_MODE: string;
	export const HOME: string;
	export const LANG: string;
	export const npm_package_version: string;
	export const WASMTIME_HOME: string;
	export const CONDA_PROMPT_MODIFIER: string;
	export const TMPDIR: string;
	export const GIT_ASKPASS: string;
	export const npm_lifecycle_script: string;
	export const NVM_DIR: string;
	export const VSCODE_GIT_ASKPASS_EXTRA_ARGS: string;
	export const TERM: string;
	export const npm_package_name: string;
	export const _CE_CONDA: string;
	export const USER: string;
	export const VSCODE_GIT_IPC_HANDLE: string;
	export const CONDA_SHLVL: string;
	export const npm_lifecycle_event: string;
	export const SHLVL: string;
	export const NVM_CD_FLAGS: string;
	export const EMSDK_NODE: string;
	export const XPC_SERVICE_NAME: string;
	export const npm_config_user_agent: string;
	export const npm_execpath: string;
	export const CONDA_PYTHON_EXE: string;
	export const HOMEBREW_GITHUB_API_TOKEN: string;
	export const SSL_CERT_FILE: string;
	export const CONDA_DEFAULT_ENV: string;
	export const PYENV_ROOT: string;
	export const npm_package_json: string;
	export const BUN_INSTALL: string;
	export const VSCODE_GIT_ASKPASS_MAIN: string;
	export const PATH: string;
	export const ORIGINAL_XDG_CURRENT_DESKTOP: string;
	export const NVM_BIN: string;
	export const EMSDK: string;
	export const WASMER_CACHE_DIR: string;
	export const npm_node_execpath: string;
	export const WASMER_DIR: string;
	export const OLDPWD: string;
	export const GOPATH: string;
	export const __CF_USER_TEXT_ENCODING: string;
	export const TERM_PROGRAM: string;
	export const NODE_ENV: string;
}

/**
 * Similar to [`$env/static/private`](https://svelte.dev/docs/kit/$env-static-private), except that it only includes environment variables that begin with [`config.kit.env.publicPrefix`](https://svelte.dev/docs/kit/configuration#env) (which defaults to `PUBLIC_`), and can therefore safely be exposed to client-side code.
 * 
 * Values are replaced statically at build time.
 * 
 * ```ts
 * import { PUBLIC_BASE_URL } from '$env/static/public';
 * ```
 */
declare module '$env/static/public' {
	
}

/**
 * This module provides access to runtime environment variables, as defined by the platform you're running on. For example if you're using [`adapter-node`](https://github.com/sveltejs/kit/tree/main/packages/adapter-node) (or running [`vite preview`](https://svelte.dev/docs/kit/cli)), this is equivalent to `process.env`. This module only includes variables that _do not_ begin with [`config.kit.env.publicPrefix`](https://svelte.dev/docs/kit/configuration#env) _and do_ start with [`config.kit.env.privatePrefix`](https://svelte.dev/docs/kit/configuration#env) (if configured).
 * 
 * This module cannot be imported into client-side code.
 * 
 * Dynamic environment variables cannot be used during prerendering.
 * 
 * ```ts
 * import { env } from '$env/dynamic/private';
 * console.log(env.DEPLOYMENT_SPECIFIC_VARIABLE);
 * ```
 * 
 * > In `dev`, `$env/dynamic` always includes environment variables from `.env`. In `prod`, this behavior will depend on your adapter.
 */
declare module '$env/dynamic/private' {
	export const env: {
		SHELL: string;
		npm_command: string;
		COLORTERM: string;
		PYENV_SHELL: string;
		XPC_FLAGS: string;
		NVM_INC: string;
		TERM_PROGRAM_VERSION: string;
		CONDA_EXE: string;
		_CE_M: string;
		NODE: string;
		__CFBundleIdentifier: string;
		SSH_AUTH_SOCK: string;
		MallocNanoZone: string;
		npm_config_local_prefix: string;
		EDITOR: string;
		PWD: string;
		EMSDK_PYTHON: string;
		LOGNAME: string;
		CONDA_PREFIX: string;
		PNPM_HOME: string;
		_: string;
		VSCODE_GIT_ASKPASS_NODE: string;
		COMMAND_MODE: string;
		HOME: string;
		LANG: string;
		npm_package_version: string;
		WASMTIME_HOME: string;
		CONDA_PROMPT_MODIFIER: string;
		TMPDIR: string;
		GIT_ASKPASS: string;
		npm_lifecycle_script: string;
		NVM_DIR: string;
		VSCODE_GIT_ASKPASS_EXTRA_ARGS: string;
		TERM: string;
		npm_package_name: string;
		_CE_CONDA: string;
		USER: string;
		VSCODE_GIT_IPC_HANDLE: string;
		CONDA_SHLVL: string;
		npm_lifecycle_event: string;
		SHLVL: string;
		NVM_CD_FLAGS: string;
		EMSDK_NODE: string;
		XPC_SERVICE_NAME: string;
		npm_config_user_agent: string;
		npm_execpath: string;
		CONDA_PYTHON_EXE: string;
		HOMEBREW_GITHUB_API_TOKEN: string;
		SSL_CERT_FILE: string;
		CONDA_DEFAULT_ENV: string;
		PYENV_ROOT: string;
		npm_package_json: string;
		BUN_INSTALL: string;
		VSCODE_GIT_ASKPASS_MAIN: string;
		PATH: string;
		ORIGINAL_XDG_CURRENT_DESKTOP: string;
		NVM_BIN: string;
		EMSDK: string;
		WASMER_CACHE_DIR: string;
		npm_node_execpath: string;
		WASMER_DIR: string;
		OLDPWD: string;
		GOPATH: string;
		__CF_USER_TEXT_ENCODING: string;
		TERM_PROGRAM: string;
		NODE_ENV: string;
		[key: `PUBLIC_${string}`]: undefined;
		[key: `${string}`]: string | undefined;
	}
}

/**
 * Similar to [`$env/dynamic/private`](https://svelte.dev/docs/kit/$env-dynamic-private), but only includes variables that begin with [`config.kit.env.publicPrefix`](https://svelte.dev/docs/kit/configuration#env) (which defaults to `PUBLIC_`), and can therefore safely be exposed to client-side code.
 * 
 * Note that public dynamic environment variables must all be sent from the server to the client, causing larger network requests — when possible, use `$env/static/public` instead.
 * 
 * Dynamic environment variables cannot be used during prerendering.
 * 
 * ```ts
 * import { env } from '$env/dynamic/public';
 * console.log(env.PUBLIC_DEPLOYMENT_SPECIFIC_VARIABLE);
 * ```
 */
declare module '$env/dynamic/public' {
	export const env: {
		[key: `PUBLIC_${string}`]: string | undefined;
	}
}
