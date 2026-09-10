use crate::domain::error::DomainError;
use crate::domain::language::Language;
use crate::domain::payload::Payload;
use crate::engines::QuineEngine;

pub struct JavascriptEngine;

impl QuineEngine for JavascriptEngine {
    fn language(&self) -> Language {
        Language::Javascript
    }

    fn generate(&self, payload: Option<&Payload>) -> Result<String, DomainError> {
        let comments = payload
            .map(|p| p.format_as_comment("// "))
            .unwrap_or_default();

        let code = if comments.is_empty() {
            let template = "s=%j;console.log(s,s)";
            let serialized = serde_json::to_string(template)?;
            format!("s={};console.log(s,s)\n", serialized)
        } else {
            let template = format!("{}s=%j;console.log(s,s)", comments);
            let serialized = serde_json::to_string(&template)?;
            format!("{}s={};console.log(s,s)\n", comments, serialized)
        };

        Ok(code)
    }
}
