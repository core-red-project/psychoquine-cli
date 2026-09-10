use crate::domain::error::DomainError;
use crate::domain::language::Language;
use crate::domain::payload::Payload;
use crate::engines::QuineEngine;

pub struct PerlEngine;

impl QuineEngine for PerlEngine {
    fn language(&self) -> Language {
        Language::Perl
    }

    fn generate(&self, payload: Option<&Payload>) -> Result<String, DomainError> {
        let comments = payload
            .map(|p| p.format_as_comment("# "))
            .unwrap_or_default();

        let code = if comments.is_empty() {
            "$_=q(print\"\\$_=q($_);eval\");eval".to_string()
        } else {
            format!(
                "{}$_=q(print\"{}\\$_=q($_);eval\");eval",
                comments, comments
            )
        };

        Ok(code)
    }
}
