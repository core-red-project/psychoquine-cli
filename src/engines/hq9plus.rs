use crate::domain::error::DomainError;
use crate::domain::language::Language;
use crate::domain::payload::Payload;
use crate::engines::QuineEngine;

pub struct Hq9PlusEngine;

impl QuineEngine for Hq9PlusEngine {
    fn language(&self) -> Language {
        Language::Hq9Plus
    }

    fn generate(&self, _payload: Option<&Payload>) -> Result<String, DomainError> {
        // In the HQ9+ programming language, the 'Q' instruction prints the program source code.
        Ok("Q\n".to_string())
    }
}
