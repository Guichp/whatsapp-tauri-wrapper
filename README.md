# WhatsApp Desktop Tauri

> A lightweight desktop wrapper for WhatsApp Web built with Tauri.

<!-- Replace this section with a project screenshot or demo GIF. -->

## Overview

<!-- Describe what this project does and why it exists. -->

This project packages [WhatsApp Web](https://web.whatsapp.com) as a desktop application using Tauri. The goal of this is detaching WhatsApp from the browser to avoid distractions and reduce the resource usage of it.

## Features

- Use WhatsApp Web in a standalone desktop window
- Lightweight native application built with Tauri
- Linux bundles supported through `.deb` and AppImage targets

<!-- Add project-specific features here. -->

## Requirements

- Node.js and npm
- Rust and Cargo
- Tauri system dependencies for your operating system

See the [Tauri prerequisites](https://v2.tauri.app/start/prerequisites/) for platform-specific requirements.

## Installation

Clone the repository and install the JavaScript dependencies:

```bash
git clone <repository-url>
cd whatsapp-desktop-tauri
npm install
```

## Development

Start the application in development mode:

```bash
npm run tauri dev
```

The frontend can also be run independently with:

```bash
npm run dev
```

## Build

Build the frontend:

```bash
npm run build
```

Build the desktop application and generate distributable bundles:

```bash
npm run tauri build
```

Generated artifacts are written under `src-tauri/target/release/bundle/`.

## Project Structure

```text
.
├── src/                  # Frontend source files
├── src-tauri/            # Rust and Tauri configuration
│   ├── icons/             # Application icons
│   ├── src/               # Rust application code
│   └── tauri.conf.json    # Tauri configuration
├── index.html             # Frontend entry point
├── package.json           # JavaScript scripts and dependencies
└── vite.config.ts         # Vite configuration
```

## Configuration

The application window and bundle settings are defined in `src-tauri/tauri.conf.json`.

<!-- Document environment variables, permissions, window behavior, or custom settings here. -->

## Testing

<!-- Add test commands when tests are introduced. -->

Currently, TypeScript validation and the production frontend build can be run with:

```bash
npm run build
```

## License

<!-- Add the project license and attribution details here. -->
