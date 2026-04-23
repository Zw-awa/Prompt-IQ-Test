use serde::{Deserialize, Serialize};
use std::time::Duration;
use reqwest::{Client, Response};
use anyhow::{Result, Context};
use log::{error, warn, info};

#[derive(Debug, Clone)]
pub struct AiClient {
    client: Client,
    base_url: String,
    api_key: String,
    model: String,
    temperature: f32,
    max_tokens: Option<u32>,
    timeout_ms: u32,
}

#[derive(Debug, Serialize)]
pub struct ChatCompletionRequest {
    pub model: String,
    pub messages: Vec<ChatMessage>,
    pub temperature: Option<f32>,
    pub max_tokens: Option<u32>,
    pub top_p: Option<f32>,
    pub response_format: Option<ResponseFormat>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Serialize)]
pub struct ResponseFormat {
    #[serde(rename = "type")]
    pub format_type: String,
}

#[derive(Debug, Deserialize)]
pub struct ChatCompletionResponse {
    pub id: Option<String>,
    pub choices: Vec<Choice>,
    pub usage: Option<Usage>,
}

#[derive(Debug, Deserialize)]
pub struct Choice {
    pub message: ChatMessage,
    pub finish_reason: Option<String>,
    pub index: u32,
}

#[derive(Debug, Deserialize)]
pub struct Usage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

#[derive(Debug, Deserialize)]
pub struct ApiError {
    pub error: ApiErrorDetail,
}

#[derive(Debug, Deserialize)]
pub struct ApiErrorDetail {
    pub message: String,
    #[serde(rename = "type")]
    pub error_type: Option<String>,
    pub code: Option<String>,
}

impl AiClient {
    pub fn new(
        base_url: String,
        api_key: String,
        model: String,
        temperature: f32,
        max_tokens: Option<u32>,
        timeout_ms: u32,
    ) -> Result<Self> {
        let client = Client::builder()
            .timeout(Duration::from_millis(timeout_ms as u64))
            .build()
            .context("Failed to build HTTP client")?;
        
        Ok(Self {
            client,
            base_url,
            api_key,
            model,
            temperature,
            max_tokens,
            timeout_ms,
        })
    }
    
    pub async fn chat_completion(
        &self,
        messages: Vec<ChatMessage>,
        temperature_override: Option<f32>,
        max_tokens_override: Option<u32>,
    ) -> Result<ChatCompletionResponse> {
        let url = format!("{}/chat/completions", self.base_url.trim_end_matches('/'));
        
        let request = ChatCompletionRequest {
            model: self.model.clone(),
            messages,
            temperature: temperature_override.or(Some(self.temperature)),
            max_tokens: max_tokens_override.or(self.max_tokens),
            top_p: Some(1.0),
            response_format: Some(ResponseFormat {
                format_type: "text".to_string(),
            }),
        };
        
        info!("Sending chat completion request to {}", url);
        
        let response = self.client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .context("Failed to send request to AI API")?;
        
        self.handle_response(response).await
    }
    
    pub async fn chat_completion_json(
        &self,
        messages: Vec<ChatMessage>,
        temperature_override: Option<f32>,
        max_tokens_override: Option<u32>,
    ) -> Result<ChatCompletionResponse> {
        let url = format!("{}/chat/completions", self.base_url.trim_end_matches('/'));
        
        let request = ChatCompletionRequest {
            model: self.model.clone(),
            messages,
            temperature: temperature_override.or(Some(self.temperature)),
            max_tokens: max_tokens_override.or(self.max_tokens),
            top_p: Some(1.0),
            response_format: Some(ResponseFormat {
                format_type: "json_object".to_string(),
            }),
        };
        
        info!("Sending JSON chat completion request to {}", url);
        
        let response = self.client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .context("Failed to send request to AI API")?;
        
        self.handle_response(response).await
    }
    
    async fn handle_response(&self, response: Response) -> Result<ChatCompletionResponse> {
        let status = response.status();
        let response_text = response.text().await
            .context("Failed to read response body")?;
        
        if !status.is_success() {
            error!("AI API error: HTTP {} - {}", status, response_text);
            
            // Try to parse error response
            if let Ok(api_error) = serde_json::from_str::<ApiError>(&response_text) {
                return Err(anyhow::anyhow!("AI API error: {}", api_error.error.message));
            } else {
                return Err(anyhow::anyhow!("AI API error: HTTP {} - {}", status, response_text));
            }
        }
        
        serde_json::from_str(&response_text)
            .context("Failed to parse AI API response")
    }
}

// Helper function to create a client from a Profile
pub fn client_from_profile(profile: &crate::Profile) -> Result<AiClient> {
    let api_key = profile.api_key.as_ref()
        .ok_or_else(|| anyhow::anyhow!("API key is missing for profile"))?;
    
    AiClient::new(
        profile.base_url.clone(),
        api_key.clone(),
        profile.model.clone(),
        profile.temperature,
        profile.max_tokens,
        profile.timeout_ms,
    )
}