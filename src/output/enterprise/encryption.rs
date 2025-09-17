//! Stub encryption module for alpha release

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptionConfig {
    pub algorithm: String,
    pub key_size: usize,
}

impl Default for EncryptionConfig {
    fn default() -> Self {
        Self {
            algorithm: "AES-256".to_string(),
            key_size: 256,
        }
    }
}

pub struct EncryptionService;

impl EncryptionService {
    pub fn new(_config: &EncryptionConfig) -> anyhow::Result<Self> {
        Ok(Self)
    }

    pub fn disabled() -> Self {
        Self
    }

    pub async fn encrypt_analysis_results(
        &self,
        _results: &crate::types::AnalysisResults,
    ) -> anyhow::Result<EncryptedData> {
        Ok(EncryptedData {
            encrypted_payload: vec![],
            key_id: "stub".to_string(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct EncryptedData {
    pub encrypted_payload: Vec<u8>,
    pub key_id: String,
}
