use crate::domain::error::DomainError;
use crate::domain::language::Language;
use crate::domain::payload::Payload;
use crate::engines::QuineEngine;

pub struct PythonEngine;

fn python_repr(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('\'');
    for c in s.chars() {
        match c {
            '\'' => out.push_str("\\'"),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            other => out.push(other),
        }
    }
    out.push('\'');
    out
}

impl QuineEngine for PythonEngine {
    fn language(&self) -> Language {
        Language::Python
    }

    fn generate(&self, payload: Option<&Payload>) -> Result<String, DomainError> {
        let comments = payload
            .map(|p| p.format_as_comment("# "))
            .unwrap_or_default();

        let code = if comments.is_empty() {
            let template = "s = {!r}\nprint(s.format(s), end=\"\")\n";
            format!(
                "s = {}\nprint(s.format(s), end=\"\")\n",
                python_repr(template)
            )
        } else {
            let template = format!("{}s = {{!r}}\nprint(s.format(s), end=\"\")\n", comments);
            format!(
                "{}s = {}\nprint(s.format(s), end=\"\")\n",
                comments,
                python_repr(&template)
            )
        };

        Ok(code)
    }
}
