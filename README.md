# Blipty

Blipty is a modern streaming media companion application that helps you manage and watch your streaming content from various providers.

## Features

- Manage multiple streaming providers in one place
- Browse channels and content with an intuitive interface
- Watch streams directly within the application
- Cross-platform desktop application (macOS, Windows, Linux)
- Support for various streaming formats including HLS and MPEG-TS

## Development

Blipty is built with:

- [Tauri 2](https://tauri.app/) - For cross-platform desktop app functionality
- [Svelte 5](https://svelte.dev/) - For reactive UI components
- [ShadCN](https://next.shadcn-svelte.com/) - For beautiful UI components

### Requirements

To develop Blipty, you need:

- [Bun](https://bun.sh/docs/installation) - JavaScript runtime and package manager
- [Rust](https://www.rust-lang.org/tools/install) - Required for Tauri
- If on Windows, [MSVC](https://visualstudio.microsoft.com/vs/community/) with "Desktop development with C++" workload

### Setup

```bash
# Clone the repository
git clone https://github.com/yourusername/blipty.git
cd blipty

# Install dependencies
bun install
```

### Development Commands

```bash
# Start development server
bun run tauri dev

# Build executable
bun run tauri build
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Website

Visit [https://blipty.net](https://blipty.net) for more information.