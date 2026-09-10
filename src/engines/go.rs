use crate::domain::error::DomainError;
use crate::domain::language::Language;
use crate::domain::payload::Payload;
use crate::engines::QuineEngine;

pub struct GoEngine;

impl QuineEngine for GoEngine {
    fn language(&self) -> Language {
        Language::Go
    }

    fn generate(&self, payload: Option<&Payload>) -> Result<String, DomainError> {
        let comments = payload
            .map(|p| p.format_as_comment("// "))
            .unwrap_or_default();

        let code = if comments.is_empty() {
            "package main;import\"fmt\";func main(){s:=\"package main;import\\\"fmt\\\";func main(){s:=%q;fmt.Printf(s,s)}\";fmt.Printf(s,s)}\n".to_string()
        } else {
            let tmpl = format!(
                "{}package main;import\"fmt\";func main(){{s:=%q;fmt.Printf(s,s)}}",
                comments
            );
            format!(
                "{}package main;import\"fmt\";func main(){{s:=\"{}\";fmt.Printf(s,s)}}\n",
                comments,
                tmpl.replace('\\', "\\\\").replace('"', "\\\"")
            )
        };

        Ok(code)
    }
}
