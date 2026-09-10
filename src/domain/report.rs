use crate::domain::language::Language;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationReport {
    pub language: Language,
    pub success: bool,
    pub is_dry_run: bool,
    pub runtime_binary: String,
    pub source_bytes: usize,
    pub output_bytes: usize,
    pub sha256_source: String,
    pub sha256_output: String,
    pub diff_preview: Option<String>,
}
