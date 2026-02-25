# ValleyFlow

Windows Desktop Voice Transcription App - lightweight, offline-first, AI-powered.

## Features

- **System Tray App** - runs in background, minimal UI
- **Global Hotkey** - Ctrl+Shift+Space to start/stop recording
- **Local Whisper** - offline transcription (no cloud required)
- **DeepSeek Post-processing** - removes fillers, fixes punctuation, formats text
- **Clipboard Integration** - text ready to paste instantly
- **History** - 50 last transcriptions saved locally

## Prerequisites

1. **Rust** - Install from https://rustup.rs
   ```powershell
   winget install Rustlang.Rustup
   ```

2. **Node.js** - Install from https://nodejs.org
   ```powershell
   winget install OpenJS.NodeJS.LTS
   ```

3. **pnpm**
   ```powershell
   npm install -g pnpm
   ```

4. **Visual Studio Build Tools** (for Rust on Windows)
   ```powershell
   winget install Microsoft.VisualStudio.2022.BuildTools
   ```
   - Select "Desktop development with C++" workload

## Setup

```powershell
# Clone the repository
git clone https://github.com/xajt/ValleyFlow.git
cd ValleyFlow

# Install dependencies
pnpm install

# Install Tauri CLI (if not already installed)
pnpm add -D @tauri-apps/cli

# Run in development mode
pnpm tauri:dev
```

## Build

```powershell
# Build for production
pnpm tauri:build
```

Output: `src-tauri/target/release/bundle/msi/ValleyFlow_1.0.0_x64.msi`

## Usage

1. Install and run ValleyFlow
2. Icon appears in system tray
3. Press **Ctrl+Shift+Space** to start recording
4. Speak (up to 5 minutes)
5. Press **Ctrl+Shift+Space** again to stop
6. Text is automatically in your clipboard - paste anywhere!

## Configuration

- **Settings** - Right-click tray icon → Settings
- **API Key** - Configure DeepSeek API key in Settings
- **Language** - Polish and English supported (auto-detect)
- **Microphone** - Select input device in Settings

## Tech Stack

- **Tauri v2** - Rust backend + TypeScript/React frontend
- **whisper.cpp** - Local speech recognition
- **DeepSeek API** - Text post-processing
- **SQLite** - Local history storage

## License

Proprietary - All rights reserved

## Development Status

See [GitHub Issues](https://github.com/xajt/ValleyFlow/issues) for current progress.
