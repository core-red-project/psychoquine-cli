use crate::domain::error::DomainError;
use crate::domain::language::Language;
use crate::domain::payload::Payload;
use crate::engines::QuineEngine;

pub struct DosBatchEngine;

impl QuineEngine for DosBatchEngine {
    fn language(&self) -> Language {
        Language::DosBatch
    }

    fn generate(&self, _payload: Option<&Payload>) -> Result<String, DomainError> {
        // Canonical DOS / CMD Batch Quine
        Ok("@echo off\nset s=@echo off^&echo set s=%%s%%^&call echo %%s%%\necho set s=%s%&call echo %s%\n".to_string())
    }
}
