use crate::domain::error::DomainError;
use crate::domain::language::Language;
use crate::domain::payload::Payload;
use crate::engines::QuineEngine;

pub struct RubyEngine;

impl QuineEngine for RubyEngine {
    fn language(&self) -> Language {
        Language::Ruby
    }

    fn generate(&self, payload: Option<&Payload>) -> Result<String, DomainError> {
        let comments = payload
            .map(|p| p.format_as_comment("# "))
            .unwrap_or_default();

        let code = if comments.is_empty() {
            "eval s=\"print 'eval s=';p s\"\n".to_string()
        } else {
            format!(
                "{}eval s=\"print '{}\\neval s=';p s\"\n",
                comments,
                comments.trim_end()
            )
        };

        Ok(code)
    }
}
