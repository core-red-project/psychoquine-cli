use crate::domain::language::Language;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuineStats {
    pub bytes: usize,
    pub lines: usize,
    pub expansion_ratio: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quine {
    pub language: Language,
    pub source_code: String,
    pub sha256: String,
    pub stats: QuineStats,
    pub has_payload: bool,
}

impl Quine {
    pub fn new(language: Language, source_code: String, payload_bytes: usize) -> Self {
        let sha256 = Self::calculate_sha256(&source_code);
        let bytes = source_code.len();
        let lines = source_code.lines().count().max(1);

        let expansion_ratio = if payload_bytes > 0 {
            bytes as f64 / payload_bytes as f64
        } else {
            1.0
        };

        let stats = QuineStats {
            bytes,
            lines,
            expansion_ratio,
        };

        Self {
            language,
            source_code,
            sha256,
            stats,
            has_payload: payload_bytes > 0,
        }
    }

    pub fn calculate_sha256(text: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(text.as_bytes());
        format!("{:x}", hasher.finalize())
    }
}
