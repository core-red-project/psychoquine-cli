use crate::domain::error::DomainError;
use crate::domain::language::Language;
use crate::domain::payload::Payload;
use crate::engines::QuineEngine;

pub struct LuaEngine;

impl QuineEngine for LuaEngine {
    fn language(&self) -> Language {
        Language::Lua
    }

    fn generate(&self, payload: Option<&Payload>) -> Result<String, DomainError> {
        let comments = payload
            .map(|p| p.format_as_comment("-- "))
            .unwrap_or_default();

        let code = if comments.is_empty() {
            "s=\"s=%q;io.write(string.format(s,s))\";io.write(string.format(s,s))".to_string()
        } else {
            let tmpl = format!("{}s=%q;io.write(string.format(s,s))", comments);
            format!("{}s={:?};io.write(string.format(s,s))", comments, tmpl)
        };

        Ok(code)
    }
}
