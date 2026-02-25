use anyhow::{Context, Result};
use std::path::PathBuf;
use whisper_rs::{FullParams, SamplingStrategy, WhisperContext, WhisperContextParameters};

/// Language detection result
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Language {
    Polish,
    English,
    Unknown,
}

impl From<Language> for &str {
    fn from(lang: Language) -> Self {
        match lang {
            Language::Polish => "pl",
            Language::English => "en",
            Language::Unknown => "auto",
        }
    }
}

pub struct Transcriber {
    ctx: WhisperContext,
    model_path: PathBuf,
}

impl Transcriber {
    /// Initialize transcriber with Whisper model
    pub fn new(model_path: Option<PathBuf>) -> Result<Self> {
        let model_path = model_path.unwrap_or_else(|| {
            dirs::data_local_dir()
                .unwrap_or_else(|| PathBuf::from("."))
                .join("ValleyFlow")
                .join("models")
                .join("ggml-small.bin")
        });

        log::info!("Loading Whisper model from: {:?}", model_path);

        if !model_path.exists() {
            anyhow::bail!(
                "Whisper model not found at {:?}. Please download ggml-small.bin from https://huggingface.co/ggerganov/whisper.cpp",
                model_path
            );
        }

        let ctx_params = WhisperContextParameters::default();
        let ctx = WhisperContext::new_with_params(model_path.to_str().unwrap(), ctx_params)
            .context("Failed to create Whisper context")?;

        log::info!("Whisper model loaded successfully");

        Ok(Self { ctx, model_path })
    }

    /// Transcribe audio samples to text
    pub fn transcribe(&mut self, samples: &[f32]) -> Result<(String, Language)> {
        // Create params
        let mut params = FullParams::new(SamplingStrategy::Greedy { best_of: 1 });

        // Configure for speed and accuracy balance
        params.set_n_threads(4);
        params.set_translate(false);
        params.set_no_context(true);
        params.set_single_segment(true);
        params.set_print_special(false);
        params.set_print_progress(false);
        params.set_print_realtime(false);
        params.set_print_timestamps(false);

        // Auto-detect language
        params.set_language(None);

        // Run transcription
        let state = self
            .ctx
            .full(params, samples)
            .context("Failed to run transcription")?;

        // Extract text
        let mut text = String::new();
        let mut detected_lang = Language::Unknown;

        for i in 0..state.full_n_segments() {
            let segment = state.full_get_segment_text(i);
            text.push_str(&segment);
            text.push(' ');

            // Get detected language from first segment
            if i == 0 {
                let lang_id = state.full_lang_id();
                detected_lang = match lang_id {
                    18 => Language::Polish,  // Polish language ID in Whisper
                    0 => Language::English, // English language ID
                    _ => Language::Unknown,
                };
            }
        }

        let text = text.trim().to_string();
        log::info!("Transcription complete: {} chars, detected language: {:?}", text.len(), detected_lang);

        Ok((text, detected_lang))
    }

    /// Get model path
    pub fn model_path(&self) -> &PathBuf {
        &self.model_path
    }
}

/// Convert audio samples to 16kHz mono (Whisper requirement)
pub fn resample_to_16k_mono(samples: &[f32], original_sample_rate: u32, channels: u16) -> Vec<f32> {
    let mono: Vec<f32> = if channels > 1 {
        // Convert to mono by averaging channels
        samples
            .chunks(channels as usize)
            .map(|chunk| chunk.iter().sum::<f32>() / channels as f32)
            .collect()
    } else {
        samples.to_vec()
    };

    if original_sample_rate == 16000 {
        return mono;
    }

    // Simple linear resampling
    let ratio = 16000.0 / original_sample_rate as f64;
    let new_len = (mono.len() as f64 * ratio) as usize;
    let mut resampled = Vec::with_capacity(new_len);

    for i in 0..new_len {
        let src_idx = (i as f64 / ratio) as usize;
        let sample = mono.get(src_idx).copied().unwrap_or(0.0);
        resampled.push(sample);
    }

    resampled
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_language_conversion() {
        assert_eq!(<&str>::from(Language::Polish), "pl");
        assert_eq!(<&str>::from(Language::English), "en");
        assert_eq!(<&str>::from(Language::Unknown), "auto");
    }
}
