use anyhow::Result;
use log::{debug, info};
use reqwest::multipart;
use serde::{Deserialize, Serialize};
use std::env;

const GROQ_API_URL: &str = "https://api.groq.com/openai/v1/audio/transcriptions";

fn get_groq_api_key() -> Result<String> {
    // API key must be set via GROQ_API_KEY environment variable
    // Get your key from: https://console.groq.com/keys
    env::var("GROQ_API_KEY")
        .map_err(|_| anyhow::anyhow!("GROQ_API_KEY environment variable not set. Please set it with your Groq API key from https://console.groq.com/keys"))
}

#[derive(Debug, Serialize, Deserialize)]
struct GroqTranscriptionResponse {
    text: String,
}

pub struct GroqEngine {
    client: reqwest::Client,
}

impl GroqEngine {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    /// Transcribe audio samples using Groq API
    ///
    /// # Arguments
    /// * `audio` - Audio samples as f32 array (16kHz sample rate expected)
    /// * `language` - Optional language code (e.g., "es" for Spanish, "en" for English)
    ///
    /// # Returns
    /// Transcription text or error
    pub async fn transcribe_samples(
        &self,
        audio: Vec<f32>,
        language: Option<String>,
    ) -> Result<String> {
        let start = std::time::Instant::now();
        debug!("Starting Groq transcription for {} samples", audio.len());

        // Convert f32 samples to WAV file bytes
        let wav_bytes = self.samples_to_wav(&audio)?;
        debug!("Converted samples to WAV: {} bytes", wav_bytes.len());

        // Create multipart form
        let file_part = multipart::Part::bytes(wav_bytes)
            .file_name("audio.wav")
            .mime_str("audio/wav")?;

        let mut form = multipart::Form::new()
            .part("file", file_part)
            .text("model", "whisper-large-v3")
            .text("temperature", "0")
            .text("response_format", "json");

        // Add language if specified
        if let Some(lang) = language {
            if lang != "auto" {
                form = form.text("language", lang);
            }
        }

        // Make API request
        debug!("Sending request to Groq API");
        let api_key = get_groq_api_key()?;
        let response = self
            .client
            .post(GROQ_API_URL)
            .header("Authorization", format!("Bearer {}", api_key))
            .multipart(form)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(anyhow::anyhow!("Groq API error {}: {}", status, error_text));
        }

        let groq_response: GroqTranscriptionResponse = response.json().await?;

        let duration = start.elapsed();
        info!(
            "Groq transcription completed in {}ms: {}",
            duration.as_millis(),
            groq_response.text
        );

        Ok(groq_response.text.trim().to_string())
    }

    /// Convert f32 audio samples to WAV file bytes
    fn samples_to_wav(&self, samples: &[f32]) -> Result<Vec<u8>> {
        use std::io::Cursor;

        let mut cursor = Cursor::new(Vec::new());

        let spec = hound::WavSpec {
            channels: 1,
            sample_rate: 16000,
            bits_per_sample: 16,
            sample_format: hound::SampleFormat::Int,
        };

        let mut writer = hound::WavWriter::new(&mut cursor, spec)?;

        // Convert f32 samples (-1.0 to 1.0) to i16 samples
        for &sample in samples {
            let sample_i16 = (sample.clamp(-1.0, 1.0) * i16::MAX as f32) as i16;
            writer.write_sample(sample_i16)?;
        }

        writer.finalize()?;
        Ok(cursor.into_inner())
    }
}

impl Default for GroqEngine {
    fn default() -> Self {
        Self::new()
    }
}
