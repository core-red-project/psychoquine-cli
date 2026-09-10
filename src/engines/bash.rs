use crate::domain::error::DomainError;
use crate::domain::language::Language;
use crate::domain::payload::Payload;
use crate::engines::QuineEngine;

pub struct BashEngine;

impl QuineEngine for BashEngine {
    fn language(&self) -> Language {
        Language::Bash
    }

    fn generate(&self, payload: Option<&Payload>) -> Result<String, DomainError> {
        let code = match payload {
            None => "s='s=\\47%s\\47;printf \"$s\" \"$s\"';printf \"$s\" \"$s\"".to_string(),
            Some(p) => {
                let comment_lines = p
                    .content
                    .lines()
                    .map(|l| format!("# {}", l))
                    .collect::<Vec<_>>()
                    .join("\n");
                let comment_block = format!("{}\n", comment_lines);
                let tmpl = format!("{}s=\\47%s\\47;printf \"$s\" \"$s\"", comment_block);
                format!("{}s='{}';printf \"$s\" \"$s\"", comment_block, tmpl)
            }
        };

        Ok(code)
    }
}
