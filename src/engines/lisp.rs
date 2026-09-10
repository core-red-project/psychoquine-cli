use crate::domain::error::DomainError;
use crate::domain::language::Language;
use crate::domain::payload::Payload;
use crate::engines::QuineEngine;

pub struct SchemeEngine;

impl QuineEngine for SchemeEngine {
    fn language(&self) -> Language {
        Language::Scheme
    }

    fn generate(&self, _payload: Option<&Payload>) -> Result<String, DomainError> {
        // Canonical Wikipedia Lisp / Scheme Quine
        Ok(
            "((lambda (x) (list x (list 'quote x))) '(lambda (x) (list x (list 'quote x))))\n"
                .to_string(),
        )
    }
}

pub struct CommonLispEngine;

impl QuineEngine for CommonLispEngine {
    fn language(&self) -> Language {
        Language::CommonLisp
    }

    fn generate(&self, _payload: Option<&Payload>) -> Result<String, DomainError> {
        // Canonical Common Lisp format quine
        Ok("((lambda (x) (list x (list (quote quote) x))) (quote (lambda (x) (list x (list (quote quote) x)))))\n".to_string())
    }
}
