use crate::domain::error::DomainError;
use crate::domain::language::Language;
use crate::domain::payload::Payload;
use crate::engines::QuineEngine;

pub struct PhpEngine;

impl QuineEngine for PhpEngine {
    fn language(&self) -> Language {
        Language::Php
    }

    fn generate(&self, payload: Option<&Payload>) -> Result<String, DomainError> {
        let comments = payload
            .map(|p| p.format_as_comment("// "))
            .unwrap_or_default();

        let code = if comments.is_empty() {
            "<?php $s='<?php $s=%c%s%c;printf($s,39,$s,39);';printf($s,39,$s,39);".to_string()
        } else {
            let tmpl = format!("{}<?php $s=%c%s%c;printf($s,39,$s,39);", comments);
            format!("{}<?php $s='{}';printf($s,39,$s,39);", comments, tmpl)
        };

        Ok(code)
    }
}
