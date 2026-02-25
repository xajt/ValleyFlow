use anyhow::Result;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Device, Host, SampleFormat, Stream, StreamConfig};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

pub struct AudioCapture {
    host: Host,
    device: Option<Device>,
    stream: Option<Stream>,
    is_recording: Arc<AtomicBool>,
    audio_data: Arc<std::sync::Mutex<Vec<f32>>>,
}

impl AudioCapture {
    pub fn new() -> Result<Self> {
        let host = cpal::default_host();

        Ok(Self {
            host,
            device: None,
            stream: None,
            is_recording: Arc::new(AtomicBool::new(false)),
            audio_data: Arc::new(std::sync::Mutex::new(Vec::new())),
        })
    }

    pub fn list_devices(&self) -> Result<Vec<String>> {
        let devices: Vec<String> = self
            .host
            .input_devices()?
            .filter_map(|d| d.name().ok())
            .collect();
        Ok(devices)
    }

    pub fn select_device(&mut self, name: &str) -> Result<()> {
        let device = self
            .host
            .input_devices()?
            .find(|d| d.name().map(|n| n == name).unwrap_or(false))
            .ok_or_else(|| anyhow::anyhow!("Device not found: {}", name))?;

        self.device = Some(device);
        Ok(())
    }

    pub fn start_recording(&mut self) -> Result<()> {
        let device = self
            .device
            .as_ref()
            .or(self.host.default_input_device().as_ref())
            .ok_or_else(|| anyhow::anyhow!("No input device available"))?;

        let supported_config = device
            .default_input_config()
            .unwrap_or_else(|_| cpal::StreamConfig {
                channels: 1,
                sample_rate: cpal::SampleRate(16000),
                buffer_size: cpal::BufferSize::Default,
            });

        let config = StreamConfig {
            channels: supported_config.channels,
            sample_rate: supported_config.sample_rate,
            buffer_size: supported_config.buffer_size,
        };

        self.is_recording.store(true, Ordering::SeqCst);
        self.audio_data.lock().unwrap().clear();

        let is_recording = self.is_recording.clone();
        let audio_data = self.audio_data.clone();

        let stream = device.build_input_stream(
            &config,
            move |data: &[f32], _: &cpal::InputCallbackInfo| {
                if is_recording.load(Ordering::SeqCst) {
                    let mut buffer = audio_data.lock().unwrap();
                    buffer.extend_from_slice(data);
                }
            },
            |err| log::error!("Audio stream error: {}", err),
            None,
        )?;

        stream.play()?;
        self.stream = Some(stream);

        log::info!("Recording started");
        Ok(())
    }

    pub fn stop_recording(&mut self) -> Result<Vec<f32>> {
        self.is_recording.store(false, Ordering::SeqCst);
        self.stream = None;

        let data = self.audio_data.lock().unwrap().clone();
        log::info!("Recording stopped, {} samples captured", data.len());

        Ok(data)
    }

    pub fn is_recording(&self) -> bool {
        self.is_recording.load(Ordering::SeqCst)
    }

    pub fn save_to_wav(&self, samples: &[f32], path: &str) -> Result<()> {
        // Simple WAV file writer
        // For production, consider using the `hound` crate
        let spec = hound::WavSpec {
            channels: 1,
            sample_rate: 16000,
            bits_per_sample: 16,
            sample_format: hound::SampleFormat::Int,
        };

        let mut writer = hound::WavWriter::create(path, spec)?;
        for sample in samples {
            let amplitude = (sample * 32767.0) as i16;
            writer.write_sample(amplitude)?;
        }
        writer.finalize()?;

        log::info!("Audio saved to {}", path);
        Ok(())
    }
}

impl Default for AudioCapture {
    fn default() -> Self {
        Self::new().expect("Failed to initialize audio capture")
    }
}
