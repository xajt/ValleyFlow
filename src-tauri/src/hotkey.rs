use crate::audio::AudioCapture;
use std::sync::{Arc, Mutex};
use tauri::{AppHandle, Emitter, Manager};

pub struct RecordingState {
    pub is_recording: bool,
    pub audio_capture: Arc<Mutex<AudioCapture>>,
}

pub fn toggle_recording(app: &AppHandle, state: &Arc<Mutex<RecordingState>>) {
    let mut recording_state = state.lock().unwrap();

    if recording_state.is_recording {
        // Stop recording
        if let Ok(samples) = recording_state.audio_capture.lock().unwrap().stop_recording() {
            log::info!("Recording stopped, {} samples", samples.len());

            // Emit event to frontend
            let _ = app.emit("recording-state", false);
            let _ = app.emit("recording-complete", samples.len());

            // TODO: In Task #2, we'll send this to Whisper for transcription
        }
        recording_state.is_recording = false;
    } else {
        // Start recording
        if let Err(e) = recording_state.audio_capture.lock().unwrap().start_recording() {
            log::error!("Failed to start recording: {}", e);
            let _ = app.emit("recording-error", e.to_string());
            return;
        }
        recording_state.is_recording = true;
        log::info!("Recording started");

        // Emit event to frontend
        let _ = app.emit("recording-state", true);
    }
}
