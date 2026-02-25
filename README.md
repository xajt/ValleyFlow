# ValleyFlow

Windows Desktop Voice Transcription App - lightweight, offline-first, AI-powered.

## Features

- **System Tray App** - runs in background, minimal UI
- **Global Hotkey** - Ctrl+Shift+Space to start/stop recording
- **Local Whisper** - offline transcription (no cloud required)
- **DeepSeek Post-processing** - removes fillers, fixes punctuation, formats text
- **Clipboard Integration** - text ready to paste instantly
- **History** - 50 last transcriptions saved locally
- **Welcome Wizard** - easy first-time setup
- **Auto-start** - starts with Windows automatically
- **Auto-update** - updates in background

## Prerequisites

### Required

1. **Rust** - Install from https://rustup.rs
   ```powershell
   winget install Rustlang.Rustup
   ```

2. **Node.js 20+** - Install from https://nodejs.org
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

## Quick Start

```powershell
# Clone the repository
git clone https://github.com/xajt/ValleyFlow.git
cd ValleyFlow

# Copy environment file and add your API key
copy .env.example .env
# Edit .env and add your DeepSeek API key

# Install dependencies
pnpm install

# Download Whisper model (~244MB)
.\scripts\download-model.ps1

# Run in development mode
pnpm tauri:dev
```

## Build Installer

```powershell
# Build MSI installer with embedded model
.\scripts\build-installer.ps1 -Release
```

Output: `src-tauri/target/release/bundle/msi/ValleyFlow_1.0.0_x64.msi`

## Usage

1. Install and run ValleyFlow
2. Complete the Welcome Wizard (language, API key, microphone test)
3. Icon appears in system tray
4. Press **Ctrl+Shift+Space** to start recording
5. Speak (up to 5 minutes)
6. Press **Ctrl+Shift+Space** again to stop
7. Text is automatically in your clipboard - paste anywhere!

### Keyboard Shortcuts

| Shortcut | Action |
|----------|--------|
| `Ctrl+Shift+Space` | Start/Stop recording |
| `Ctrl+Shift+S` | Open Settings |

### Tray Menu

Right-click the tray icon for:
- **Settings** - Configure language, microphone, API key
- **Quit** - Close the application

## Configuration

### DeepSeek API Key

Get your API key from https://platform.deepseek.com/

Add it to `.env` file:
```
DEEPSEEK_API_KEY=your_api_key_here
```

Or configure in Settings window.

### Supported Languages

- Polish (PL)
- English (EN)

Auto-detection is supported.

## Architecture

```
┌─────────────────────────────────────────────────┐
│                 ValleyFlow                       │
├─────────────┬─────────────┬─────────────────────┤
│  Tray Icon  │   Overlay   │    Settings Window  │
│   (Rust)    │  (React)    │      (React)        │
├─────────────┴─────────────┴─────────────────────┤
│              Tauri Core (Rust)                   │
├──────────────┬──────────────┬───────────────────┤
│ Audio Capture │  whisper.cpp │   HTTP Client    │
│    (cpal)     │   (FFI)      │   (reqwest)     │
├──────────────┴──────────────┴───────────────────┤
│              LocalStorage (history)              │
└─────────────────────────────────────────────────┘
```

## Tech Stack

| Component | Technology |
|-----------|------------|
| Backend | Tauri v2 (Rust) |
| Frontend | React + TypeScript |
| Audio | cpal |
| Transcription | whisper.cpp (whisper-rs) |
| Post-processing | DeepSeek API |
| Clipboard | arboard |
| Installer | WiX (MSI) |

## Development

```powershell
# Run in dev mode
pnpm tauri:dev

# Build frontend only
pnpm build

# Build Rust only
cd src-tauri && cargo build --release

# Run tests
cd src-tauri && cargo test
```

## Project Structure

```
ValleyFlow/
├── src/                    # React frontend
│   ├── components/         # UI components
│   ├── store/              # State management
│   └── styles.css          # Styles
├── src-tauri/              # Rust backend
│   ├── src/
│   │   ├── main.rs         # Entry point
│   │   ├── audio.rs        # Audio capture
│   │   ├── transcription.rs # Whisper
│   │   ├── deepseek.rs     # API client
│   │   ├── clipboard.rs    # Clipboard
│   │   └── sound.rs        # Success sound
│   ├── models/             # Whisper models
│   └── tauri.conf.json     # Tauri config
├── scripts/                # Build scripts
└── package.json
```

## Troubleshooting

### "Whisper model not found"
Run the download script:
```powershell
.\scripts\download-model.ps1
```

### "Microphone not working"
1. Check Windows privacy settings
2. Allow microphone access for desktop apps
3. Restart ValleyFlow

### "DeepSeek API error"
1. Check your API key in Settings
2. Verify you have API credits
3. Check internet connection

## License

Proprietary - All rights reserved

## Links

- [GitHub Issues](https://github.com/xajt/ValleyFlow/issues)
- [Releases](https://github.com/xajt/ValleyFlow/releases)
