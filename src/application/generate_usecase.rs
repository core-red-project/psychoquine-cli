use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::application::registry::EngineRegistry;
use crate::application::verify_usecase::VerifyQuineUseCase;
use crate::domain::error::DomainError;
use crate::domain::language::Language;
use crate::domain::payload::{Payload, PayloadMode};
use crate::domain::quine::Quine;
use crate::domain::report::VerificationReport;
use crate::infrastructure::detector::SystemDetector;

#[derive(Debug, Clone)]
pub struct GenerateQuineRequest {
    pub language: Option<Language>,
    pub payload_mode: PayloadMode,
    pub dry_run: bool,
    pub verify: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateQuineResponse {
    pub quine: Quine,
    pub is_dry_run: bool,
    pub verification: Option<VerificationReport>,
}

pub struct GenerateQuineUseCase {
    registry: Arc<EngineRegistry>,
}

impl GenerateQuineUseCase {
    pub fn new(registry: Arc<EngineRegistry>) -> Self {
        Self { registry }
    }

    /// Select optimal language based on installed runtimes
    pub fn resolve_default_language(&self) -> Language {
        let all_runtimes = SystemDetector::detect_all();
        for rt in all_runtimes {
            if rt.available {
                return rt.language;
            }
        }
        Language::Python
    }

    pub fn execute(
        &self,
        request: GenerateQuineRequest,
    ) -> Result<GenerateQuineResponse, DomainError> {
        let language = match request.language {
            Some(l) => l,
            None => self.resolve_default_language(),
        };

        let engine = self.registry.get(language)?;
        let payload = Payload::try_from_mode(&request.payload_mode)?;
        let payload_bytes = payload.as_ref().map(|p| p.content.len()).unwrap_or(0);

        // Generate the quine
        let source_code = engine.generate(payload.as_ref())?;
        let quine = Quine::new(language, source_code, payload_bytes);

        // Handle verification or dry-run simulation
        let verification = if request.dry_run {
            let runtime = SystemDetector::detect_for_language(language);
            Some(VerificationReport {
                language,
                success: true,
                is_dry_run: true,
                runtime_binary: runtime.binary,
                source_bytes: quine.stats.bytes,
                output_bytes: quine.stats.bytes,
                sha256_source: quine.sha256.clone(),
                sha256_output: quine.sha256.clone(),
                diff_preview: None,
            })
        } else if request.verify {
            let verifier = VerifyQuineUseCase;
            let report = verifier.execute(language, &quine.source_code)?;
            Some(report)
        } else {
            None
        };

        Ok(GenerateQuineResponse {
            quine,
            is_dry_run: request.dry_run,
            verification,
        })
    }
}
