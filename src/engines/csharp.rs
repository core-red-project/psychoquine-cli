use crate::domain::error::DomainError;
use crate::domain::language::Language;
use crate::domain::payload::Payload;
use crate::engines::QuineEngine;

pub struct CSharpEngine;

impl QuineEngine for CSharpEngine {
    fn language(&self) -> Language {
        Language::CSharp
    }

    fn generate(&self, payload: Option<&Payload>) -> Result<String, DomainError> {
        let comments = payload
            .map(|p| p.format_as_comment("// "))
            .unwrap_or_default();

        let code = if comments.is_empty() {
            "class P{static void Main(){string s=\"class P{{static void Main(){{string s={0}{1}{0};System.Console.Write(s,'\\\"',s);}}}}\";System.Console.Write(s,'\"',s);}}".to_string()
        } else {
            let tmpl = format!(
                "{}class P{{{{static void Main(){{{{string s={{0}}{{1}}{{0}};System.Console.Write(s,'\\\"',s);}}}}}}}}",
                comments
            );
            format!(
                "{}class P{{static void Main(){{string s=\"{}\";System.Console.Write(s,'\"',s);}}}}\n",
                comments,
                tmpl.replace('\\', "\\\\").replace('"', "\\\"")
            )
        };

        Ok(code)
    }
}
