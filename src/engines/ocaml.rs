use crate::domain::error::DomainError;
use crate::domain::language::Language;
use crate::domain::payload::Payload;
use crate::engines::QuineEngine;

pub struct OcamlEngine;

impl QuineEngine for OcamlEngine {
    fn language(&self) -> Language {
        Language::Ocaml
    }

    fn generate(&self, _payload: Option<&Payload>) -> Result<String, DomainError> {
        // Wikipedia OCaml Canonical Quine
        Ok(
            "(fun s -> Printf.printf \"%s %S\" s s) \"(fun s -> Printf.printf \\\"%s %S\\\" s s)\""
                .to_string(),
        )
    }
}
