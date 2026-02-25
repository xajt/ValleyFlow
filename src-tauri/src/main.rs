//! ValleyFlow - Windows Desktop Voice Transcription App
//!
//! Main entry point for the Tauri application.

mod audio;
mod autostart;
mod clipboard;
mod deepseek;
mod hotkey;
mod sound;
mod tray;
mod transcription;

use audio::AudioCapture;
use clipboard::ClipboardManager;
use deepseek::DeepSeekClient;
use hotkey::RecordingState;
use sound::SoundPlayer;
use std::sync::{Arc, Mutex};
use tauri::Manager;
use transcription::Transcriber;

fn main() {
    // Initialize logger
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .init();

    log::info!("Starting ValleyFlow...");

    // Load environment variables from .env file
    if let Err(e) = dotenvy::from_filename(".env") {
        log::debug!("No .env file found: {}", e);
    }

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .setup(|app| {
            // Initialize audio capture
            let audio_capture = Arc::new(Mutex::new(AudioCapture::new()?));

            // Initialize transcriber (will fail gracefully if model not found)
            let transcriber = match Transcriber::new(None) {
                Ok(t) => Some(Arc::new(Mutex::new(t))),
                Err(e) => {
                    log::warn!("Transcriber not initialized: {}. Download model first.", e);
                    None
                }
            };

            // Initialize DeepSeek client
            let deepseek_client = Arc::new(Mutex::new(DeepSeekClient::new()));

            // Initialize clipboard manager
            let clipboard = Arc::new(Mutex::new(ClipboardManager::new()?));

            // Initialize sound player
            let sound_player = Arc::new(Mutex::new(SoundPlayer::new()));

            // Initialize recording state
            let recording_state = Arc::new(Mutex::new(RecordingState {
                is_recording: false,
                audio_capture: audio_capture.clone(),
                transcriber: transcriber.clone(),
                deepseek_client: deepseek_client.clone(),
                clipboard: clipboard.clone(),
                sound_player: sound_player.clone(),
            }));

            // Store state in app
            app.manage(recording_state.clone());
            app.manage(deepseek_client.clone());

            // Setup system tray
            tray::setup_tray(app)?;

            // Setup global hotkey
            let shortcut = Shortcut::new(
                Some(
                    tauri_plugin_global_shortcut::Modifiers::CONTROL
                        | tauri_plugin_global_shortcut::Modifiers::SHIFT,
                ),
                tauri_plugin_global_shortcut::Code::Space,
            );

            let state_for_hotkey = recording_state.clone();
            app.global_shortcut().on_shortcut(
                shortcut,
                move |_app, _shortcut, _event| {
                    log::info!("Hotkey pressed: Ctrl+Shift+Space");
                    hotkey::toggle_recording(&_app.app_handle(), &state_for_hotkey);
                },
            )?;

            // Enable autostart by default
            if let Err(e) = autostart::enable_autostart() {
                log::warn!("Failed to enable autostart: {}", e);
            }

            // Hide main window on startup (tray-only app)
            if let Some(window) = app.get_webview_window("main") {
                window.hide()?;
            }

            log::info!("ValleyFlow initialized successfully");
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
