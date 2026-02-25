//! ValleyFlow - Windows Desktop Voice Transcription App
//!
//! Main entry point for the Tauri application.

mod audio;
mod autostart;
mod hotkey;
mod tray;

use audio::AudioCapture;
use hotkey::RecordingState;
use std::sync::{Arc, Mutex};
use tauri::Manager;
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut};

fn main() {
    // Initialize logger
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .init();

    log::info!("Starting ValleyFlow...");

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .setup(|app| {
            // Initialize audio capture
            let audio_capture = Arc::new(Mutex::new(AudioCapture::new()?));

            // Initialize recording state
            let recording_state = Arc::new(Mutex::new(RecordingState {
                is_recording: false,
                audio_capture: audio_capture.clone(),
            }));

            // Store state in app
            app.manage(recording_state.clone());

            // Setup system tray
            tray::setup_tray(app)?;

            // Setup global hotkey
            let shortcut = Shortcut::new(Some(tauri_plugin_global_shortcut::Modifiers::CONTROL | tauri_plugin_global_shortcut::Modifiers::SHIFT), tauri_plugin_global_shortcut::Code::Space);

            let state_for_hotkey = recording_state.clone();
            app.global_shortcut().on_shortcut(shortcut, move |_app, _shortcut, _event| {
                log::info!("Hotkey pressed: Ctrl+Shift+Space");
                hotkey::toggle_recording(&_app.app_handle(), &state_for_hotkey);
            })?;

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
