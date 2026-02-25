use anyhow::{Context, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;

const DEEPSEEK_API_URL: &str = "https://api.deepseek.com/v1/chat/completions";
const DEFAULT_MODEL: &str = "deepseek-chat";

#[derive(Debug, Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<Message>,
    temperature: f32,
    max_tokens: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct ChatResponse {
    choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
struct Choice {
    message: Message,
}

pub struct DeepSeekClient {
    client: Client,
    api_key: Option<String>,
}

impl DeepSeekClient {
    pub fn new() -> Self {
        let api_key = env::var("DEEPSEEK_API_KEY").ok();

        Self {
            client: Client::new(),
            api_key,
        }
    }

    pub fn set_api_key(&mut self, key: String) {
        self.api_key = Some(key);
    }

    pub fn has_api_key(&self) -> bool {
        self.api_key.is_some()
    }

    /// Process raw transcription text through DeepSeek
    pub async fn process_text(&self, raw_text: &str, language: &str) -> Result<String> {
        let api_key = self
            .api_key
            .as_ref()
            .context("DeepSeek API key not configured")?;

        let system_prompt = self.build_system_prompt(language);
        let user_prompt = format!("Input: {}\nOutput:", raw_text);

        let request = ChatRequest {
            model: DEFAULT_MODEL.to_string(),
            messages: vec![
                Message {
                    role: "system".to_string(),
                    content: system_prompt,
                },
                Message {
                    role: "user".to_string(),
                    content: user_prompt,
                },
            ],
            temperature: 0.3,
            max_tokens: 2048,
        };

        let response = self
            .client
            .post(DEEPSEEK_API_URL)
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .context("Failed to send request to DeepSeek API")?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            anyhow::bail!("DeepSeek API error: {} - {}", status, body);
        }

        let chat_response: ChatResponse = response
            .json()
            .await
            .context("Failed to parse DeepSeek response")?;

        let processed_text = chat_response
            .choices
            .first()
            .map(|c| c.message.content.trim().to_string())
            .unwrap_or_else(|| raw_text.to_string());

        log::info!("Text processed by DeepSeek: {} chars", processed_text.len());
        Ok(processed_text)
    }

    fn build_system_prompt(&self, language: &str) -> String {
        let lang_instruction = match language {
            "pl" => "Respond in Polish.",
            "en" => "Respond in English.",
            _ => "Respond in the same language as the input.",
        };

        format!(
            r#"You are a text cleaner and formatter. Your job is to process raw speech transcription.

Rules:
1. Remove ALL filler words: "eee", "yyy", "um", "uh", "ehm", "taki", "no", "właśnie" (Polish), etc.
2. Fix punctuation - add periods, commas, question marks where appropriate
3. Fix capitalization - sentences should start with capital letters
4. Detect context and format appropriately:
   - If it sounds like an email: format with greeting and proper structure
   - If it sounds like a note: keep it concise, use bullet points if appropriate
   - If it sounds like chat message: keep it casual but clean
5. Adjust tone to be natural and professional
6. Do NOT add any explanations or meta-commentary
7. Output ONLY the cleaned text, nothing else
8. {}

Examples:
Input: "eee więc yyy myślę że powinniśmy się spotkać jutro"
Output: "Myślę, że powinniśmy się spotkać jutro."

Input: "um so basically we need to finish the project by friday"
Output: "We need to finish the project by Friday."

Input: "cześć eee chciałem zapytać czy możesz mi przesłać ten raport"
Output: "Cześć, chciałem zapytać, czy możesz mi przesłać ten raport.""#,
            lang_instruction
        )
    }
}

impl Default for DeepSeekClient {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let client = DeepSeekClient::new();
        assert!(!client.has_api_key());
    }

    #[test]
    fn test_set_api_key() {
        let mut client = DeepSeekClient::new();
        client.set_api_key("test-key".to_string());
        assert!(client.has_api_key());
    }
}
