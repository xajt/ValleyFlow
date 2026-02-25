use anyhow::Result;
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};
use std::io::{BufReader, Cursor};

pub struct SoundPlayer {
    _stream: Option<OutputStream>,
    _handle: Option<OutputStreamHandle>,
    success_sound_data: Vec<u8>,
}

impl SoundPlayer {
    pub fn new() -> Self {
        Self {
            _stream: None,
            _handle: None,
            success_sound_data: generate_success_sound(),
        }
    }

    /// Play success sound (non-blocking)
    pub fn play_success(&mut self) -> Result<()> {
        let (_stream, handle) = OutputStream::try_default()?;
        let sink = Sink::try_new(&handle)?;

        let cursor = Cursor::new(self.success_sound_data.clone());
        let reader = BufReader::new(cursor);
        let source = Decoder::new(reader)?;
        sink.append(source);

        // Keep stream alive while playing
        self._stream = Some(_stream);
        self._handle = Some(handle);

        // Sink will play in background and drop when done
        sink.detach();

        log::info!("Success sound played");
        Ok(())
    }
}

impl Default for SoundPlayer {
    fn default() -> Self {
        Self::new()
    }
}

/// Generate a success sound WAV file in memory
/// Creates a pleasant 880Hz sine wave beep for 150ms with fade-in/fade-out
fn generate_success_sound() -> Vec<u8> {
    let sample_rate = 16000u32;
    let duration_ms = 150u32;
    let frequency = 880.0f32; // A5 note - pleasant and clear
    let fade_ms = 30u32; // Fade in/out duration

    let samples = (sample_rate * duration_ms / 1000) as usize;
    let fade_samples = (sample_rate * fade_ms / 1000) as usize;
    let data_size = samples as u32;

    let mut wav_data = Vec::with_capacity(44 + samples);

    // RIFF header
    wav_data.extend_from_slice(b"RIFF");
    let file_size = 36 + data_size;
    wav_data.extend_from_slice(&file_size.to_le_bytes());
    wav_data.extend_from_slice(b"WAVE");

    // fmt chunk
    wav_data.extend_from_slice(b"fmt ");
    wav_data.extend_from_slice(&16u32.to_le_bytes()); // chunk size
    wav_data.extend_from_slice(&1u16.to_le_bytes()); // PCM format
    wav_data.extend_from_slice(&1u16.to_le_bytes()); // mono
    wav_data.extend_from_slice(&sample_rate.to_le_bytes());
    wav_data.extend_from_slice(&sample_rate.to_le_bytes()); // byte rate
    wav_data.extend_from_slice(&1u16.to_le_bytes()); // block align
    wav_data.extend_from_slice(&8u16.to_le_bytes()); // bits per sample

    // data chunk
    wav_data.extend_from_slice(b"data");
    wav_data.extend_from_slice(&data_size.to_le_bytes());

    // Generate sine wave with envelope
    for i in 0..samples {
        let t = i as f32 / sample_rate as f32;

        // Sine wave
        let value = (2.0 * std::f32::consts::PI * frequency * t).sin();

        // Apply envelope (fade in/out)
        let envelope = if i < fade_samples {
            // Fade in
            i as f32 / fade_samples as f32
        } else if i > samples - fade_samples {
            // Fade out
            (samples - i) as f32 / fade_samples as f32
        } else {
            1.0
        };

        let amplitude = ((value * envelope + 1.0) * 127.5) as u8;
        wav_data.push(amplitude);
    }

    wav_data
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_sound() {
        let sound = generate_success_sound();
        assert!(&sound[..4] == b"RIFF");
        assert!(&sound[8..12] == b"WAVE");
        assert!(sound.len() > 44);
    }

    #[test]
    fn test_sound_player_creation() {
        let player = SoundPlayer::new();
        assert!(!player.success_sound_data.is_empty());
    }
}
