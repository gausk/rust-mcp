use anyhow::Result;
use async_trait::async_trait;
use reqwest::Client as HttpClient;

use crate::model::{CompletionRequest, CompletionResponse};

#[async_trait]
pub trait ChatClient: Sync + Send {
    async fn complete(&self, request: CompletionRequest) -> Result<CompletionResponse>;
}

pub struct OpenAIClient {
    pub base_url: String,
    pub client: HttpClient,
    pub api_key: String,
}

impl OpenAIClient {
    pub fn new(api_key: String, url: Option<String>) -> Self {
        Self {
            api_key,
            client: HttpClient::new(),
            base_url: url.unwrap_or("https://api.openai.com/v1/chat/completions".to_string()),
        }
    }

    pub fn with_base_url(&mut self, url: impl Into<String>) {
        self.base_url = url.into();
    }
}

#[async_trait]
impl ChatClient for OpenAIClient {
    async fn complete(&self, request: CompletionRequest) -> anyhow::Result<CompletionResponse> {
        let response = self
            .client
            .post(&self.base_url)
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&request)
            .send()
            .await?;

        let status = response.status();
        let response_text: String = response.text().await?;

        if status.is_success() {
            let completion: CompletionResponse = serde_json::from_str(&response_text)?;
            Ok(completion)
        } else {
            Err(anyhow::anyhow!("API Err ({}): {}", status, response_text))
        }
    }
}
