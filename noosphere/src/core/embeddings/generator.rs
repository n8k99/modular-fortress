//! Embedding generation via Ollama or OpenAI
//!
//! Supports multiple embedding providers with a unified interface.

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use tracing::{debug, warn};

/// Embedding dimension (768 for nomic-embed-text, can be configured)
pub const DEFAULT_EMBEDDING_DIM: usize = 768;

/// Embedding provider selection
#[derive(Debug, Clone, PartialEq)]
pub enum EmbeddingProvider {
    /// Ollama local inference (nomic-embed-text)
    Ollama,
    /// OpenAI API (text-embedding-3-small)
    OpenAI,
}

impl Default for EmbeddingProvider {
    fn default() -> Self {
        EmbeddingProvider::Ollama
    }
}

/// Configuration for embedding service
#[derive(Debug, Clone)]
pub struct EmbeddingConfig {
    pub provider: EmbeddingProvider,
    pub ollama_endpoint: String,
    pub ollama_model: String,
    pub openai_endpoint: String,
    pub openai_model: String,
    pub openai_api_key: Option<String>,
}

impl Default for EmbeddingConfig {
    fn default() -> Self {
        Self {
            provider: EmbeddingProvider::Ollama,
            ollama_endpoint: "http://localhost:11434".to_string(),
            ollama_model: "nomic-embed-text".to_string(),
            openai_endpoint: "https://api.openai.com/v1/embeddings".to_string(),
            openai_model: "text-embedding-3-small".to_string(),
            openai_api_key: std::env::var("OPENAI_API_KEY").ok(),
        }
    }
}

impl EmbeddingConfig {
    /// Create config for Ollama
    pub fn ollama() -> Self {
        Self {
            provider: EmbeddingProvider::Ollama,
            ..Default::default()
        }
    }

    /// Create config for OpenAI
    pub fn openai(api_key: String) -> Self {
        Self {
            provider: EmbeddingProvider::OpenAI,
            openai_api_key: Some(api_key),
            ..Default::default()
        }
    }

    /// Create config from environment, preferring Ollama if available
    pub fn from_env() -> Self {
        Self::default()
    }
}

/// Embedding service for generating vector representations
pub struct EmbeddingService {
    config: EmbeddingConfig,
    client: reqwest::Client,
}

impl EmbeddingService {
    pub fn new(config: EmbeddingConfig) -> Self {
        Self {
            config,
            client: reqwest::Client::new(),
        }
    }

    /// Create service with default config
    pub fn default_service() -> Self {
        Self::new(EmbeddingConfig::default())
    }

    /// Generate embedding for text
    pub async fn generate(&self, text: &str) -> Result<Vec<f32>> {
        match self.config.provider {
            EmbeddingProvider::Ollama => self.generate_ollama(text).await,
            EmbeddingProvider::OpenAI => self.generate_openai(text).await,
        }
    }

    /// Check if embedding service is available
    pub async fn is_available(&self) -> bool {
        match self.config.provider {
            EmbeddingProvider::Ollama => self.check_ollama().await,
            EmbeddingProvider::OpenAI => self.config.openai_api_key.is_some(),
        }
    }

    /// Generate embedding via Ollama
    async fn generate_ollama(&self, text: &str) -> Result<Vec<f32>> {
        let url = format!("{}/api/embeddings", self.config.ollama_endpoint);
        
        let request = OllamaEmbedRequest {
            model: self.config.ollama_model.clone(),
            prompt: text.to_string(),
        };

        debug!("Generating Ollama embedding for {} chars", text.len());

        let response = self.client
            .post(&url)
            .json(&request)
            .timeout(std::time::Duration::from_secs(30))
            .send()
            .await
            .map_err(|e| anyhow!("Ollama request failed: {}", e))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(anyhow!("Ollama error {}: {}", status, body));
        }

        let result: OllamaEmbedResponse = response
            .json()
            .await
            .map_err(|e| anyhow!("Failed to parse Ollama response: {}", e))?;

        Ok(result.embedding)
    }

    /// Generate embedding via OpenAI
    async fn generate_openai(&self, text: &str) -> Result<Vec<f32>> {
        let api_key = self.config.openai_api_key.as_ref()
            .ok_or_else(|| anyhow!("OpenAI API key not configured"))?;

        let request = OpenAIEmbedRequest {
            model: self.config.openai_model.clone(),
            input: text.to_string(),
        };

        debug!("Generating OpenAI embedding for {} chars", text.len());

        let response = self.client
            .post(&self.config.openai_endpoint)
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .timeout(std::time::Duration::from_secs(30))
            .send()
            .await
            .map_err(|e| anyhow!("OpenAI request failed: {}", e))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(anyhow!("OpenAI error {}: {}", status, body));
        }

        let result: OpenAIEmbedResponse = response
            .json()
            .await
            .map_err(|e| anyhow!("Failed to parse OpenAI response: {}", e))?;

        result.data.first()
            .map(|d| d.embedding.clone())
            .ok_or_else(|| anyhow!("No embedding in OpenAI response"))
    }

    /// Check if Ollama is available
    async fn check_ollama(&self) -> bool {
        let url = format!("{}/api/tags", self.config.ollama_endpoint);
        match self.client.get(&url).timeout(std::time::Duration::from_secs(5)).send().await {
            Ok(resp) => resp.status().is_success(),
            Err(e) => {
                warn!("Ollama not available: {}", e);
                false
            }
        }
    }
}

// ============ Ollama API Structures ============

#[derive(Debug, Serialize)]
struct OllamaEmbedRequest {
    model: String,
    prompt: String,
}

#[derive(Debug, Deserialize)]
struct OllamaEmbedResponse {
    embedding: Vec<f32>,
}

// ============ OpenAI API Structures ============

#[derive(Debug, Serialize)]
struct OpenAIEmbedRequest {
    model: String,
    input: String,
}

#[derive(Debug, Deserialize)]
struct OpenAIEmbedResponse {
    data: Vec<OpenAIEmbedding>,
}

#[derive(Debug, Deserialize)]
struct OpenAIEmbedding {
    embedding: Vec<f32>,
}

// ============ Convenience Function ============

/// Generate embedding using default service
/// 
/// This is a convenience function that uses the default configuration.
/// For more control, use `EmbeddingService` directly.
pub async fn generate_embedding(text: &str) -> Result<Vec<f32>> {
    let service = EmbeddingService::default_service();
    
    if !service.is_available().await {
        return Err(anyhow!("No embedding service available (Ollama not running, no OpenAI key)"));
    }
    
    service.generate(text).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_defaults() {
        let config = EmbeddingConfig::default();
        assert_eq!(config.provider, EmbeddingProvider::Ollama);
        assert_eq!(config.ollama_endpoint, "http://localhost:11434");
        assert_eq!(config.ollama_model, "nomic-embed-text");
    }

    #[test]
    fn test_config_openai() {
        let config = EmbeddingConfig::openai("test-key".to_string());
        assert_eq!(config.provider, EmbeddingProvider::OpenAI);
        assert_eq!(config.openai_api_key, Some("test-key".to_string()));
    }
}
