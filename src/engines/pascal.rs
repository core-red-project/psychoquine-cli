use crate::domain::error::DomainError;
use crate::domain::language::Language;
use crate::domain::payload::Payload;
use crate::engines::QuineEngine;

pub struct PascalEngine;

impl QuineEngine for PascalEngine {
    fn language(&self) -> Language {
        Language::Pascal
    }

    fn generate(&self, _payload: Option<&Payload>) -> Result<String, DomainError> {
        // Wikipedia canonical Pascal quine
        Ok("program q(output);const a='program q(output);const a=';b=';begin write(a,#39,a,#39,b,b) end.';begin write(a,#39,a,#39,b,b) end.\n".to_string())
    }
}
