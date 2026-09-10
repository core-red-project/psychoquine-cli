use crate::domain::error::DomainError;
use crate::domain::language::Language;
use crate::domain::payload::Payload;
use crate::engines::QuineEngine;

pub struct RustEngine;

impl QuineEngine for RustEngine {
    fn language(&self) -> Language {
        Language::Rust
    }

    fn generate(&self, payload: Option<&Payload>) -> Result<String, DomainError> {
        let code = match payload {
            None => {
                "fn main(){print!(\"fn main(){{print!({0:?},{0:?});}}\\n\",\"fn main(){{print!({0:?},{0:?});}}\\n\");}\n".to_string()
            }
            Some(p) => {
                let clean = p.content.replace("*/", "* /");
                let comment_source = format!("/* {} */\n", clean.trim());
                let comment_escaped = comment_source
                    .replace('\\', "\\\\")
                    .replace('\n', "\\n")
                    .replace('"', "\\\"");
                let s = format!(
                    "{}fn main(){{{{print!({{0:?}},{{0:?}});}}}}\\n",
                    comment_escaped
                );
                format!(
                    "{}fn main(){{print!(\"{}\",\"{}\");}}\n",
                    comment_source, s, s
                )
            }
        };

        Ok(code)
    }
}
