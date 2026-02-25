use crate::audio::AudioCapture;
use crate::clipboard::ClipboardManager;
use crate::deepseek::DeepSeekClient;
use crate::sound::SoundPlayer;
use crate::transcription::{resample_to_16k_mono, Transcriber};
use std::sync::{Arc, Mutex};
use tauri::{AppHandle, Emitter};

pub struct RecordingState {
    pub is_recording: bool,
    pub audio_capture: Arc<Mutex<AudioCapture>>,
    pub transcriber: Option<Arc<Mutex<Transcriber>>>,
    pub deepseek_client: Arc<Mutex<DeepSeekClient>>,
    pub clipboard: Arc<Mutex<ClipboardManager>>,
    pub sound_player: Arc<Mutex<SoundPlayer>>,
}

pub fn toggle_recording(app: &AppHandle, state: &Arc<Mutex<RecordingState>>) {
    let mut recording_state = state.lock().unwrap();

    if recording_state.is_recording {
        // Stop recording
        let samples = recording_state.audio_capture.lock().unwrap().stop_recording();

        match samples {
            Ok(samples) => {
                log::info!("Recording stopped, {} samples captured", samples.len());

                // Emit event to frontend
                let _ = app.emit("recording-state", false);
                let _ = app.emit("recording-processing", true);

                // Process the recording
                let transcriber = recording_state.transcriber.clone();
                let deepseek_client = recording_state.deepseek_client.clone();
                let clipboard = recording_state.clipboard.clone();
                let sound_player = recording_state.sound_player.clone();
                let app_handle = app.clone();

                // Spawn async task for processing
                tauri::async_runtime::spawn(async move {
                    if let Err(e) = process_recording(
                        samples,
                        transcriber,
                        deepseek_client,
                        clipboard,
                        sound_player,
                        &app_handle,
                    )
                    .await
                    {
                        log::error!("Failed to process recording: {}", e);
                        let _ = app_handle.emit("recording-error", e.to_string());
                    }
                    let _ = app_handle.emit("recording-processing", false);
                });
            }
            Err(e) => {
                log::error!("Failed to stop recording: {}", e);
                let _ = app.emit("recording-error", e.to_string());
            }
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

async fn process_recording(
    samples: Vec<f32>,
    transcriber: Option<Arc<Mutex<Transcriber>>>,
    deepseek_client: Arc<Mutex<DeepSeekClient>>,
    clipboard: Arc<Mutex<ClipboardManager>>,
    sound_player: Arc<Mutex<SoundPlayer>>,
    app: &AppHandle,
) -> anyhow::Result<()> {
    // Resample to 16kHz mono (Whisper requirement)
    // TODO: Get actual sample rate from audio capture
    let resampled = resample_to_16k_mono(&samples, 48000, 1);
    log::info!("Resampled to {} samples at 16kHz mono", resampled.len());

    // Transcribe with Whisper
    let (raw_text, language) = if let Some(transcriber) = transcriber {
        let mut t = transcriber.lock().unwrap();
        t.transcribe(&resampled)?
    } else {
        anyhow::bail!("Transcriber not initialized");
    };

    log::info!("Raw transcription: {}", raw_text);
    let _ = app.emit("transcription-raw", &raw_text);

    // Post-process with DeepSeek
    let final_text = {
        let client = deepseek_client.lock().unwrap();
        if client.has_api_key() {
            match client.process_text(&raw_text, language.into()).await {
                Ok(processed) => {
                    log::info!("Processed text: {}", processed);
                    processed
                }
                Err(e) => {
                    log::warn!("DeepSeek processing failed: {}, using raw text", e);
                    raw_text
                }
            }
        } else {
            log::info!("No DeepSeek API key, using raw transcription");
            raw_text
        }
    };

    // Copy to clipboard
    clipboard.lock().unwrap().copy_text(&final_text)?;
    let _ = app.emit("transcription-complete", &final_text);

    // Play success sound
    sound_player.lock().unwrap().play_success()?;

    log::info!("Transcription pipeline complete!");
    Ok(())
}
